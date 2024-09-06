import { addTickets } from "./addTickets";
import { adminCreateAccount } from "./adminCreateAccounts";
import {
  ADMIN_ACCOUNTS,
  EXISTING_FACTORY,
  NEW_FACTORY,
  SHOULD_CREATE_SPONSORS,
  SIGNER_ACCOUNT,
  SPONSOR_DATA,
  TICKET_DATA,
} from "./config";
import { deployFactory } from "./createEvent";
import { convertMapToRawJsonCsv, initNear, updateConfigFile } from "./utils";
import fs from "fs";
import path from "path";
import {
  decodeEd25519SecretKey,
  decryptOnChainData,
  decryptStoredData,
} from "./encryptionUtils";

const main = async () => {
  const near = await initNear();

  const signerAccount = await near.account(SIGNER_ACCOUNT);

  // Ensure the "data" directory exists, create it if it doesn't
  const dataDir = path.join(__dirname, "data");
  if (!fs.existsSync(dataDir)) {
    fs.mkdirSync(dataDir);
  }

  // STEP 1: Deploy the factory contract
  let factoryAccountId = EXISTING_FACTORY;
  if (NEW_FACTORY) {
    factoryAccountId = `${Date.now().toString()}-factory.testnet`;
    await deployFactory({
      near,
      signerAccount,
      adminAccounts: ADMIN_ACCOUNTS,
      factoryAccountId,
      ticketData: TICKET_DATA,
    });

    // STEP 1.1: Update the EXISTING_FACTORY in config.ts
    updateConfigFile(factoryAccountId);
  }

  let csvFilePath;
  // STEP 2: Create Sponsors
  if (SHOULD_CREATE_SPONSORS) {
    const sponsorCSV: string[] = [];
    for (const sponsorData of SPONSOR_DATA) {
      const { connectionObject } = await adminCreateAccount({
        signerAccount,
        factoryAccountId,
        newAccountName: sponsorData.accountName,
        startingNearBalance: sponsorData.startingNearBalance,
        startingTokenBalance: sponsorData.startingTokenBalance,
        accountType: sponsorData.accountType,
      });
      sponsorCSV.push(
        `${sponsorData.accountName}, http://localhost:3000/dashboard?connection=${btoa(connectionObject)}`,
      );
    }

    // Write the sponsors CSV to the "data" directory
    csvFilePath = path.join(dataDir, "sponsors.csv");
    fs.writeFileSync(csvFilePath, sponsorCSV.join("\n"));
  }

  // STEP 3: Create Worker
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

  // STEP 4: Add Tickets
  // TODO: Add airtable integration
  const attendeeInfo = [
    {
      name: "test",
      email: "test",
    },
  ];

  const keyPairMap = await addTickets({
    signerAccount,
    factoryAccountId,
    dropId: "ga_pass",
    attendeeInfo,
  });
  // Convert the keyPairMap to CSV with raw JSON and write to a file
  const csvData = convertMapToRawJsonCsv(keyPairMap);
  csvFilePath = path.join(dataDir, "tickets.csv");
  fs.writeFileSync(csvFilePath, csvData);

  console.log("Done!");
  console.log(`https://testnet.nearblocks.io/address/${factoryAccountId}`);
};

main().catch(console.error);
