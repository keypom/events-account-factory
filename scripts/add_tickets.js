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
const nearconFactory = "keypom-factory.keypom.testnet";
const keypomContract = "ncon23.keypom.testnet";

// 3 tickets will be added. The first two will not be owned by anyone and the last will be owned by benjiman.testnet
const ticketOwners = [
    null,
    null,
    "benjiman.testnet",
];

const ticketBaseUrl = "https://test.near.org/nearpad.testnet/widget/Index?tab=ticket";
const dropId = "nearcon2023";
let basePassword = "nearcon2023-password";

const main = async () => {
    // Initiate connection to the NEAR blockchain.
    const CREDENTIALS_DIR = '.near-credentials';
    const credentialsPath = path.join(homedir, CREDENTIALS_DIR);

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

    let keyData = {
        keys: [],
        publicKeys: []
    };
    // Loop through from 0 -> ticketOwners.length 50 at a time
    for (let i = 0; i < ticketOwners.length; i += 50) {
        // Get the ticket owners for this current iteration
        let ticketOwnersForIteration = ticketOwners.slice(i, i + 50);

        let { keys, publicKeys } = await addKeys({
            funderAccount,
            keypomAccount,
            ticketOwners: ticketOwnersForIteration,
            dropId
        })

        console.log('keys: ', keys);
        console.log('publicKeys: ', publicKeys);

        keyData.keys = keyData.keys.concat(keys);
        keyData.publicKeys = keyData.publicKeys.concat(publicKeys);
    }

    let stringToWrite = '';
    // Loop through each secret key
    var i = 0;
    for (const sk of keyData.keys) {
        stringToWrite += `${ticketBaseUrl}&secretKey=${sk}&contractId=${keypomContract}` + '\n';
        i++;
    }

    await writeFile(path.resolve(__dirname, 'tickets-added.json'), stringToWrite);
}


const addKeys = async ({
    funderAccount,
    keypomAccount,
    ticketOwners,
    dropId,
}) => {
    let keyData = [];
    let keysToReturn = [];
    let publicKeysToReturn = [];

    for (var ticketOwner of ticketOwners) {
        let { keys, publicKeys } = await generateKeyPairs(1);

        // Concat the keys and publicKeys
        keysToReturn = keysToReturn.concat(keys);
        publicKeysToReturn = publicKeysToReturn.concat(publicKeys);

        let password_by_use = generatePasswordsForKey(publicKeys[0], [1], basePassword);
        keyData.push({
            public_key: publicKeys[0],
            password_by_use,
            key_owner: ticketOwner,
        });
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

    return { keys: keysToReturn, publicKeys: publicKeysToReturn };
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