import { sendTransaction } from "../utils";
import { KeyPair } from "near-api-js";
import { GLOBAL_NETWORK } from "./config";

// Scan a ticket
export async function scanTicket(
  near: any,
  ticketKey: string,
  factoryAccountId: string,
) {
  // Switch signer to User Account
  const signerAccount = await near.account(factoryAccountId);
  const keyStore = near.connection.signer.keyStore;
  const sponsorKeyPair = KeyPair.fromString(ticketKey);
  await keyStore.setKey(GLOBAL_NETWORK, factoryAccountId, sponsorKeyPair);

  await sendTransaction({
    signerAccount,
    receiverId: factoryAccountId,
    methodName: "scan_ticket",
    args: {},
    deposit: "0",
    gas: "300000000000000",
  });
}

// Scan 10 tickets
export async function scan10Tickets(
  near: any,
  ticketKeys: string[],
  factoryAccountId: string,
) {
  for (let i = 0; i < 10; i++) {
    await scanTicket(near, ticketKeys[i], factoryAccountId);
  }
}
