import { Account } from "near-api-js";
import { sendTransaction } from "../utils";
import { EXISTING_FACTORY, GLOBAL_NETWORK } from "./config";
import { KeyPair } from "near-api-js";

// Scan a ticket
export async function scanTicket(near: any, ticketKey: string) {
  // Switch signer to User Account
  const signerAccount = await near.account(EXISTING_FACTORY);
  const keyStore = near.connection.signer.keyStore;
  const sponsorKeyPair = KeyPair.fromString(ticketKey);
  await keyStore.setKey(GLOBAL_NETWORK, EXISTING_FACTORY, sponsorKeyPair);

  await sendTransaction({
    signerAccount,
    receiverId: EXISTING_FACTORY,
    methodName: "scan_ticket",
    args: {},
    deposit: "0",
    gas: "300000000000000",
  });
}

// Scan 10 tickets
export async function scan10Tickets(near: any, ticketKeys: string[]) {
  for (let i = 0; i < 10; i++) {
    await scanTicket(near, ticketKeys[i]);
  }
}
