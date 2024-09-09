import { sendTransaction } from "./utils";
import nacl from "tweetnacl";
import bs58 from "bs58"; // Library for decoding base58
import { KeyPair, utils } from "near-api-js";
import { encryptAndStoreData } from "./encryptionUtils";
import { GLOBAL_NETWORK, TICKET_URL_BASE } from "./config";

interface DropData {
  scavenger_hunt?: {
    piece: string;
    description: string;
  };
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

    if ((drop as TokenDropInfo).token_amount !== undefined) {
      // Send the transaction in batches of 50 tickets
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
        gas: "300000000000000", // Set gas limit
      });
    } else {
      res = await sendTransaction({
        signerAccount,
        receiverId: factoryAccountId,
        methodName: "create_nft_drop",
        args: {
          drop_data: drop.drop_data,
          nft_metadata: (drop as NFTDropInfo).nft_metadata,
        },
        deposit: "0",
        gas: "300000000000000", // Set gas limit
      });
    }

    console.log("Response:", res);
    const status = res?.status;
    console.log("Status:", status);
    if (status && status.SuccessValue) {
      console.log("SuccessValue:", status.SuccessValue);
      // Now we're sure SuccessValue exists and is a string
      let dropId = atob(status.SuccessValue);
      if (dropId.startsWith('"') && dropId.endsWith('"')) {
        dropId = dropId.slice(1, -1);
      }
      dropIds.push(dropId);
    } else {
      console.error("SuccessValue is not available");
    }
  }

  return dropIds;
};
