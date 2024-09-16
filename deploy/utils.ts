import { GLOBAL_NETWORK, TICKET_URL_BASE } from "./config";

const {
  KeyPair,
  connect,
  utils,
  transactions,
  keyStores,
} = require("near-api-js");
const fs = require("fs");
const path = require("path");
const homedir = require("os").homedir();

const CREDENTIALS_DIR = ".near-credentials";
const credentialsPath = path.join(homedir, CREDENTIALS_DIR);
const keyStore = new keyStores.UnencryptedFileSystemKeyStore(credentialsPath);

const config = {
  keyStore,
  networkId: GLOBAL_NETWORK,
  nodeUrl: `https://rpc.${GLOBAL_NETWORK}.near.org`,
};

export async function initNear() {
  const near = await connect({ ...config, keyStore });
  return near;
}

export async function sendTransaction({
  signerAccount,
  receiverId,
  methodName,
  args,
  deposit,
  gas,
  wasmPath = undefined,
}: {
  signerAccount: any;
  receiverId: string;
  methodName: string;
  args: any;
  deposit: string;
  gas: string;
  wasmPath?: string;
}) {
  const result = await signerAccount.signAndSendTransaction({
    receiverId: receiverId,
    actions: [
      ...(wasmPath
        ? [transactions.deployContract(fs.readFileSync(wasmPath))]
        : []),
      transactions.functionCall(
        methodName,
        Buffer.from(JSON.stringify(args)),
        gas,
        utils.format.parseNearAmount(deposit),
      ),
    ],
  });
  return result;
}

export async function createAccountDeployContract({
  signerAccount,
  newAccountId,
  amount,
  near,
  wasmPath,
  methodName,
  args,
  deposit = "0",
  gas = "300000000000000",
}: {
  signerAccount: any;
  newAccountId: string;
  amount: string;
  near: any;
  wasmPath: string;
  methodName: string;
  args: any;
  deposit?: string;
  gas?: string;
}) {
  console.log("Creating account: ", newAccountId);
  let keyPair = await createAccount({ signerAccount, newAccountId, amount });
  console.log("Deploying contract: ", newAccountId);
  const accountObj = await near.account(newAccountId);
  await sendTransaction({
    signerAccount: accountObj,
    receiverId: newAccountId,
    methodName,
    args,
    deposit,
    gas,
    wasmPath,
  });

  console.log("Deployed.");
  return keyPair;
}

export async function createAccount({
  signerAccount,
  newAccountId,
  amount,
}: {
  signerAccount: any;
  newAccountId: string;
  amount: string;
}) {
  const keyPair = KeyPair.fromRandom("ed25519");
  const publicKey = keyPair.publicKey.toString();
  await keyStore.setKey(config.networkId, newAccountId, keyPair);

  await signerAccount.functionCall({
    contractId: GLOBAL_NETWORK === "testnet" ? "testnet" : "near",
    methodName: "create_account",
    args: {
      new_account_id: newAccountId,
      new_public_key: publicKey,
    },
    gas: "300000000000000",
    attachedDeposit: utils.format.parseNearAmount(amount),
  });
  return keyPair.toString();
}

// Convert the map to CSV with the key and the raw JSON stringified data
export const convertMapToRawJsonCsv = (
  map: Map<string, Record<string, string>>,
): string => {
  let csvString = "Secret Key,Raw JSON Data\n"; // CSV header

  for (const [encodedTicket, attendeeInfo] of map.entries()) {
    const rawJsonData = JSON.stringify(attendeeInfo);
    csvString += `${TICKET_URL_BASE}${encodedTicket},"${rawJsonData}"\n`;
  }

  return csvString;
};

// Helper function to update the EXISTING_FACTORY in config.ts dynamically
export const updateConfigFile = (newFactoryId: string) => {
  const configFilePath = path.join(__dirname, "config.ts");

  // Read the current content of config.ts
  const configContent = fs.readFileSync(configFilePath, "utf-8");

  // Replace the EXISTING_FACTORY value
  const updatedContent = configContent.replace(
    /export const EXISTING_FACTORY = `.*?`;/,
    `export const EXISTING_FACTORY = \`${newFactoryId}\`;`,
  );

  // Write the updated content back to config.ts
  fs.writeFileSync(configFilePath, updatedContent);
  console.log(`Updated config.ts with new factory ID: ${newFactoryId}`);
};
