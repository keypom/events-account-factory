import { Account } from "near-api-js";
import { adminCreateAccount } from "../adminCreateAccounts";
import { addTickets } from "../addTickets";
import { createDrops } from "../createDrops";

// Create sponsor account
export async function createSponsorAccount(
  signerAccount: Account,
  factoryAccountId: string,
): Promise<{ accountId: string; secretKey: string }> {
  const result = await adminCreateAccount({
    signerAccount,
    factoryAccountId: factoryAccountId,
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
  factoryAccountId: string,
): Promise<{ accountId: string; secretKey: string }> {
  const result = await adminCreateAccount({
    signerAccount,
    factoryAccountId: factoryAccountId,
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
  factoryAccountId: string,
): Promise<{ accountId: string; secretKey: string }> {
  const result = await adminCreateAccount({
    signerAccount,
    factoryAccountId: factoryAccountId,
    newAccountName: "admin",
    startingNearBalance: "0.01",
    startingTokenBalance: "0",
    accountType: "Admin",
  });

  return { accountId: result.accountId, secretKey: result.secretKey };
}

// Add one ticket
export async function addOneTicket(
  signerAccount: Account,
  factoryAccountId: string,
) {
  const result = await addTickets({
    signerAccount,
    factoryAccountId: factoryAccountId,
    dropId: "ga_pass",
    attendeeInfo: [{ name: "Test User", email: "test@example.com" }],
    encodeTickets: false,
  });
  // Return ticket keys if needed
  return { ticketKeys: Array.from(result.keys()) };
}

// Add fifty tickets
export async function addTenTickets(
  signerAccount: Account,
  factoryAccountId: string,
) {
  const attendees = Array(10).fill({
    name: "Test User",
    email: "test@example.com",
  });
  const result = await addTickets({
    signerAccount,
    factoryAccountId: factoryAccountId,
    dropId: "ga_pass",
    attendeeInfo: attendees,
    encodeTickets: false,
  });
  // Return ticket keys
  return { ticketKeys: Array.from(result.keys()) };
}

// Add token drop
export async function addTokenDrop(
  signerAccount: Account,
  factoryAccountId: string,
) {
  const drops = await createDrops({
    signerAccount,
    factoryAccountId: factoryAccountId,
    drops: [
      {
        drop_data: {
          name: "Illia's Talk POAP",
          image: "bafkreihgosptxbojx37vxo4bly5opn5iqx2hmffdmg6ztokjmvtwa36axu",
        },
        token_amount: "1",
      },
    ],
  });
  return drops;
}

// Add NFT drop
export async function addNFTDrop(
  signerAccount: Account,
  factoryAccountId: string,
) {
  const drops = await createDrops({
    signerAccount,
    factoryAccountId: factoryAccountId,
    drops: [
      {
        drop_data: {
          name: "Illia's Talk POAP",
          image: "bafkreihgosptxbojx37vxo4bly5opn5iqx2hmffdmg6ztokjmvtwa36axu",
        },
        nft_metadata: {
          title: "Illia Talk POAP",
          description:
            "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
          media: "bafkreihgosptxbojx37vxo4bly5opn5iqx2hmffdmg6ztokjmvtwa36axu",
        },
      },
    ],
  });
  return drops;
}

// Add multichain drop
export async function addMultichainDrop(
  signerAccount: Account,
  factoryAccountId: string,
) {
  const drops = await createDrops({
    signerAccount,
    factoryAccountId: factoryAccountId,
    drops: [
      {
        drop_data: {
          image: "bafkreihgosptxbojx37vxo4bly5opn5iqx2hmffdmg6ztokjmvtwa36axu",
          name: "Surfer Dog POAP",
        },
        nft_metadata: {
          title: "Multichain Test Drop",
          description:
            "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
          media: "bafkreihgosptxbojx37vxo4bly5opn5iqx2hmffdmg6ztokjmvtwa36axu",
        },
        multichain_metadata: {
          chain_id: 84532,
          contract_id: "0xD6B95F11213cC071B982D717721B1aC7Bc628d46",
          series_id: 1,
        },
      },
    ],
  });
  return drops;
}
