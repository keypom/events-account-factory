import { getAccountBalance, getContractStorageUsage } from "./helpers";
import { initNear, updateConfigFile } from "../utils";
import { Config } from "../types";
import {
  ADMIN_ACCOUNTS,
  CREATION_CONFIG,
  EXISTING_FACTORY,
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
import { utils, KeyPair } from "near-api-js";
import { deployFactory } from "../createEvent";

import {
  createSponsorAccount,
  createWorkerAccount,
  createAdminAccount,
  addOneTicket,
  addTenTickets,
  addTokenDrop,
  addNFTDrop,
  addMultichainDrop,
} from "./createActions";

import {
  addScavengerTokenHunt2Piece,
  addScavengerTokenHunt4Pieces,
  addScavengerTokenHunt10Pieces,
  addScavengerNFTHunt2Piece,
  addScavengerNFTHunt4Pieces,
  addScavengerNFTHunt10Pieces,
  addScavengerMultichainHunt2Piece,
  addScavengerMultichainHunt4Pieces,
  addScavengerMultichainHunt10Pieces,
} from "./scavengerActions";

import {
  claimNFTDrop,
  claimTokenDrop,
  claimMultichainDrop,
  claimScavengerHuntPiece,
} from "./claimActions";

import { scanTicket, scan10Tickets } from "./ticketActions";
import {
  createConferenceAccount,
  create10ConferenceAccounts,
} from "./accountActions";
import { getPubFromSecret } from "@keypom/core";

async function main() {
  const config: Config = {
    GLOBAL_NETWORK,
    SIGNER_ACCOUNT,
    TICKET_URL_BASE,
    CLEANUP_CONTRACT: false,
    CREATION_CONFIG,
    NUM_TICKETS_TO_ADD,
    EXISTING_FACTORY,
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

  // wait 4 seconds
  console.log("Waiting 4 seconds...");
  await new Promise((resolve) => setTimeout(resolve, TIME_DELAY));

  // Initialize data structures
  const accounts: {
    [key: string]: string;
  } = {};
  const drops: {
    [key: string]: {
      dropId: string;
      privateKey: string;
    };
  } = {};

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
    // wait 4 seconds for the transaction to be processed
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

  // Action 1: Create Sponsor Account
  const sponsorAccountData = await measureAction(
    "Create 1 sponsor account",
    async () => {
      const data = await createSponsorAccount(signerAccount, factoryAccountId);
      accounts["sponsor"] = data.secretKey;
      return data;
    },
  );

  // Action 2: Create Worker Account
  const workerAccountData = await measureAction(
    "Create 1 worker account",
    async () => {
      const data = await createWorkerAccount(signerAccount, factoryAccountId);
      accounts["worker"] = data.secretKey;
      return data;
    },
  );

  // Action 3: Create Admin Account
  const adminAccountData = await measureAction(
    "Create 1 admin account",
    async () => {
      const data = await createAdminAccount(signerAccount, factoryAccountId);
      accounts["admin"] = data.secretKey;
      return data;
    },
  );

  // Action 4: Add 1 Ticket
  const oneTicketData = await measureAction("Add 1 ticket", async () => {
    const data = await addOneTicket(signerAccount, factoryAccountId);

    // Since data.ticketKeys contains the private keys in base64, assign them to the appropriate user
    accounts["user0"] = data.ticketKeys[0]; // Map the first ticket to user0

    return data;
  });

  // Action 5: Add 10 Tickets
  const tenTicketsData = await measureAction("Add 10 tickets", async () => {
    const data = await addTenTickets(signerAccount, factoryAccountId);

    // Loop through ticketKeys and map them to user1 through user50
    for (let i = 0; i < data.ticketKeys.length; i++) {
      const userKey = `user${i + 1}`; // Dynamic user key (user1, user2, ...)
      accounts[userKey] = data.ticketKeys[i]; // Store the corresponding secret key
    }

    return data;
  });
  console.log("Accounts: ", accounts);

  // Action 6: Scan a ticket
  const scanTicketData = await measureAction("Scan a ticket", async () => {
    const data = await scanTicket(near, accounts["user0"], factoryAccountId);
    return data;
  });

  // Action 7: Scan 10 tickets
  const scanFiftyTicketsData = await measureAction(
    "Scan 10 tickets",
    async () => {
      const user1Through10 = Array.from(
        { length: 10 },
        (_, i) => `user${i + 1}`,
      );
      const users = user1Through10.map((user) => accounts[user]);
      const data = await scan10Tickets(near, users, factoryAccountId);
      return data;
    },
  );

  // Action 8: Create Conference Account
  const createAccountData = await measureAction(
    "Create Conference Account",
    async () => {
      const data = await createConferenceAccount(
        near,
        accounts["user0"],
        `user0.${factoryAccountId}`,
        factoryAccountId,
      );
      return data;
    },
  );

  // Action 9: Create 10 Conference Accounts
  const createTenAccountData = await measureAction(
    "Create 10 Conference Accounts",
    async () => {
      const user1Through10 = Array.from(
        { length: 10 },
        (_, i) => `user${i + 1}`,
      );
      const userKeys = user1Through10.map((user) => accounts[user]);
      const userAccountIds = user1Through10.map(
        (user) => `${user}.${factoryAccountId}`,
      );
      const data = await create10ConferenceAccounts(
        near,
        userKeys,
        userAccountIds,
        factoryAccountId,
      );
      return data;
    },
  );

  // Switch signer to Sponsor Account
  signerAccount = await near.account(factoryAccountId);
  const keyStore = near.connection.signer.keyStore;
  const sponsorKeyPair = KeyPair.fromString(accounts["sponsor"]);
  await keyStore.setKey(GLOBAL_NETWORK, factoryAccountId, sponsorKeyPair);

  // Action 10: Add a Token Drop
  const tokenDropData = await measureAction("Add a token drop", async () => {
    const data = await addTokenDrop(signerAccount, factoryAccountId);
    const dropId = data[0];
    const secretKey = dropId.split("%%")[1];
    drops["tokenDrop"] = { dropId, privateKey: secretKey };
    return data;
  });

  // Action 11: Add an NFT Drop
  const nftDropData = await measureAction("Add an NFT drop", async () => {
    const data = await addNFTDrop(signerAccount, factoryAccountId);
    const dropId = data[0];
    const secretKey = dropId.split("%%")[1];
    drops["nftDrop"] = { dropId, privateKey: secretKey };
    return data;
  });

  // Action 12: Add a Multichain Drop
  const multichainDropData = await measureAction(
    "Add a multichain drop",
    async () => {
      const data = await addMultichainDrop(signerAccount, factoryAccountId);
      const dropId = data[0];
      const secretKey = dropId.split("%%")[1];
      drops["multichainDrop"] = { dropId, privateKey: secretKey };
      return data;
    },
  );

  // Action 13: Add Scavenger Token Hunt with 2 Piece
  const scavengerTokenHunt2Data = await measureAction(
    "Add scavenger token hunt with 2 piece",
    async () => {
      const data = await addScavengerTokenHunt2Piece(
        signerAccount,
        factoryAccountId,
      );
      const dropId = data[0];
      const secretKey = dropId.split("%%")[2];
      drops["scavengerTokenHunt2"] = { dropId, privateKey: secretKey };
      return data;
    },
  );

  // Action 14: Add Scavenger Token Hunt with 4 Piece
  const scavengerTokenHunt4Data = await measureAction(
    "Add scavenger token hunt with 4 piece",
    async () => {
      const data = await addScavengerTokenHunt4Pieces(
        signerAccount,
        factoryAccountId,
      );
      const dropId = data[0];
      const secretKey = dropId.split("%%")[2];
      drops["scavengerTokenHunt4"] = { dropId, privateKey: secretKey };
      return data;
    },
  );
  // Action 15: Add Scavenger Token Hunt with 10 Pieces
  const scavengerTokenHunt10Data = await measureAction(
    "Add scavenger token hunt with 10 pieces",
    async () => {
      const data = await addScavengerTokenHunt10Pieces(
        signerAccount,
        factoryAccountId,
      );
      const dropId = data[0];
      const secretKey = dropId.split("%%")[2];
      drops["scavengerTokenHunt10"] = { dropId, privateKey: secretKey };
      return data;
    },
  );

  // Action 16: Add Scavenger NFT Hunt with 2 Pieces
  const scavengerNftHunt2Data = await measureAction(
    "Add scavenger nft hunt with 2 pieces",
    async () => {
      const data = await addScavengerNFTHunt2Piece(
        signerAccount,
        factoryAccountId,
      );
      const dropId = data[0];
      const secretKey = dropId.split("%%")[2];
      drops["scavengerNftHunt2"] = { dropId, privateKey: secretKey };
      return data;
    },
  );

  // Action 17: Add Scavenger NFT Hunt with 4 Pieces
  const scavengerNftHunt4Data = await measureAction(
    "Add scavenger nft hunt with 4 piece",
    async () => {
      const data = await addScavengerNFTHunt4Pieces(
        signerAccount,
        factoryAccountId,
      );
      const dropId = data[0];
      const secretKey = dropId.split("%%")[2];
      drops["scavengerNftHunt4"] = { dropId, privateKey: secretKey };
      return data;
    },
  );

  // Action 18: Add Scavenger NFT Hunt with 10 Pieces
  const scavengerNftHunt10Data = await measureAction(
    "Add scavenger nft hunt with 10 piece",
    async () => {
      const data = await addScavengerNFTHunt10Pieces(
        signerAccount,
        factoryAccountId,
      );
      const dropId = data[0];
      const secretKey = dropId.split("%%")[2];
      drops["scavengerNftHunt10"] = { dropId, privateKey: secretKey };
      return data;
    },
  );

  // Action 19: Add Scavenger Multichain Hunt with 2 Pieces
  const scavengerMultichainHunt1Data = await measureAction(
    "Add scavenger multichain hunt with 2 piece",
    async () => {
      const data = await addScavengerMultichainHunt2Piece(
        signerAccount,
        factoryAccountId,
      );
      const dropId = data[0];
      const secretKey = dropId.split("%%")[2];
      drops["scavengerMultichainHunt2"] = { dropId, privateKey: secretKey };
      return data;
    },
  );

  // Action 20: Add Scavenger Multichain Hunt with 4 Pieces
  const scavengerMultichainHunt4Data = await measureAction(
    "Add scavenger multichain hunt with 4 piece",
    async () => {
      const data = await addScavengerMultichainHunt4Pieces(
        signerAccount,
        factoryAccountId,
      );
      const dropId = data[0];
      const secretKey = dropId.split("%%")[2];
      drops["scavengerMultichainHunt4"] = { dropId, privateKey: secretKey };
      return data;
    },
  );

  // Action 21: Add Scavenger Multichain Hunt with 4 Pieces
  const scavengerMultichainHunt10Data = await measureAction(
    "Add scavenger multichain hunt with 10 piece",
    async () => {
      const data = await addScavengerMultichainHunt10Pieces(
        signerAccount,
        factoryAccountId,
      );
      const dropId = data[0];
      const secretKey = dropId.split("%%")[2];
      drops["scavengerMultichainHunt10"] = { dropId, privateKey: secretKey };
      return data;
    },
  );
  console.log("Drops: ", drops);

  // Switch signer to Ticket User
  // Assuming you have ticket user data from oneTicketData
  const ticketUserKey = accounts["user0"];
  const ticketUserId = `user0.${factoryAccountId}`;

  // Add ticket user's key to keystore
  const ticketUserKeyPair = KeyPair.fromString(ticketUserKey);
  await keyStore.setKey(GLOBAL_NETWORK, factoryAccountId, ticketUserKeyPair);
  signerAccount = await near.account(factoryAccountId);

  // Action: Claim an NFT Drop
  const nftSignatureData = generateSignature(
    drops["nftDrop"].privateKey,
    ticketUserId,
  );

  // Action 22: Claim an NFT Drop
  await measureAction("Claim an NFT drop", async () => {
    const dropId = drops["nftDrop"].dropId.split("%%")[2];
    await claimNFTDrop(
      signerAccount,
      dropId,
      nftSignatureData,
      factoryAccountId,
    );
  });

  // Action 23: Claim a Token Drop
  const tokenSignatureData = generateSignature(
    drops["tokenDrop"].privateKey,
    ticketUserId,
  );

  await measureAction("Claim a token drop", async () => {
    const dropId = drops["tokenDrop"].dropId.split("%%")[2];
    await claimTokenDrop(
      signerAccount,
      dropId,
      tokenSignatureData,
      factoryAccountId,
    );
  });

  // Action: Claim a Multichain Drop
  const multichainSignatureData = generateSignature(
    drops["multichainDrop"].privateKey,
    ticketUserId,
  );

  // Action 24: Claim a Multichain Drop
  await measureAction("Claim a multichain drop", async () => {
    const dropId = drops["multichainDrop"].dropId.split("%%")[2];
    await claimMultichainDrop(
      signerAccount,
      dropId,
      multichainSignatureData,
      factoryAccountId,
    );
  });

  const scavengerPieceKey = drops["scavengerTokenHunt2"].privateKey;
  const scavengerPieceId = getPublicKey(scavengerPieceKey);
  const scavengerSignatureData = generateSignature(
    scavengerPieceKey,
    ticketUserId,
  );

  // Action 25: Claim a Scavenger Hunt Piece
  await measureAction("Claim a scavenger hunt piece", async () => {
    const dropId = drops["scavengerTokenHunt2"].dropId.split("%%")[3];
    await claimScavengerHuntPiece(
      signerAccount,
      dropId,
      scavengerPieceId,
      scavengerSignatureData,
      factoryAccountId,
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
