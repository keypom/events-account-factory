import { BN } from "bn.js";
import { createHash } from "crypto";
import { KeyPair, NearAccount } from "near-workspaces";
import { JsonKeyInfo } from "./types";

export const DEFAULT_GAS: string = "30000000000000";
export const LARGE_GAS: string = "300000000000000";
export const WALLET_GAS: string = "100000000000000";
export const DEFAULT_DEPOSIT: string = "1000000000000000000000000";
export const GAS_PRICE = new BN("100000000");
export const DEFAULT_TERRA_IN_NEAR: string = "3000000000000000000000";
export const CONTRACT_METADATA = {
  version: "1.0.0",
  link: "https://github.com/mattlockyer/proxy/commit/71a943ea8b7f5a3b7d9e9ac2208940f074f8afba",
};

export function hash(string: string, double = false) {
  if (double) {
    return createHash("sha256")
      .update(Buffer.from(string, "hex"))
      .digest("hex");
  }

  return createHash("sha256").update(Buffer.from(string)).digest("hex");
}

export function generatePasswordsForKey(
  pubKey: string,
  usesWithPassword: number[],
  basePassword: string
) {
  let passwords: Record<number, string> = {};

  // Loop through usesWithPassword
  for (var use of usesWithPassword) {
    passwords[use] = hash(hash(basePassword + pubKey + use.toString()), true);
  }

  return passwords;
}

export async function getKeyInformation(
  keypom: NearAccount,
  publicKey: string
): Promise<JsonKeyInfo> {
  const keyInformation: JsonKeyInfo = await keypom.view("get_key_information", {
    key: publicKey,
  });
  return keyInformation;
}

export async function generateKeyPairs(
  numKeys: number
): Promise<{ keys: KeyPair[]; publicKeys: string[] }> {
  // Generate NumKeys public keys
  let kps: KeyPair[] = [];
  let pks: string[] = [];
  for (let i = 0; i < numKeys; i++) {
    let keyPair = await KeyPair.fromRandom("ed25519");
    kps.push(keyPair);
    pks.push(keyPair.getPublicKey().toString());
  }
  return {
    keys: kps,
    publicKeys: pks,
  };
}
