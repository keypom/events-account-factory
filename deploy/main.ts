import { addPremadeTickets, addTickets, encodeToBase64 } from "./addTickets";
import { adminCreateAccount } from "./adminCreateAccounts";
import fs from "fs";
import path from "path";
import { KeyPair } from "near-api-js";
import { createDrops } from "./createDrops";
import { Config, PremadeTicketData } from "./types";
import {
  convertMapToRawJsonCsv,
  initNear,
  setFactoryFullAccessKey,
  updateConfigFile,
} from "./utils";
import { deployFactory } from "./createEvent";
import { cleanupContract } from "./cleanup";
import { addAirtableKeys } from "./addAirtableKeys";

// Get the environment (dev or prod) from the command-line arguments
const env = process.argv[2] || "dev"; // Default to "dev" if no argument is provided

// Helper function to dynamically load the config
const loadConfig = async (env: string) => {
  console.log(`Loading config for ${env}`);
  const config: Config = await import(`./${env}/config`);
  const ticketData = await import(`./${env}/configData/ticketData`);
  const sponsorData = await import(`./${env}/configData/sponsorData`);
  const premadeTokenDrops = await import(
    `./${env}/configData/premadeTokenDrops`
  );
  const premadeNftDrops = await import(`./${env}/configData/premadeNFTDrops`);
  const premadeScavengers = await import(
    `./${env}/configData/premadeScavengers`
  );
  const premadeMultichainDrops = await import(
    `./${env}/configData/premadeMultichainDrops`
  );

  return {
    config,
    ticketData,
    sponsorData,
    premadeTokenDrops,
    premadeNftDrops,
    premadeScavengers,
    premadeMultichainDrops,
  };
};

