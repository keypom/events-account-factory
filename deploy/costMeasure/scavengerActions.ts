import { Account } from "near-api-js";
import { createDrops } from "../createDrops";
import { EXISTING_FACTORY } from "./config";

// Add scavenger token hunt with 1 piece
export async function addScavengerTokenHunt1Piece(signerAccount: Account) {
  const drops = await createDrops({
    signerAccount,
    factoryAccountId: EXISTING_FACTORY,
    drops: [
      {
        drop_data: {
          name: "Scavenger Hunt - 1 Piece",
          image: "image-hash",
          scavenger_hunt: [
            { id: 1, description: "Find this token at location 1" },
          ],
        },
        token_amount: "10",
      },
    ],
  });
  return drops;
}

// Add scavenger token hunt with 4 pieces
export async function addScavengerTokenHunt4Pieces(signerAccount: Account) {
  const drops = await createDrops({
    signerAccount,
    factoryAccountId: EXISTING_FACTORY,
    drops: [
      {
        drop_data: {
          name: "Scavenger Hunt - 4 Pieces",
          image: "image-hash",
          scavenger_hunt: Array(4)
            .fill(0)
            .map((_, idx) => ({
              id: idx + 1,
              description: `Find this token at location ${idx + 1}`,
            })),
        },
        token_amount: "40",
      },
    ],
  });
  return drops;
}

// Add scavenger token hunt with 10 pieces
export async function addScavengerTokenHunt10Pieces(signerAccount: Account) {
  const drops = await createDrops({
    signerAccount,
    factoryAccountId: EXISTING_FACTORY,
    drops: [
      {
        drop_data: {
          name: "Scavenger Hunt - 10 Pieces",
          image: "image-hash",
          scavenger_hunt: Array(10)
            .fill(0)
            .map((_, idx) => ({
              id: idx + 1,
              description: `Find this token at location ${idx + 1}`,
            })),
        },
        token_amount: "100",
      },
    ],
  });
  return drops;
}

// Add scavenger NFT hunt with 1 piece
export async function addScavengerNFTHunt1Piece(signerAccount: Account) {
  const drops = await createDrops({
    signerAccount,
    factoryAccountId: EXISTING_FACTORY,
    drops: [
      {
        drop_data: {
          name: "Scavenger NFT Hunt - 1 Piece",
          image: "image-hash",
          scavenger_hunt: [
            { id: 1, description: "Find this NFT at location 1" },
          ],
        },
        nft_metadata: {
          title: "Scavenger NFT",
          description: "An NFT found at location 1",
          media: "image-hash",
        },
      },
    ],
  });
  return drops;
}

// Add scavenger NFT hunt with 4 pieces
export async function addScavengerNFTHunt4Pieces(signerAccount: Account) {
  const drops = await createDrops({
    signerAccount,
    factoryAccountId: EXISTING_FACTORY,
    drops: [
      {
        drop_data: {
          name: "Scavenger NFT Hunt - 4 Pieces",
          image: "image-hash",
          scavenger_hunt: Array(4)
            .fill(0)
            .map((_, idx) => ({
              id: idx + 1,
              description: `Find this NFT at location ${idx + 1}`,
            })),
        },
        nft_metadata: {
          title: "Scavenger NFT",
          description: "An NFT found at different locations",
          media: "image-hash",
        },
      },
    ],
  });
  return drops;
}

// Add scavenger NFT hunt with 10 pieces
export async function addScavengerNFTHunt10Pieces(signerAccount: Account) {
  const drops = await createDrops({
    signerAccount,
    factoryAccountId: EXISTING_FACTORY,
    drops: [
      {
        drop_data: {
          name: "Scavenger NFT Hunt - 10 Pieces",
          image: "image-hash",
          scavenger_hunt: Array(10)
            .fill(0)
            .map((_, idx) => ({
              id: idx + 1,
              description: `Find this NFT at location ${idx + 1}`,
            })),
        },
        nft_metadata: {
          title: "Scavenger NFT",
          description: "An NFT found in different locations",
          media: "image-hash",
        },
      },
    ],
  });
  return drops;
}

// Add scavenger multichain hunt with 1 piece
export async function addScavengerMultichainHunt1Piece(signerAccount: Account) {
  const drops = await createDrops({
    signerAccount,
    factoryAccountId: EXISTING_FACTORY,
    drops: [
      {
        drop_data: {
          name: "Scavenger Multichain Hunt - 1 Piece",
          image: "image-hash",
          scavenger_hunt: [
            { id: 1, description: "Find this multichain item at location 1" },
          ],
        },
        multichain_metadata: {
          chain_id: 84532,
          contract_id: "0xContractAddress",
          series_id: 1,
        },
        nft_metadata: {
          title: "Multichain NFT",
          description: "An NFT found at location 1",
          media: "image-hash",
        },
      },
    ],
  });
  return drops;
}

// Add scavenger multichain hunt with 4 pieces
export async function addScavengerMultichainHunt4Pieces(
  signerAccount: Account,
) {
  const drops = await createDrops({
    signerAccount,
    factoryAccountId: EXISTING_FACTORY,
    drops: [
      {
        drop_data: {
          name: "Scavenger Multichain Hunt - 4 Pieces",
          image: "image-hash",
          scavenger_hunt: Array(4)
            .fill(0)
            .map((_, idx) => ({
              id: idx + 1,
              description: `Find this multichain item at location ${idx + 1}`,
            })),
        },
        multichain_metadata: {
          chain_id: 84532,
          contract_id: "0xContractAddress",
          series_id: 1,
        },
        nft_metadata: {
          title: "Multichain NFT",
          description: "An NFT found in different locations",
          media: "image-hash",
        },
      },
    ],
  });
  return drops;
}

// Add scavenger multichain hunt with 10 pieces
export async function addScavengerMultichainHunt10Pieces(
  signerAccount: Account,
) {
  const drops = await createDrops({
    signerAccount,
    factoryAccountId: EXISTING_FACTORY,
    drops: [
      {
        drop_data: {
          name: "Scavenger Multichain Hunt - 10 Pieces",
          image: "image-hash",
          scavenger_hunt: Array(10)
            .fill(0)
            .map((_, idx) => ({
              id: idx + 1,
              description: `Find this multichain item at location ${idx + 1}`,
            })),
        },
        multichain_metadata: {
          chain_id: 84532,
          contract_id: "0xContractAddress",
          series_id: 1,
        },
        nft_metadata: {
          title: "Multichain NFT",
          description: "An NFT found in different locations",
          media: "image-hash",
        },
      },
    ],
  });
  return drops;
}
