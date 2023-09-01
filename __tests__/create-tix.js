const path = require("path");
const homedir = require("os").homedir();
const { KeyPair } = require("@near-js/crypto");
const { createHash } = require("crypto");
const { readFileSync } = require('fs');
const { writeFile, mkdir, readFile } = require('fs/promises');
const { UnencryptedFileSystemKeyStore } = require("@near-js/keystores-node");
const { Account } = require('@near-js/accounts');
const { connect, Near } = require("@near-js/wallet-account");
const { parseNearAmount } = require('@near-js/utils');

const NETWORK_ID = 'testnet';

const funderAccountId = 'benjiman.testnet';
const mintbaseContract = "keypom.market.mintspace2.testnet";
const nearconFactory = "keypom-factory.keypom.testnet";
const keypomContract = "nearcon2023.keypom.testnet";

const originalTicketOwner = "benjiman.testnet";
const createDrop = false;
const numKeys = 10;
const numOwners = 0;

const baseUrl = "https://test.near.org/nearpad.testnet/widget/Index?tab=ticket";
const dropId = "nearcon2023";
let basePassword = "nearcon2023-password";

const main = async () => {
    // Initiate connection to the NEAR blockchain.
    const CREDENTIALS_DIR = '.near-credentials';
    const credentialsPath =  path.join(homedir, CREDENTIALS_DIR);

    let keyStore = new UnencryptedFileSystemKeyStore(credentialsPath);  

    let nearConfig = {
        networkId: NETWORK_ID,
        keyStore: keyStore,
        nodeUrl: `https://rpc.${NETWORK_ID}.near.org`,
        walletUrl: `https://wallet.${NETWORK_ID}.near.org`,
        helperUrl: `https://helper.${NETWORK_ID}.near.org`,
        explorerUrl: `https://explorer.${NETWORK_ID}.near.org`,
    };  

    let near = new Near(nearConfig);
    const funderAccount = new Account(near.connection, funderAccountId);
    console.log('funderAccount: ', funderAccount)
    const keypomAccount = new Account(near.connection, keypomContract);
    const nearconAccount = new Account(near.connection, nearconFactory);

    await keypomAccount.functionCall({
        contractId: mintbaseContract,
        methodName: 'deposit_storage',
        args: {},
        gas: 300000000000000,
        attachedDeposit: parseNearAmount("10")
    })

    let sellerKeys = await createNearconDrop({
        createDrop,
        funderAccount,
        keypomAccount,
        nearconAccount,
        originalTicketOwner,
        numKeys,
        numOwners,
      });

      let stringToWrite = '';
      // Loop through each secret key
      var i = 0;
      for (const sk of sellerKeys.keys) {
          stringToWrite += `${baseUrl}&secretKey=${sk}&contractId=${keypomContract}` + '\n';
          i++;
      }
  
      await writeFile(path.resolve(__dirname, 'nearcon-keys.json'), stringToWrite);
}

const createNearconDrop = async ({
    createDrop,
    funderAccount,
    keypomAccount,
    nearconAccount,
    originalTicketOwner,
    numKeys,
    numOwners
  }) => {
    let assetData = [
      {uses: 1, assets: [null], config: {permissions: "claim"}}, // Password protected scan into the event
      {uses: 1, assets: [null], config: {permissions: "create_account_and_claim", account_creation_keypom_args: {drop_id_field: "drop_id"}, root_account_id: nearconAccount.accountId}},
        // Create their trial account, deposit their fungible tokens, deploy the contract & call setup
    ];


    if (createDrop == true) {
      await funderAccount.functionCall({
        contractId: keypomAccount.accountId,
        methodName: 'create_drop',
        args: {
          drop_id: dropId,
          key_data: [],
          drop_config: {
              delete_empty_drop: false,
              extra_allowance_per_key: parseNearAmount("0.02").toString()
          },
          asset_data: assetData,
          keep_excess_deposit: true
        },
        gas: 300000000000000,
        attachedDeposit: parseNearAmount("100").toString()
    });
    }
    
  let keyData = {
    keys: [],
    publicKeys: []
  };
  // Loop through from 0 -> numKeys 50 at a time
    for (let i = 0; i < numKeys; i += 50) {
        let {keys, publicKeys} = await addKeys({
            funderAccount,
            keypomAccount,
            originalTicketOwner,
            numKeys: Math.min(numKeys - i, 50),
            numOwners: Math.min(numOwners - i, 50),
            dropId
        })

        keyData.keys = keyData.keys.concat(keys);
        keyData.publicKeys = keyData.publicKeys.concat(publicKeys);
    }
    return keyData;
}

const addKeys = async ({
  funderAccount,
  keypomAccount,
  originalTicketOwner,
  numKeys,
  numOwners,
  dropId,
}) => {
  let { keys, publicKeys } = await generateKeyPairs(numKeys);
  let keyData = [];
  
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

  await funderAccount.functionCall({
    contractId: keypomAccount.accountId,
    methodName: 'add_keys',
    args: {
      drop_id: dropId,
      key_data: keyData,
    },
    gas: 300000000000000,
    attachedDeposit: parseNearAmount("20").toString()
  });

  return { keys, publicKeys };
};

function hash(string, double = false) {
  if (double) {
    return createHash("sha256")
      .update(Buffer.from(string, "hex"))
      .digest("hex");
  }

  return createHash("sha256").update(Buffer.from(string)).digest("hex");
}

function generatePasswordsForKey(
  pubKey,
  usesWithPassword,
  basePassword
) {
  let passwords = {};

  // Loop through usesWithPassword
  for (var use of usesWithPassword) {
    passwords[use] = hash(hash(basePassword + pubKey + use.toString()), true);
  }

  return passwords;
}

async function generateKeyPairs(
  numKeys
 ) {
  // Generate NumKeys public keys
  let kps = [];
  let pks = [];
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

main()