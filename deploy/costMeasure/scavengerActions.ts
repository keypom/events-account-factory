import { Account } from "near-api-js";
import { createDrops } from "../createDrops";

// Add scavenger token hunt with 2 piece
export async function addScavengerTokenHunt2Piece(
  signerAccount: Account,
  factoryAccountId: string,
) {
  const drops = await createDrops({
    signerAccount,
    factoryAccountId: factoryAccountId,
    drops: [
      {
        drop_data: {
          name: "Official Scavenger Hunt",
          image: "bafkreihgosptxbojx37vxo4bly5opn5iqx2hmffdmg6ztokjmvtwa36axu",
          scavenger_hunt: [
            { id: 1, description: "Find this token at location 1" },
            { id: 2, description: "Find this token at location 1" },
          ],
        },
        token_amount: "1",
      },
    ],
  });
  return drops;
}

// Add scavenger token hunt with 4 pieces
export async function addScavengerTokenHunt4Pieces(
  signerAccount: Account,
  factoryAccountId: string,
) {
  const drops = await createDrops({
    signerAccount,
    factoryAccountId: factoryAccountId,
    drops: [
      {
        drop_data: {
          name: "Official Scavenger Hunt",
          image: "bafkreihgosptxbojx37vxo4bly5opn5iqx2hmffdmg6ztokjmvtwa36axu",
          scavenger_hunt: Array(4)
            .fill(0)
            .map((_, idx) => ({
              id: idx + 1,
              description: `Find this token at location ${idx + 1}`,
            })),
        },
        token_amount: "1",
      },
    ],
  });
  return drops;
}

// Add scavenger token hunt with 10 pieces
export async function addScavengerTokenHunt10Pieces(
  signerAccount: Account,
  factoryAccountId: string,
) {
  const drops = await createDrops({
    signerAccount,
    factoryAccountId: factoryAccountId,
    drops: [
      {
        drop_data: {
          name: "Official Scavenger Hunt",
          image: "bafkreihgosptxbojx37vxo4bly5opn5iqx2hmffdmg6ztokjmvtwa36axu",
          scavenger_hunt: Array(10)
            .fill(0)
            .map((_, idx) => ({
              id: idx + 1,
              description: `Find this token at location ${idx + 1}`,
            })),
        },
        token_amount: "1",
      },
    ],
  });
  return drops;
}

// Add scavenger NFT hunt with 2 piece
export async function addScavengerNFTHunt2Piece(
  signerAccount: Account,
  factoryAccountId: string,
) {
  const drops = await createDrops({
    signerAccount,
    factoryAccountId: factoryAccountId,
    drops: [
      {
        drop_data: {
          name: "Official Scavenger Hunt",
          image: "bafkreihgosptxbojx37vxo4bly5opn5iqx2hmffdmg6ztokjmvtwa36axu",
          scavenger_hunt: [
            { id: 1, description: "Find this NFT at location 1" },
            { id: 2, description: "Find this NFT at location 2" },
          ],
        },
        nft_metadata: {
          title: "Redacted Scavenger Hunt POAP",
          description:
            "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
          media: "bafkreihgosptxbojx37vxo4bly5opn5iqx2hmffdmg6ztokjmvtwa36axu",
        },
      },
    ],
  });
  return drops;
}

// Add scavenger NFT hunt with 4 pieces
export async function addScavengerNFTHunt4Pieces(
  signerAccount: Account,
  factoryAccountId: string,
) {
  const drops = await createDrops({
    signerAccount,
    factoryAccountId: factoryAccountId,
    drops: [
      {
        drop_data: {
          name: "Official Scavenger Hunt",
          image: "bafkreihgosptxbojx37vxo4bly5opn5iqx2hmffdmg6ztokjmvtwa36axu",
          scavenger_hunt: Array(4)
            .fill(0)
            .map((_, idx) => ({
              id: idx + 1,
              description: `Find this NFT at location ${idx + 1}`,
            })),
        },
        nft_metadata: {
          title: "Redacted Scavenger Hunt POAP",
          description:
            "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
          media: "bafkreihgosptxbojx37vxo4bly5opn5iqx2hmffdmg6ztokjmvtwa36axu",
        },
      },
    ],
  });
  return drops;
}

// Add scavenger NFT hunt with 10 pieces
export async function addScavengerNFTHunt10Pieces(
  signerAccount: Account,
  factoryAccountId: string,
) {
  const drops = await createDrops({
    signerAccount,
    factoryAccountId: factoryAccountId,
    drops: [
      {
        drop_data: {
          name: "Official Scavenger Hunt",
          image: "bafkreihgosptxbojx37vxo4bly5opn5iqx2hmffdmg6ztokjmvtwa36axu",
          scavenger_hunt: Array(10)
            .fill(0)
            .map((_, idx) => ({
              id: idx + 1,
              description: `Find this NFT at location ${idx + 1}`,
            })),
        },
        nft_metadata: {
          title: "Redacted Scavenger Hunt POAP",
          description:
            "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
          media: "bafkreihgosptxbojx37vxo4bly5opn5iqx2hmffdmg6ztokjmvtwa36axu",
        },
      },
    ],
  });
  return drops;
}

// Add scavenger multichain hunt with 2 piece
export async function addScavengerMultichainHunt2Piece(
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
          scavenger_hunt: [
            { id: 1, description: "Find this multichain item at location 1" },
            { id: 2, description: "Find this multichain item at location 2" },
          ],
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

// Add scavenger multichain hunt with 4 pieces
export async function addScavengerMultichainHunt4Pieces(
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
          scavenger_hunt: Array(4)
            .fill(0)
            .map((_, idx) => ({
              id: idx + 1,
              description: `Find this multichain item at location ${idx + 1}`,
            })),
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

// Add scavenger multichain hunt with 10 pieces
export async function addScavengerMultichainHunt10Pieces(
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
          scavenger_hunt: Array(10)
            .fill(0)
            .map((_, idx) => ({
              id: idx + 1,
              description: `Find this multichain item at location ${idx + 1}`,
            })),
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
