import { BN } from "bn.js";
import { createHash } from "crypto";
import { KeyPair, NEAR, NearAccount, PublicKey } from "near-workspaces";
import { ExtKeyInfo, TrialRules, UserProvidedFCArgs } from "./types";
import { functionCall } from "./workspaces";
import { ExecutionContext } from "ava";

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
): Promise<ExtKeyInfo> {
  const keyInformation: ExtKeyInfo = await keypom.view("get_key_information", {
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

export async function claimWithRequiredGas({
  keypom,
  keyPair,
  root,
  fcArgs,
  password,
  receiverId,
  createAccount = false,
  useLongAccount = true,
  useImplicitAccount = false,
  shouldPanic = false,
}: {
  keypom: NearAccount;
  keyPair: KeyPair;
  root: NearAccount;
  fcArgs?: UserProvidedFCArgs;
  password?: string;
  receiverId?: string;
  createAccount?: boolean;
  useLongAccount?: boolean;
  useImplicitAccount?: boolean;
  shouldPanic?: boolean;
}) {
  // Set key and get required gas
  await keypom.setKey(keyPair);
  let keyPk = keyPair.getPublicKey().toString();

  const keyInfo: { required_gas: string } = await keypom.view(
    "get_key_information",
    { key: keyPk }
  );
  console.log("keyInfo: ", keyInfo);

  // To allow custom receiver ID without needing to specify useLongAccount
  if (receiverId != undefined && !createAccount) {
    useLongAccount = false;
  }

  // customized error message to reduce chances of accidentally passing in this receiverid and throwing an error
  let errorMsg = "Error-" + Date.now();

  // actualReceiverId for non-forced-failure case
  let actualReceiverId = useLongAccount
    ? createAccount
      ? `ac${Date.now().toString().repeat(4)}.${root.accountId}`
      : useImplicitAccount
      ? Buffer.from(PublicKey.fromString(keyPk).data).toString("hex")
      : errorMsg
    : receiverId;
  if (actualReceiverId == errorMsg) {
    throw new Error(
      "Must specify desired usage, see claimWithRequiredGas function for more information"
    );
  }

  if (createAccount) {
    // Generate new keypair
    let keyPairs = await generateKeyPairs(1);
    let newPublicKey = keyPairs.publicKeys[0];

    if (receiverId != undefined) {
      actualReceiverId = receiverId;
    }

    console.log(
      `create_account_and_claim with ${actualReceiverId} with ${keyInfo.required_gas} Gas`
    );
    let response = await functionCall({
      signer: keypom,
      receiver: keypom,
      methodName: "create_account_and_claim",
      args: {
        new_account_id: actualReceiverId,
        new_public_key: newPublicKey,
        fc_args: fcArgs,
        password,
      },
      gas: keyInfo.required_gas,
      shouldPanic,
    });
    console.log(`Response from create_account_and_claim: ${response}`);
    return { response, actualReceiverId };
  }

  console.log(
    `claim with ${actualReceiverId} with ${keyInfo.required_gas} Gas`
  );

  let response = await functionCall({
    signer: keypom,
    receiver: keypom,
    methodName: "claim",
    args: {
      account_id: actualReceiverId,
      fc_args: fcArgs,
      password,
    },
    gas: keyInfo.required_gas,
    shouldPanic,
  });
  console.log(response);
  return { response, actualReceiverId };
}

export const claimAndAssertTrialCreated = async ({
  t,
  keypom,
  nearcon,
  keyPair,
}: {
  t: ExecutionContext;
  keypom: NearAccount;
  nearcon: NearAccount;
  keyPair: KeyPair;
}) => {
  let newAccountId = `benji.${nearcon.accountId}`;
  await claimWithRequiredGas({
    keypom,
    root: keypom,
    keyPair,
    receiverId: newAccountId,
    createAccount: true,
    useLongAccount: false,
    useImplicitAccount: false,
    shouldPanic: false,
  });

  try {
    const keyInfo = await keypom.view("get_key_information", {
      key: keyPair.getPublicKey().toString(),
    });
    t.fail();
  } catch (e) {
    t.pass();
  }

  let account = keypom.getAccount(newAccountId);
  let doesExist = await account.exists();
  t.is(doesExist, true, `Account ${newAccountId} does not exist`);
  let rules: TrialRules = await account.view("get_rules", {});
  console.log("rules: ", rules);
  t.deepEqual(rules, {
    amounts: "100000000000000000000000000",
    contracts: "nearcon.keypom.near",
    floor: "0",
    funder: "",
    methods: "*",
    repay: "0",
    current_floor: "1000000000000000000000000",
  });
};

export const addKeys = async ({
  funder,
  keypom,
  originalTicketOwner,
  numKeys,
  numOwners,
  dropId,
}: {
  funder: NearAccount;
  keypom: NearAccount;
  originalTicketOwner: NearAccount;
  numKeys: number;
  numOwners: number;
  dropId: string;
}): Promise<{ keys: KeyPair[]; publicKeys: string[] }> => {
  let { keys, publicKeys } = await generateKeyPairs(numKeys);
  let keyData: Array<any> = [];
  let basePassword = "nearcon23-password";
  let idx = 0;
  for (var pk of publicKeys) {
    let password_by_use = generatePasswordsForKey(pk, [1], basePassword);
    keyData.push({
      public_key: pk,
      password_by_use,
      key_owner: idx < numOwners ? originalTicketOwner.accountId : null,
    });
    idx += 1;
  }

  await functionCall({
    signer: funder,
    receiver: keypom,
    methodName: "add_keys",
    args: {
      drop_id: dropId,
      key_data: keyData,
    },
    attachedDeposit: NEAR.parse("20").toString(),
  });

  return { keys, publicKeys };
};
