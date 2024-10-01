import { Account, KeyPair } from "near-api-js";
import { sendTransaction } from "../utils";
import { EXISTING_FACTORY } from "./config";

export async function claimNFTDrop(
  signerAccount: Account,
  dropId: string,
  signatureData: { signature: string; publicKey: string },
) {
  await sendTransaction({
    signerAccount,
    receiverId: EXISTING_FACTORY,
    methodName: "claim_drop",
    args: {
      drop_id: dropId,
      scavenger_id: null,
      signature: signatureData.signature,
    },
    deposit: "0",
    gas: "300000000000000",
  });
}

export async function claimTokenDrop(
  signerAccount: Account,
  dropId: string,
  signatureData: { signature: string; publicKey: string },
) {
  await sendTransaction({
    signerAccount,
    receiverId: EXISTING_FACTORY,
    methodName: "claim_drop",
    args: {
      drop_id: dropId,
      scavenger_id: null,
      signature: signatureData.signature,
    },
    deposit: "0",
    gas: "300000000000000",
  });
}

export async function claimMultichainDrop(
  signerAccount: Account,
  dropId: string,
  signatureData: { signature: string; publicKey: string },
) {
  await sendTransaction({
    signerAccount,
    receiverId: EXISTING_FACTORY,
    methodName: "claim_drop",
    args: {
      drop_id: dropId,
      scavenger_id: null,
      signature: signatureData.signature,
    },
    deposit: "0",
    gas: "300000000000000",
  });
}

export async function claimScavengerHuntPiece(
  signerAccount: Account,
  dropId: string,
  scavengerId: string,
  signatureData: { signature: string; publicKey: string },
) {
  await sendTransaction({
    signerAccount,
    receiverId: EXISTING_FACTORY,
    methodName: "claim_drop",
    args: {
      drop_id: dropId,
      scavenger_id: scavengerId,
      signature: signatureData.signature,
    },
    deposit: "0",
    gas: "300000000000000",
  });
}
