import { addPremadeTickets, addTickets } from "./addTickets";
import { adminCreateAccount } from "./adminCreateAccounts";
import {
  ADMIN_ACCOUNTS,
  CREATION_CONFIG,
  EXISTING_FACTORY,
  GLOBAL_NETWORK,
  NUM_TICKETS_TO_ADD,
  PREMADE_TICKET_DATA,
  SIGNER_ACCOUNT,
} from "./config";
import { deployFactory } from "./createEvent";
import { convertMapToRawJsonCsv, initNear, updateConfigFile } from "./utils";
import fs from "fs";
import path from "path";
import { createDrops } from "./createDrops";
import { TICKET_DATA } from "./configData/ticketData";
import { SPONSOR_DATA } from "./configData/sponsorData";
import { PREMADE_TOKEN_DROP_DATA } from "./configData/premadeTokenDrops";
import { PREMADE_NFT_DROP_DATA } from "./configData/premadeNFTDrops";
import { PREMADE_SCAVENGER_HUNTS } from "./configData/premadeScavengers";
import { PREMADE_MULTICHAIN_DROPS } from "./configData/premadeMultichainDrops";

const main = async () => {
  const near = await initNear();
  console.log("Connected to Near: ", near);

  const signerAccount = await near.account(SIGNER_ACCOUNT);

  // Ensure the "data" directory exists, create it if it doesn't
  const dataDir = path.join(__dirname, "data");
  if (!fs.existsSync(dataDir)) {
    fs.mkdirSync(dataDir);
  }

  // STEP 1: Deploy the factory contract
  let csvFilePath;
  let factoryAccountId = EXISTING_FACTORY;
  if (CREATION_CONFIG.deployContract) {
    factoryAccountId = `${Date.now().toString()}-factory.${GLOBAL_NETWORK === "testnet" ? "testnet" : "near"}`;
    let factoryKey = await deployFactory({
      near,
      signerAccount,
      adminAccounts: ADMIN_ACCOUNTS,
      factoryAccountId,
      ticketData: TICKET_DATA,
    });

    // Write the sponsors CSV to the "data" directory
    csvFilePath = path.join(dataDir, "factoryKey.csv");
    fs.writeFileSync(csvFilePath, `${factoryAccountId},${factoryKey}`);

    // STEP 1.1: Update the EXISTING_FACTORY in config.ts
    updateConfigFile(factoryAccountId);
  }

  // STEP 2: Create Sponsors
  if (CREATION_CONFIG.createSponsors) {
    const sponsorCSV: string[] = [];
    for (const sponsorData of SPONSOR_DATA) {
      const { accountId, secretKey } = await adminCreateAccount({
        signerAccount,
        factoryAccountId,
        newAccountName: sponsorData.accountName,
        startingNearBalance: sponsorData.startingNearBalance,
        startingTokenBalance: sponsorData.startingTokenBalance,
        accountType: sponsorData.accountType,
      });
      sponsorCSV.push(
        `${sponsorData.accountName}, http://localhost:3000/sponsorDashboard/${accountId}#${secretKey}`,
      );
    }

    // Write the sponsors CSV to the "data" directory
    csvFilePath = path.join(dataDir, "sponsors.csv");
    fs.writeFileSync(csvFilePath, sponsorCSV.join("\n"));
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

  // STEP 4: Add Tickets
  if (CREATION_CONFIG.addTickets) {
    const defaultAttendeeInfo = new Array(NUM_TICKETS_TO_ADD).fill({
      name: "Test User",
      email: "test",
    });
    const keyPairMap = await addTickets({
      signerAccount,
      factoryAccountId,
      dropId: "ga_pass",
      attendeeInfo: defaultAttendeeInfo,
    });
    // Convert the keyPairMap to CSV with raw JSON and write to a file
    const csvData = convertMapToRawJsonCsv(keyPairMap);
    csvFilePath = path.join(dataDir, "tickets.csv");
    fs.writeFileSync(csvFilePath, csvData);
  }

  if (CREATION_CONFIG.premadeTickets) {
    const premadeCSV = await addPremadeTickets({
      near,
      signerAccount,
      factoryAccountId,
      dropId: "ga_pass",
      attendeeInfo: PREMADE_TICKET_DATA,
    });
    // Write the sponsors CSV to the "data" directory
    csvFilePath = path.join(dataDir, "premade-tickets.csv");
    fs.writeFileSync(csvFilePath, premadeCSV.join("\n"));
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

  console.log("Done!");
  console.log(
    `https://${GLOBAL_NETWORK}.nearblocks.io/address/${factoryAccountId}`,
  );
};

main().catch(console.error);