const main = async () => {
  const {
    config,
    ticketData: { TICKET_DATA },
    sponsorData: { SPONSOR_DATA },
    premadeTokenDrops: { PREMADE_TOKEN_DROP_DATA },
    premadeNftDrops: { PREMADE_NFT_DROP_DATA },
    premadeScavengers: { PREMADE_SCAVENGER_HUNTS },
    premadeMultichainDrops: { PREMADE_MULTICHAIN_DROPS },
  } = await loadConfig(env);

  const {
    ADMIN_ACCOUNTS,
    CLEANUP_CONTRACT,
    FREEZE_CONTRACT,
    CREATION_CONFIG,
    EXISTING_FACTORY,
    GLOBAL_NETWORK,
    PREMADE_TICKET_DATA,
    SIGNER_ACCOUNT,
  } = config;

  const near = await initNear(config);
  console.log("Connected to Near: ", near);

  const signerAccount = await near.account(SIGNER_ACCOUNT);
  let factoryKey: string | undefined = undefined;

  // Ensure the "data" directory exists, create it if it doesn't
  const dataDir = path.join(__dirname, env, "data");
  if (!fs.existsSync(dataDir)) {
    fs.mkdirSync(dataDir, { recursive: true });
  }

  // STEP 1: Deploy the factory contract
  let csvFilePath;
  let factoryAccountId = EXISTING_FACTORY;
  if (CREATION_CONFIG.deployContract) {
    factoryAccountId =
      GLOBAL_NETWORK === "testnet"
        ? `${Date.now().toString()}-factory.testnet`
        : `redacted2024.near`;

    factoryKey = await deployFactory({
      near,
      config,
      signerAccount,
      adminAccounts: ADMIN_ACCOUNTS,
      factoryAccountId,
      ticketData: TICKET_DATA,
    });

    // Write the sponsors CSV to the "data" directory
    csvFilePath = path.join(dataDir, "factoryKey.csv");
    fs.writeFileSync(csvFilePath, `${factoryAccountId},${factoryKey}`);

    // STEP 1.1: Update the EXISTING_FACTORY in config.ts
    updateConfigFile(factoryAccountId, env);
  }

  // STEP 2: Create Sponsors
  if (CREATION_CONFIG.createSponsors) {
    const sponsorCSV: string[] = [];
    const sponsorTicketsCSV: string[] = [];

    for (const sponsorData of SPONSOR_DATA) {
      const { accountId, secretKey } = await adminCreateAccount({
        signerAccount,
        factoryAccountId,
        newAccountName: sponsorData.accountName,
        startingNearBalance: sponsorData.startingNearBalance,
        startingTokenBalance: sponsorData.startingTokenBalance,
        accountType: sponsorData.accountType,
      });

      // Write sponsor link
      const sponsorLink = `${sponsorData.accountName}, ${config.SITE_BASE_URL}/sponsorDashboard/${accountId}#${secretKey}`;
      sponsorCSV.push(sponsorLink);

      // Prepare attendee info for ticket creation
      const name =
        sponsorData.accountName.charAt(0).toUpperCase() +
        sponsorData.accountName.slice(1);
      const userData = { name, email: "" };
      // Instead of encrypting, create a base64-encoded JSON object
      const sponsorJsonObject = {
        ticket: secretKey, // Private key of the ticket
        userData,
      };

      const encodedJson = encodeToBase64(sponsorJsonObject); // Encode to base64
      // Map the keypair's public key to the corresponding attendee info
      const sponsorTicketLink = `${userData.name}, ${config.SITE_BASE_URL}/tickets/ticket/ga_pass#${encodedJson}`;
      sponsorTicketsCSV.push(sponsorTicketLink);
    }

    // Write the sponsor links CSV to the "data" directory
    csvFilePath = path.join(dataDir, "sponsors.csv");
    fs.writeFileSync(csvFilePath, sponsorCSV.join("\n"));

    // Write the sponsor tickets CSV to the "data" directory
    csvFilePath = path.join(dataDir, "sponsorTickets.csv");
    fs.writeFileSync(csvFilePath, sponsorTicketsCSV.join("\n"));
  }

  // STEP 3: Create Worker
  if (CREATION_CONFIG.createWorker) {
    const { keyPair } = await adminCreateAccount({
      signerAccount,
      factoryAccountId,
      newAccountName: "worker",
      startingNearBalance: "0.01",
      startingTokenBalance: "0",
      accountType: "DataSetter",
    });

    // Write the worker information to the "data" directory
    csvFilePath = path.join(dataDir, "worker.csv");
    fs.writeFileSync(csvFilePath, `worker, ${keyPair.toString()}`);
  }

  // STEP 4: Create Ticket Adder
  if (CREATION_CONFIG.createTicketAdder) {
    const { keyPair } = await adminCreateAccount({
      signerAccount,
      factoryAccountId,
      newAccountName: "ticket-worker",
      startingNearBalance: "0.01",
      startingTokenBalance: "0",
      accountType: "TicketAdder",
    });

    // Write the worker information to the "data" directory
    csvFilePath = path.join(dataDir, "ticket-adder.csv");
    fs.writeFileSync(csvFilePath, `ticket-adder, ${keyPair.toString()}`);
  }

  if (CREATION_CONFIG.createAdmin) {
    const { keyPair } = await adminCreateAccount({
      signerAccount,
      factoryAccountId,
      newAccountName: "admin5",
      startingNearBalance: "0.01",
      startingTokenBalance: "0",
      accountType: "Admin",
    });

    // Write the worker information to the "data" directory
    csvFilePath = path.join(dataDir, "admin.csv");
    fs.writeFileSync(csvFilePath, `admin, ${keyPair.toString()}`);
  }

  // STEP 5: Add Tickets
  if (CREATION_CONFIG.addTickets) {
    const mailingListDataDir = path.join(__dirname, env, "toRead");
    if (!fs.existsSync(mailingListDataDir)) {
      throw new Error(`Directory ${mailingListDataDir} does not exist.`);
    }
    const mailingListPath = path.join(
      mailingListDataDir,
      "mailinglist-current.json",
    );
    const mailingList = JSON.parse(fs.readFileSync(mailingListPath, "utf-8"));
    const { keyPairMap, failedTickets, updatedMailingList } =
      await addAirtableKeys({
        mailingList,
        signerAccount,
        factoryAccountId,
        dropId: "ga_pass",
      });

    const updatedMailingListPath = path.join(
      mailingListDataDir,
      "mailinglist-with-keys.json",
    );
    fs.writeFileSync(
      updatedMailingListPath,
      JSON.stringify(updatedMailingList, null, 2),
    );

    // STEP 6: Write failed tickets to a separate file if there are any
    if (failedTickets.length > 0) {
      const failedTicketsPath = path.join(
        mailingListDataDir,
        "failed-tickets.json",
      );
      fs.writeFileSync(
        failedTicketsPath,
        JSON.stringify(failedTickets, null, 2),
      );
      console.log(`Failed tickets written to ${failedTicketsPath}`);
      throw new Error("Failed to add tickets");
    }

    // Convert the keyPairMap to CSV with raw JSON and write to a file
    const csvData = convertMapToRawJsonCsv(keyPairMap, config);
    csvFilePath = path.join(dataDir, "tickets.csv");
    fs.writeFileSync(csvFilePath, csvData);
  }

  if (CREATION_CONFIG.premadeTickets) {
    const PREMADE_TICKET_DATA_FILLED: PremadeTicketData = Array.from(
      { length: 1800 },
      (_, index) => ({
        name: `Test User ${index + 1}`,
        email: index === 0 ? "" : "foo", // Empty email for the first user, "foo" for others
      }),
    );

    const { premadeCSV, premadeTicketCSV } = await addPremadeTickets({
      near,
      signerAccount,
      factoryAccountId,
      dropId: "ga_pass",
      attendeeInfo: PREMADE_TICKET_DATA_FILLED,
      config,
    });
    // Write the sponsors CSV to the "data" directory
    csvFilePath = path.join(dataDir, "premade-tickets.csv");
    fs.writeFileSync(csvFilePath, premadeCSV.join("\n"));

    csvFilePath = path.join(dataDir, "premade-ticket-keys.csv");
    fs.writeFileSync(csvFilePath, premadeTicketCSV.join("\n"));
  }

  if (CREATION_CONFIG.tokenDrops) {
    const premadeTokenDropCSV = await createDrops({
      signerAccount,
      factoryAccountId,
      drops: PREMADE_TOKEN_DROP_DATA,
    });
    csvFilePath = path.join(dataDir, "premade-token-drops.csv");
    fs.writeFileSync(csvFilePath, premadeTokenDropCSV.join("\n"));
  }

  if (CREATION_CONFIG.nftDrops) {
    const premadeNFTDropCSV = await createDrops({
      signerAccount,
      factoryAccountId,
      drops: PREMADE_NFT_DROP_DATA,
    });
    csvFilePath = path.join(dataDir, "premade-nft-drops.csv");
    fs.writeFileSync(csvFilePath, premadeNFTDropCSV.join("\n"));
  }

  if (CREATION_CONFIG.scavDrops) {
    const premadeScavDropCSV = await createDrops({
      signerAccount,
      factoryAccountId,
      drops: PREMADE_SCAVENGER_HUNTS,
    });
    csvFilePath = path.join(dataDir, "premade-scav-drops.csv");
    fs.writeFileSync(csvFilePath, premadeScavDropCSV.join("\n"));
  }

  if (CREATION_CONFIG.multichainDrops) {
    const premadeMultichainDropCSV = await createDrops({
      signerAccount,
      factoryAccountId,
      drops: PREMADE_MULTICHAIN_DROPS,
    });
    csvFilePath = path.join(dataDir, "premade-multichain-drops.csv");
    fs.writeFileSync(csvFilePath, premadeMultichainDropCSV.join("\n"));
  }

  // Reset the key in file system to be full access key
  if (factoryKey !== undefined) {
    await setFactoryFullAccessKey(factoryKey, factoryAccountId, config);
  }

  if (factoryKey !== undefined) {
    let factoryKeyPair = KeyPair.fromString(factoryKey);
    let keyStore = near.connection.signer.keyStore;
    await keyStore.setKey(GLOBAL_NETWORK, factoryAccountId, factoryKeyPair);
  }
  const factoryAccount = await near.account(factoryAccountId);
  // Freeze the contract
  console.log("Setting contract freeze to: ", FREEZE_CONTRACT);
  await factoryAccount.functionCall({
    contractId: factoryAccountId,
    methodName: "toggle_freeze",
    args: { is_freeze: FREEZE_CONTRACT },
    gas: "300000000000000",
  });

  if (CLEANUP_CONTRACT) {
    const summary = await cleanupContract({
      near,
      factoryKey,
      factoryAccountId,
      networkId: config.GLOBAL_NETWORK,
    });

    const summaryFilePath = path.join(dataDir, "cleanup_summary.json");
    fs.writeFileSync(summaryFilePath, JSON.stringify(summary, null, 2));

    console.log(`Cleanup complete. Summary written to ${summaryFilePath}`);
  }

  console.log("Done!");
  console.log(
    `https://${GLOBAL_NETWORK}.nearblocks.io/address/${factoryAccountId}`,
  );
};

main().catch(console.error);
