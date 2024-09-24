import { sendTransaction } from "./utils";
import { utils, KeyPair } from "near-api-js";
import { writeFileSync } from "fs";

interface DropDataInput {
  name: string;
  image: string;
  scavenger_hunt?: Array<{
    description: string;
    key?: string; // We'll generate `key` during processing
    // We won't include `secretKey` here to avoid exposing it
  }>;
  key?: string; // We'll generate `key` during processing
}

export interface TokenDropInfo {
  drop_data: DropDataInput;
  token_amount: string;
}

export interface NFTDropInfo {
  drop_data: DropDataInput;
  nft_metadata: {
    title?: string;
    description?: string;
    media?: string;
  };
}

export interface MultichainDropInfo {
  drop_data: DropDataInput;
  multichain_metadata: {
    chain_id: number;
    contract_id: string;
    series_id: number;
  };
}

export type DropInfo = TokenDropInfo | NFTDropInfo | MultichainDropInfo;

export const createDrops = async ({
  signerAccount,
  factoryAccountId,
  drops,
}: {
  signerAccount: any;
  factoryAccountId: string;
  drops: Array<DropInfo>;
}) => {
  const dropIds: Array<string> = [];
  for (const drop of drops) {
    let res: any;
    let dropType: string;
    let dropSecretKey: string;

    // Generate a key pair for the drop
    const dropKeyPair = KeyPair.fromRandom("ed25519");
    const dropPublicKey = dropKeyPair.getPublicKey().toString();
    dropSecretKey = dropKeyPair.toString(); // Private key of the drop

    // Assign the public key to the drop data
    drop.drop_data.key = dropPublicKey;

    // Prepare an array to hold secret keys for scavenger pieces
    const scavengerSecretKeys: Array<{
      description: string;
      secretKey: string;
      publicKey: string;
    }> = [];

    // Handle scavenger hunt pieces if present
    if (drop.drop_data.scavenger_hunt) {
      for (const piece of drop.drop_data.scavenger_hunt) {
        // Generate a key pair for each scavenger piece
        const pieceKeyPair = KeyPair.fromRandom("ed25519");
        const piecePublicKey = pieceKeyPair.getPublicKey().toString();
        const pieceSecretKey = pieceKeyPair.toString(); // Private key of the piece

        // Assign the public key to the piece (to be sent to the contract)
        piece.key = piecePublicKey;

        // Store the secret key separately (not sent to the contract)
        scavengerSecretKeys.push({
          description: piece.description,
          secretKey: pieceSecretKey,
          publicKey: piecePublicKey,
        });
      }
    }

    // Prepare arguments for the contract method
    let args: any = {
      name: drop.drop_data.name,
      image: drop.drop_data.image,
      key: drop.drop_data.key,
      scavenger_hunt: drop.drop_data.scavenger_hunt?.map(
        ({ key, description }) => ({
          key,
          description,
        }),
      ),
    };

    // Determine the drop type and send the appropriate transaction
    if ((drop as TokenDropInfo).token_amount !== undefined) {
      args.token_amount = utils.format.parseNearAmount(
        (drop as TokenDropInfo).token_amount,
      );
      res = await sendTransaction({
        signerAccount,
        receiverId: factoryAccountId,
        methodName: "create_token_drop",
        args,
        deposit: "0",
        gas: "300000000000000",
      });
      dropType = "token";
    } else if ((drop as MultichainDropInfo).multichain_metadata !== undefined) {
      args.multichain_metadata = (
        drop as MultichainDropInfo
      ).multichain_metadata;
      res = await sendTransaction({
        signerAccount,
        receiverId: factoryAccountId,
        methodName: "create_multichain_drop",
        args,
        deposit: "0",
        gas: "300000000000000",
      });
      dropType = "multichain";
    } else {
      args.nft_metadata = (drop as NFTDropInfo).nft_metadata;
      res = await sendTransaction({
        signerAccount,
        receiverId: factoryAccountId,
        methodName: "create_nft_drop",
        args,
        deposit: "0",
        gas: "300000000000000",
      });
      dropType = "nft";
    }

    console.log("Response:", res);
    const status = res?.status;
    if (status && status.SuccessValue) {
      let dropId = Buffer.from(status.SuccessValue, "base64").toString("utf-8");
      if (dropId.startsWith('"') && dropId.endsWith('"')) {
        dropId = dropId.slice(1, -1);
      }

      // Handle scavenger hunt data if present
      if (drop.drop_data.scavenger_hunt) {
        let pieceNum = 1;
        for (const piece of scavengerSecretKeys) {
          // Write a CSV entry for each scavenger piece with the secret key
          dropIds.push(
            `"${drop.drop_data.name} - Piece ${pieceNum}",${dropType}%%${piece.secretKey}%%${dropId}`,
          );
          pieceNum++;
        }
      } else {
        // Handle regular token or NFT drop
        // Write the secret key of the drop
        dropIds.push(
          `"${drop.drop_data.name}",${dropType}%%${dropSecretKey}%%${dropId}`,
        );
      }
    } else {
      console.error("SuccessValue is not available");
    }
  }

  // Optionally write dropIds to a file
  writeFileSync("drops.csv", dropIds.join("\n"));

  return dropIds;
};
