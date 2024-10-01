import { Account } from "near-api-js";
import { sendTransaction } from "../utils";

export async function claimNFTDrop(
  signerAccount: Account,
  dropId: string,
  signatureData: { signature: string; publicKey: string },
  factoryAccountId: string,
) {
  await sendTransaction({
    signerAccount,
    receiverId: factoryAccountId,
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
  factoryAccountId: string,
) {
  await sendTransaction({
    signerAccount,
    receiverId: factoryAccountId,
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
  factoryAccountId: string,
) {
  await sendTransaction({
    signerAccount,
    receiverId: factoryAccountId,
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
  factoryAccountId: string,
) {
  await sendTransaction({
    signerAccount,
    receiverId: factoryAccountId,
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
