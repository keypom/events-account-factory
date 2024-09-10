import { sendTransaction } from "./utils";
import nacl from "tweetnacl";
import bs58 from "bs58"; // Library for decoding base58
import { KeyPair, utils } from "near-api-js";
import { encryptAndStoreData } from "./encryptionUtils";
import { GLOBAL_NETWORK, TICKET_URL_BASE } from "./config";

interface DropData {
  scavenger_hunt?: Array<{
    piece: string;
    description: string;
  }>;
  name: string;
}

export interface TokenDropInfo {
  drop_data: DropData;
  token_amount: string;
}

export interface NFTDropInfo {
  drop_data: DropData;
  nft_metadata: {
    title: string;
    description: string;
    media: string;
  };
}

export type DropInfo = TokenDropInfo | NFTDropInfo;

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
    let isNFT = false;

    if ((drop as TokenDropInfo).token_amount !== undefined) {
      res = await sendTransaction({
        signerAccount,
        receiverId: factoryAccountId,
        methodName: "create_token_drop",
        args: {
          drop_data: drop.drop_data,
          token_amount: utils.format.parseNearAmount(
            (drop as TokenDropInfo).token_amount,
          ),
        },
        deposit: "0",
        gas: "300000000000000",
      });
    } else {
      isNFT = true;
      res = await sendTransaction({
        signerAccount,
        receiverId: factoryAccountId,
        methodName: "create_nft_drop",
        args: {
          drop_data: drop.drop_data,
          nft_metadata: (drop as NFTDropInfo).nft_metadata,
        },
        deposit: "0",
        gas: "300000000000000",
      });
    }

    console.log("Response:", res);
    const status = res?.status;
    if (status && status.SuccessValue) {
      let dropId = atob(status.SuccessValue);
      if (dropId.startsWith('"') && dropId.endsWith('"')) {
        dropId = dropId.slice(1, -1);
      }

      // Format the CSV line, ensuring that text fields are wrapped in quotes
      dropIds.push(
        `"${drop.drop_data.name}",${isNFT ? "nft" : "token"}%%${dropId}`,
      );
    } else {
      console.error("SuccessValue is not available");
    }
  }

  return dropIds;
};
