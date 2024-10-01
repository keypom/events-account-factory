import { getAccountBalance, getContractStorageUsage } from "./helpers";
import { initNear, updateConfigFile } from "../utils";
import { Config } from "../types";
import {
  ADMIN_ACCOUNTS,
  CREATION_CONFIG,
  GLOBAL_NETWORK,
  NUM_TICKETS_TO_ADD,
  PREMADE_TICKET_DATA,
  SIGNER_ACCOUNT,
  TICKET_DATA,
  TICKET_URL_BASE,
  TIME_DELAY,
} from "./config";
import fs from "fs";
import path from "path";
import { generateSignature, getPublicKey } from "./cryptoHelpers";
import { KeyPair } from "near-api-js";
import { deployFactory } from "../createEvent";
import { scanTickets } from "./ticketActions";

import {
  createAccount,
  addTickets,
  addDrop,
  addScavengerHunt,
} from "./createActions";
import { claimDrop } from "./claimActions";
import { createConferenceAccounts } from "./accountActions";

async function main() {
  const config: Config = {
    GLOBAL_NETWORK,
    SIGNER_ACCOUNT,
    TICKET_URL_BASE,
    CLEANUP_CONTRACT: false,
    CREATION_CONFIG,
    NUM_TICKETS_TO_ADD,
    EXISTING_FACTORY: "", // Will be set after deploying the factory
    ADMIN_ACCOUNTS,
    PREMADE_TICKET_DATA,
  };

  const near = await initNear(config);
  console.log("Connected to Near");

  // Ensure the "data" directory exists
  const dataDir = path.join(__dirname, "data");
  if (!fs.existsSync(dataDir)) {
    fs.mkdirSync(dataDir);
  }

  // Deploy the factory contract
  const factoryAccountId = `${Date.now().toString()}-factory.${
    GLOBAL_NETWORK === "testnet" ? "testnet" : "near"
  }`;
  const factoryKey = await deployFactory({
    near,
    config,
    signerAccount: await near.account(SIGNER_ACCOUNT),
    adminAccounts: ADMIN_ACCOUNTS,
    factoryAccountId,
    ticketData: TICKET_DATA,
  });

  // Write the factory key to the "data" directory
  const csvFilePath = path.join(dataDir, "factoryKey.csv");
  fs.writeFileSync(csvFilePath, `${factoryAccountId},${factoryKey}`);

  // Update the factoryAccountId in config.ts
  updateConfigFile(factoryAccountId, "costMeasure");

  // Wait for the transaction to be processed
  console.log("Waiting for transaction to be processed...");
  await new Promise((resolve) => setTimeout(resolve, TIME_DELAY));

  // Initialize data structures
  const accounts: { [key: string]: string } = {};
  const drops: { [key: string]: { dropId: string; privateKey: string } } = {};

  // Initial signer is the admin account
  let signerAccount = await near.account(SIGNER_ACCOUNT);

  const results: {
    action: string;
    storage_used_bytes: number;
    balance_before: string;
    balance_after: string;
  }[] = [];

  // Helper to measure storage and balance
  async function measureAction(
    actionName: string,
    actionFn: () => Promise<any>,
  ) {
    const storageBefore = await getContractStorageUsage(
      signerAccount,
      factoryAccountId,
    );
    const balanceBefore = await getAccountBalance(
      signerAccount,
      factoryAccountId,
    );

    const result = await actionFn();
    // Wait for the transaction to be processed
    await new Promise((resolve) => setTimeout(resolve, TIME_DELAY));

    const storageAfter = await getContractStorageUsage(
      signerAccount,
      factoryAccountId,
    );
    const balanceAfter = await getAccountBalance(
      signerAccount,
      factoryAccountId,
    );

    const storageDiff = storageAfter - storageBefore;

    results.push({
      action: actionName,
      storage_used_bytes: storageDiff,
      balance_before: balanceBefore.toString(),
      balance_after: balanceAfter.toString(),
    });

    console.log(`Action: ${actionName}`);
    console.log(`Storage used (bytes): ${storageDiff}`);

    return result;
  }

  // Perform actions by calling the functions directly

  // Create Sponsor, Worker, and Admin accounts
  const accountTypes = ["Sponsor", "DataSetter", "Admin"];
  for (const type of accountTypes) {
    await measureAction(`Create ${type} account`, async () => {
      const data = await createAccount(
        signerAccount,
        factoryAccountId,
        type,
        type.toLowerCase(),
      );
      accounts[type.toLowerCase()] = data.secretKey;
      return data;
    });
  }

  // Add Tickets
  const ticketCounts = [1, 10];
  for (const count of ticketCounts) {
    await measureAction(`Add ${count} ticket(s)`, async () => {
      const attendees = Array(count).fill({
        name: "Test User",
        email: "test@example.com",
      });
      const data = await addTickets(signerAccount, factoryAccountId, attendees);
      // Map ticket keys to user accounts
      data.ticketKeys.forEach((key, index) => {
        const userKey = `user${index}`;
        accounts[userKey] = key;
      });
      return data;
    });
  }

  console.log("Accounts: ", accounts);

  // Scan Tickets
  await measureAction("Scan a ticket", async () => {
    await scanTickets(near, [accounts["user0"]], factoryAccountId);
  });

  const userKeys = Object.keys(accounts)
    .filter((key) => key.startsWith("user"))
    .map((key) => accounts[key]);

  await measureAction("Scan 10 tickets", async () => {
    await scanTickets(near, userKeys.slice(1, 11), factoryAccountId);
  });

  // Create Conference Accounts
  await measureAction("Create Conference Account", async () => {
    await createConferenceAccounts(
      near,
      [accounts["user0"]],
      [`user0.${factoryAccountId}`],
      factoryAccountId,
    );
  });

  await measureAction("Create 10 Conference Accounts", async () => {
    const userAccountIds = userKeys
      .slice(1, 11)
      .map((_, index) => `user${index + 1}.${factoryAccountId}`);
    await createConferenceAccounts(
      near,
      userKeys.slice(1, 11),
      userAccountIds,
      factoryAccountId,
    );
  });

  // Switch signer to Sponsor Account
  signerAccount = await near.account(factoryAccountId);
  const keyStore = near.connection.signer.keyStore;
  const sponsorKeyPair = KeyPair.fromString(accounts["sponsor"]);
  await keyStore.setKey(GLOBAL_NETWORK, factoryAccountId, sponsorKeyPair);

  // Add Drops
  const dropTypes = ["Token", "NFT", "Multichain"];
  for (const type of dropTypes) {
    await measureAction(`Add a ${type.toLowerCase()} drop`, async () => {
      const data = await addDrop(signerAccount, factoryAccountId, type);
      const dropId = data[0];
      const secretKey = dropId.split("%%")[1];
      drops[`${type.toLowerCase()}Drop`] = { dropId, privateKey: secretKey };
      return data;
    });
  }

  // Add Scavenger Hunts
  const huntPieceCounts = [2, 4, 10];
  for (const type of dropTypes) {
    for (const count of huntPieceCounts) {
      await measureAction(
        `Add scavenger ${type.toLowerCase()} hunt with ${count} pieces`,
        async () => {
          const data = await addScavengerHunt(
            signerAccount,
            factoryAccountId,
            type,
            count,
          );
          const dropId = data[0];
          const secretKey = dropId.split("%%")[2];
          drops[`scavenger${type}Hunt${count}`] = {
            dropId,
            privateKey: secretKey,
          };
          return data;
        },
      );
    }
  }

  console.log("Drops: ", drops);

  // Switch signer to Ticket User
  const ticketUserKey = accounts["user0"];
  const ticketUserId = `user0.${factoryAccountId}`;

  // Add ticket user's key to keystore
  const ticketUserKeyPair = KeyPair.fromString(ticketUserKey);
  await keyStore.setKey(GLOBAL_NETWORK, factoryAccountId, ticketUserKeyPair);
  signerAccount = await near.account(factoryAccountId);

  // Claim Drops
  for (const type of dropTypes) {
    await measureAction(`Claim a ${type.toLowerCase()} drop`, async () => {
      const drop = drops[`${type.toLowerCase()}Drop`];
      const signatureData = generateSignature(drop.privateKey, ticketUserId);
      const dropId = drop.dropId.split("%%")[2];
      await claimDrop(signerAccount, dropId, signatureData, factoryAccountId);
    });
  }

  // Claim Scavenger Hunt Piece
  await measureAction("Claim a scavenger hunt piece", async () => {
    const scavengerHunt = drops["scavengerTokenHunt2"];
    const scavengerPieceKey = scavengerHunt.privateKey;
    const scavengerPieceId = getPublicKey(scavengerPieceKey);
    const signatureData = generateSignature(scavengerPieceKey, ticketUserId);
    const dropId = scavengerHunt.dropId.split("%%")[3];
    await claimDrop(
      signerAccount,
      dropId,
      signatureData,
      factoryAccountId,
      scavengerPieceId,
    );
  });

  // Write results to CSV with better formatting for Google Sheets
  const csvData = results
    .map((res) => {
      // Ensure values are properly quoted if necessary
      const action = `"${res.action}"`;
      const storageUsed = `"${res.storage_used_bytes}"`;
      const balanceBefore = `"${res.balance_before}"`;
      const balanceAfter = `"${res.balance_after}"`;

      return `${action},${storageUsed},${balanceBefore},${balanceAfter}`;
    })
    .join("\n");

  // Add the header to the CSV file
  const csvHeader = `"Action","Storage Used (Bytes)","Balance Before","Balance After"`;
  const csvContent = `${csvHeader}\n${csvData}`;

  const resultsFilePath = path.join(__dirname, "storage_costs.csv");
  fs.writeFileSync(resultsFilePath, csvContent);

  console.log(`\nResults written to ${resultsFilePath}`);
}

main().catch(console.error);
