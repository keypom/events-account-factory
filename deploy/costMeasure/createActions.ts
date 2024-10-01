import { Account, KeyPair } from "near-api-js";
import { adminCreateAccount } from "../adminCreateAccounts";
import { addTickets } from "../addTickets";
import { createDrops } from "../createDrops";
import { EXISTING_FACTORY } from "./config";
import { generateSignature } from "./cryptoHelpers";

// Create sponsor account
export async function createSponsorAccount(
  signerAccount: Account,
): Promise<{ accountId: string; secretKey: string }> {
  const result = await adminCreateAccount({
    signerAccount,
    factoryAccountId: EXISTING_FACTORY,
    newAccountName: "sponsor",
    startingNearBalance: "0.01",
    startingTokenBalance: "50",
    accountType: "Sponsor",
  });
  return { accountId: result.accountId, secretKey: result.secretKey };
}

// Create worker account
export async function createWorkerAccount(
  signerAccount: Account,
): Promise<{ accountId: string; secretKey: string }> {
  const result = await adminCreateAccount({
    signerAccount,
    factoryAccountId: EXISTING_FACTORY,
    newAccountName: "worker",
    startingNearBalance: "0.01",
    startingTokenBalance: "0",
    accountType: "DataSetter",
  });
  return { accountId: result.accountId, secretKey: result.secretKey };
}

// Create admin account
export async function createAdminAccount(
  signerAccount: Account,
): Promise<{ accountId: string; secretKey: string }> {
  const result = await adminCreateAccount({
    signerAccount,
    factoryAccountId: EXISTING_FACTORY,
    newAccountName: "admin",
    startingNearBalance: "0.01",
    startingTokenBalance: "0",
    accountType: "Admin",
  });

  return { accountId: result.accountId, secretKey: result.secretKey };
}

// Add one ticket
export async function addOneTicket(signerAccount: Account) {
  const result = await addTickets({
    signerAccount,
    factoryAccountId: EXISTING_FACTORY,
    dropId: "ga_pass",
    attendeeInfo: [{ name: "Test User", email: "test@example.com" }],
  });
  // Return ticket keys if needed
  return { ticketKeys: Array.from(result.keys()) };
}

// Add fifty tickets
export async function addTenTickets(signerAccount: Account) {
  const attendees = Array(10).fill({
    name: "Test User",
    email: "test@example.com",
  });
  const result = await addTickets({
    signerAccount,
    factoryAccountId: EXISTING_FACTORY,
    dropId: "ga_pass",
    attendeeInfo: attendees,
  });
  // Return ticket keys
  return { ticketKeys: Array.from(result.keys()) };
}

// Add token drop
export async function addTokenDrop(signerAccount: Account) {
  const drops = await createDrops({
    signerAccount,
    factoryAccountId: EXISTING_FACTORY,
    drops: [
      {
        drop_data: { name: "Test Token Drop", image: "image-hash" },
        token_amount: "100",
      },
    ],
  });
  return drops;
}

// Add NFT drop
export async function addNFTDrop(signerAccount: Account) {
  const drops = await createDrops({
    signerAccount,
    factoryAccountId: EXISTING_FACTORY,
    drops: [
      {
        drop_data: { name: "Test NFT Drop", image: "image-hash" },
        nft_metadata: {
          title: "Test NFT",
          description: "Test NFT Description",
          media: "image-hash",
        },
      },
    ],
  });
  return drops;
}

// Add multichain drop
export async function addMultichainDrop(signerAccount: Account) {
  const drops = await createDrops({
    signerAccount,
    factoryAccountId: EXISTING_FACTORY,
    drops: [
      {
        drop_data: { name: "Test Multichain Drop", image: "image-hash" },
        multichain_metadata: {
          chain_id: 84532,
          contract_id: "0xD6B95F11213cC071B982D717721B1aC7Bc628d46",
          series_id: 1,
        },
        nft_metadata: {
          title: "Test Multichain NFT",
          description: "Test Multichain NFT Description",
          media: "image-hash",
        },
      },
    ],
  });
  return drops;
}
