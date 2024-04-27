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

const NETWORK_ID = 'mainnet';

const funderAccountId = 'keypom.near'; // INSERT VERIKEN'S ACCOUNT ID HERE
const nearconFactory = "nearcon23.near";
const keypomContract = "ncon23.keypom.near";
const dropId = "nearcon2023";

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
    const keypomAccount = new Account(near.connection, keypomContract);

    await createNearconDrop({
        funderAccount,
        keypomAccount,
    });
}

const createNearconDrop = async ({
    funderAccount,
    keypomAccount
  }) => {
    let assetData = [
      {uses: 1, assets: [null], config: {permissions: "claim"}},
      {uses: 1, assets: [null], config: {permissions: "create_account_and_claim", account_creation_keypom_args: {drop_id_field: "drop_id"}, root_account_id: nearconFactory}},
    ];

    await funderAccount.functionCall({
        contractId: keypomAccount.accountId,
        methodName: 'create_drop',
        args: {
          drop_id: dropId,
          key_data: [],
          drop_config: {
              delete_empty_drop: false
          },
          drop_config: {
            add_key_allowlist: ["2023.nearcontickets.near"]
          },
          asset_data: assetData,
          keep_excess_deposit: true
        },
        gas: 300000000000000,
        attachedDeposit: parseNearAmount("1").toString()
    });
}

main()