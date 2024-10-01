import { KeyPair } from "near-api-js";
import { sendTransaction } from "../utils";
import { GLOBAL_NETWORK } from "./config";

export async function createConferenceAccount(
  near: any,
  secretKey: string,
  accountId: string,
  factoryAccountId: string
) {
  // Switch signer to User Account
  const signerAccount = await near.account(factoryAccountId);
  const keyStore = near.connection.signer.keyStore;
  const keyPair = KeyPair.fromString(secretKey);
  await keyStore.setKey(GLOBAL_NETWORK, factoryAccountId, keyPair);

  await sendTransaction({
    signerAccount,
    receiverId: factoryAccountId,
    methodName: "create_account",
    args: {
      new_account_id: accountId,
    },
    deposit: "0",
    gas: "300000000000000",
  });
}

export async function create10ConferenceAccounts(
  near: any,
  secretKeys: string[],
  accountIds: string[],
  factoryAccountId: string
) {
  for (let i = 0; i < 10; i++) {
    await createConferenceAccount(near, secretKeys[i], accountIds[i], factoryAccountId);
  }
}
