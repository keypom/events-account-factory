import { actions } from "./actions";
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
} from "./config";
import fs from "fs";
import path from "path";
import { generateSignature } from "./cryptoHelpers";
import { utils, KeyPair } from "near-api-js";
import { deployFactory } from "../createEvent";

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

  // STEP 1: Deploy the factory contract
  let factoryAccountId = EXISTING_FACTORY;
  let factoryKey: string | undefined;

  if (CREATION_CONFIG.deployContract) {
    factoryAccountId = `${Date.now().toString()}-factory.${
      GLOBAL_NETWORK === "testnet" ? "testnet" : "near"
    }`;
    factoryKey = await deployFactory({
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

    // Update the EXISTING_FACTORY in config.ts
    updateConfigFile(factoryAccountId, "costMeasure");
  }

  // Initialize data structures
  const accounts: {
    [key: string]: string;
  } = {};
  const drops: {
    [key: string]: {
      dropId: string;
      privateKey: string;
      scavengerKeys?: { id: string; privateKey: string }[];
    };
  } = {};

  // Initial signer is the admin account
  let signerAccount = await near.account(SIGNER_ACCOUNT);

  const results: {
    action: string;
    storage_used_bytes: number;
    cost_in_near: string;
  }[] = [];

  // Helper to measure storage and balance
  async function measureAction(
    actionName: string,
    actionFn: () => Promise<any>,
  ) {
    console.log(`\nPerforming action: ${actionName}`);

    const storageBefore = await getContractStorageUsage(
      signerAccount,
      factoryAccountId,
    );
    const balanceBefore = await getAccountBalance(
      signerAccount,
      factoryAccountId,
    );

    const result = await actionFn();

    const storageAfter = await getContractStorageUsage(
      signerAccount,
      factoryAccountId,
    );
    const balanceAfter = await getAccountBalance(
      signerAccount,
      factoryAccountId,
    );

    const storageDiff = storageAfter - storageBefore;
    const balanceDiff = balanceBefore - balanceAfter;

    results.push({
      action: actionName,
      storage_used_bytes: storageDiff,
      cost_in_near: utils.format.formatNearAmount(balanceDiff.toString(), 8),
    });

    console.log(`Action: ${actionName}`);
    console.log(`Storage used (bytes): ${storageDiff}`);
    console.log(
      `Cost (NEAR): ${utils.format.formatNearAmount(
        balanceDiff.toString(),
        8,
      )}`,
    );

    return result;
  }

  // Perform actions

  // Action 1: Create Sponsor Account
  const sponsorAccountData = await measureAction(
    "Create 1 sponsor account",
    async () => {
      const data = await actions[0].function(signerAccount);
      accounts["sponsor"] = data.secretKey;
      return data;
    },
  );

  // Action 2: Create Worker Account
  const workerAccountData = await measureAction(
    "Create 1 worker account",
    async () => {
      const data = await actions[1].function(signerAccount);
      accounts["worker"] = data.secretKey;
      return data;
    },
  );

  // Action 3: Create Admin Account
  const adminAccountData = await measureAction(
    "Create 1 admin account",
    async () => {
      const data = await actions[2].function(signerAccount);
      accounts["admin"] = data.secretKey;
      return data;
    },
  );

  // Action 4: Add 1 Ticket
  const oneTicketData = await measureAction("Add 1 ticket", async () => {
    const data = await actions[3].function(signerAccount);

    // Since data.ticketKeys contains the private keys in base64, assign them to the appropriate user
    accounts["user0"] = data.ticketKeys[0]; // Map the first ticket to user0

    return data;
  });

  // Action 5: Add 10 Tickets
  const tenTicketsData = await measureAction("Add 10 tickets", async () => {
    const data = await actions[4].function(signerAccount);

    // Loop through ticketKeys and map them to user1 through user50
    for (let i = 0; i < data.ticketKeys.length; i++) {
      const userKey = `user${i + 1}`; // Dynamic user key (user1, user2, ...)
      accounts[userKey] = data.ticketKeys[i]; // Store the corresponding secret key
    }

    return data;
  });

  // Action 6: Scan a ticket
  const scanTicketData = await measureAction("Scan a ticket", async () => {
    const data = await actions[5].function(near, accounts["user0"]);
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
      const data = await actions[6].function(near, users);
      return data;
    },
  );

  // Action 8: Create Conference Account
  const createAccountData = await measureAction(
    "Create Conference Account",
    async () => {
      const data = await actions[7].function(
        near,
        accounts["user1"],
        `user1.${EXISTING_FACTORY}`,
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
      const userAccountIds = user1Through10.map((user) => {
        return `user1.${EXISTING_FACTORY}.${user}`;
      });
      const data = await actions[8].function(near, userKeys, userAccountIds);
      return data;
    },
  );

  // Switch signer to Sponsor Account
  signerAccount = await near.account(EXISTING_FACTORY);
  const keyStore = near.connection.signer.keyStore;
  const sponsorKeyPair = KeyPair.fromString(accounts["sponsor"]);
  await keyStore.setKey(GLOBAL_NETWORK, EXISTING_FACTORY, sponsorKeyPair);

  // Action 10: Add a Token Drop
  const tokenDropData = await measureAction("Add a token drop", async () => {
    const data = await actions[9].function(signerAccount);
    const dropId = data[0];
    const secretKey = dropId.split("%%")[1];
    drops["tokenDrop"] = secretKey;
    return data;
  });

  // Action 11: Add an NFT Drop
  const nftDropData = await measureAction("Add an NFT drop", async () => {
    const data = await actions[10].function(signerAccount);
    const dropId = data[0];
    const secretKey = dropId.split("%%")[1];
    drops["nftDrop"] = secretKey;
    return data;
  });

  // Action 12: Add a Multichain Drop
  const multichainDropData = await measureAction(
    "Add a multichain drop",
    async () => {
      const data = await actions[11].function(signerAccount);
      const dropId = data[0];
      const secretKey = dropId.split("%%")[1];
      drops["multichainDrop"] = secretKey;
      return data;
    },
  );

  // Action 13: Add Scavenger Token Hunt with 1 Piece
  const scavengerTokenHunt1Data = await measureAction(
    "Add scavenger token hunt with 1 piece",
    async () => {
      const data = await actions[12].function(signerAccount);
      const dropId = data[0];
      const secretKey = dropId.split("%%")[2];
      drops["scavengerTokenHunt1"] = secretKey;
      return data;
    },
  );

  // Action 14: Add Scavenger Token Hunt with 4 Piece
  const scavengerTokenHunt4Data = await measureAction(
    "Add scavenger token hunt with 4 piece",
    async () => {
      const data = await actions[13].function(signerAccount);
      const dropId = data[0];
      const secretKey = dropId.split("%%")[2];
      drops["scavengerTokenHunt4"] = secretKey;
      return data;
    },
  );
  // Action 15: Add Scavenger Token Hunt with 10 Pieces
  const scavengerTokenHunt10Data = await measureAction(
    "Add scavenger token hunt with 10 pieces",
    async () => {
      const data = await actions[14].function(signerAccount);
      const dropId = data[0];
      const secretKey = dropId.split("%%")[2];
      drops["scavengerTokenHunt10"] = secretKey;
      return data;
    },
  );

  // Action 16: Add Scavenger NFT Hunt with 1 Pieces
  const scavengerNftHunt1Data = await measureAction(
    "Add scavenger nft hunt with 1 piece",
    async () => {
      const data = await actions[15].function(signerAccount);
      const dropId = data[0];
      const secretKey = dropId.split("%%")[2];
      drops["scavengerNftHunt1"] = secretKey;
      return data;
    },
  );

  // Action 17: Add Scavenger NFT Hunt with 4 Pieces
  const scavengerNftHunt4Data = await measureAction(
    "Add scavenger nft hunt with 4 piece",
    async () => {
      const data = await actions[16].function(signerAccount);
      const dropId = data[0];
      const secretKey = dropId.split("%%")[2];
      drops["scavengerNftHunt4"] = secretKey;
      return data;
    },
  );

  // Action 18: Add Scavenger NFT Hunt with 10 Pieces
  const scavengerNftHunt10Data = await measureAction(
    "Add scavenger nft hunt with 10 piece",
    async () => {
      const data = await actions[17].function(signerAccount);
      const dropId = data[0];
      const secretKey = dropId.split("%%")[2];
      drops["scavengerNftHunt10"] = secretKey;
      return data;
    },
  );

  // Action 19: Add Scavenger Multichain Hunt with 1 Pieces
  const scavengerMultichainHunt1Data = await measureAction(
    "Add scavenger multichain hunt with 1 piece",
    async () => {
      const data = await actions[18].function(signerAccount);
      const dropId = data[0];
      const secretKey = dropId.split("%%")[2];
      drops["scavengerMultichainHunt1"] = secretKey;
      return data;
    },
  );

  // Action 20: Add Scavenger Multichain Hunt with 4 Pieces
  const scavengerMultichainHunt4Data = await measureAction(
    "Add scavenger multichain hunt with 4 piece",
    async () => {
      const data = await actions[19].function(signerAccount);
      const dropId = data[0];
      const secretKey = dropId.split("%%")[2];
      drops["scavengerMultichainHunt4"] = secretKey;
      return data;
    },
  );

  // Action 21: Add Scavenger Multichain Hunt with 4 Pieces
  const scavengerMultichainHunt10Data = await measureAction(
    "Add scavenger multichain hunt with 10 piece",
    async () => {
      const data = await actions[20].function(signerAccount);
      const dropId = data[0];
      const secretKey = dropId.split("%%")[2];
      drops["scavengerMultichainHunt10"] = secretKey;
      return data;
    },
  );

  // Switch signer to Ticket User
  // Assuming you have ticket user data from oneTicketData
  const ticketUserKey = oneTicketData?.ticketKeys?.[0]?.secretKey;
  const ticketUserId = "ticket-user.testnet"; // Replace with actual user ID

  if (ticketUserKey && ticketUserId) {
    // Add ticket user's key to keystore
    const keyStore = near.connection.signer.keyStore;
    const ticketUserKeyPair = KeyPair.fromString(ticketUserKey);
    await keyStore.setKey(GLOBAL_NETWORK, ticketUserId, ticketUserKeyPair);

    signerAccount = await near.account(ticketUserId);

    // Action: Claim an NFT Drop
    const nftSignatureData = generateSignature(
      drops["nftDrop"].privateKey,
      ticketUserId,
    );

    await measureAction("Claim an NFT drop", async () => {
      await actions[21].function(
        signerAccount,
        drops["nftDrop"].dropId,
        nftSignatureData,
      );
    });

    // Action: Claim a Token Drop
    const tokenSignatureData = generateSignature(
      drops["tokenDrop"].privateKey,
      ticketUserId,
    );

    await measureAction("Claim a token drop", async () => {
      await actions[22].function(
        signerAccount,
        drops["tokenDrop"].dropId,
        tokenSignatureData,
      );
    });

    // Action: Claim a Multichain Drop
    const multichainSignatureData = generateSignature(
      drops["multichain"].privateKey,
      ticketUserId,
    );

    await measureAction("Claim a multichain drop", async () => {
      await actions[23].function(
        signerAccount,
        drops["multichain"].dropId,
        multichainSignatureData,
      );
    });

    // Action: Claim a Scavenger Hunt Piece
    if (drops["scavengerTokenHunt1"]?.scavengerKeys?.[0]) {
      const scavengerPieceKey =
        drops["scavengerTokenHunt1"].scavengerKeys[0].privateKey;
      const scavengerPieceId = drops["scavengerTokenHunt1"].scavengerKeys[0].id;
      const scavengerSignatureData = generateSignature(
        scavengerPieceKey,
        ticketUserId,
      );

      await measureAction("Claim a scavenger hunt piece", async () => {
        await actions[24].function(
          signerAccount,
          drops["scavengerTokenHunt1"].dropId,
          scavengerPieceId,
          scavengerSignatureData,
        );
      });
    }
  }

  // Write results to CSV
  const csvData = results
    .map((res) => `${res.action},${res.storage_used_bytes},${res.cost_in_near}`)
    .join("\n");

  const csvHeader = "Action,Storage Used (Bytes),Cost (NEAR)";
  const csvContent = `${csvHeader}\n${csvData}`;

  const resultsFilePath = path.join(__dirname, "storage_costs.csv");
  fs.writeFileSync(resultsFilePath, csvContent);

  console.log(`\nResults written to ${resultsFilePath}`);
}

main().catch(console.error);
