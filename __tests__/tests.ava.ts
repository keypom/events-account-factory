import anyTest, { TestFn } from "ava";
import { NEAR, NearAccount, Worker } from "near-workspaces";
import { TrialRules, generateKeyPairs, parseExecutionResults } from "./utils";
const { readFileSync } = require('fs');

const test = anyTest as TestFn<{
    worker: Worker;
    accounts: Record<string, NearAccount>;
    rpcPort: string;
  }>;

test.beforeEach(async (t) => {
    console.log(t.title);
    // Init the worker and start a Sandbox server
    const worker = await Worker.init();

    const rpcPort = (worker as any).config.rpcAddr
    console.log(`rpcPort: `, rpcPort)
    
    // Prepare sandbox for tests, create accounts, deploy contracts, etc.
    const root = worker.rootAccount;
    
    const nearcon = await root.createSubAccount('nearcon23');
    
    // Deploy the keypom contract.
    await nearcon.deploy(`./out/factory.wasm`);

    await nearcon.call(nearcon, 'new', {
        allowed_drop_id: 'nearcon-drop', 
        keypom_contract: 'keypom.test.near',
        starting_ncon_balance: NEAR.parse("1").toString(),
        starting_near_balance: NEAR.parse("1").toString(),
    });

    // Test users
    const funder = await root.createSubAccount('funder');
    const admin = await root.createSubAccount('admin');
    const foodVendor = await root.createSubAccount('food_vendor');
    const merchVendor = await root.createSubAccount('merch_vendor');

    // Save state for test runs
    t.context.worker = worker;
    t.context.accounts = { root, nearcon, funder, admin, foodVendor, merchVendor };
    t.context.rpcPort = rpcPort;
});

// If the environment is reused, use test.after to replace test.afterEach
test.afterEach(async t => {
    await t.context.worker.tearDown().catch(error => {
        console.log('Failed to tear down the worker:', error);
    });
});

// test('Create Duplicate Accounts', async t => {
//     const { root, nearcon, funder } = t.context.accounts;
//     const rpcPort = t.context.rpcPort;
//     const dropId = 'nearcon-drop';
//     const keys = await generateKeyPairs(1);
//     const newAccountId = `benji.${nearcon.accountId}`

//     // Loop 3 times
//     for (let i = 0; i < 3; i++) {
//         let rawVal = await nearcon.callRaw(nearcon, 'create_account', {
//             new_account_id: newAccountId, 
//             new_public_key: keys.publicKeys[0],
//             drop_id: dropId,
//             keypom_args: {
//                 drop_id_field: "drop_id"
//             }
//         });

//         parseExecutionResults(
//             "create_account",
//             nearcon.accountId,
//             rawVal,
//             true,
//             false
//         );

//         let expectedAccountId = i == 0 ? `benji.${nearcon.accountId}` : `benji-${i}.${nearcon.accountId}`
//         let account = root.getAccount(expectedAccountId);
//         let doesExist = await account.exists();
//         t.is(doesExist, true, `Account ${expectedAccountId} does not exist`);

//         let accountContract = await account.viewCode();
//         console.log('accountContract: ', accountContract)

//         let rules: TrialRules = await account.view("get_rules", {});
//         console.log('rules: ', rules)
//         t.is(rules, 
//         {
//             amounts: '100000000000000000000000000',
//             contracts: 'testnet',
//             floor: '0',
//             funder: 'bar',
//             methods: 'bar',
//             repay: 'bar',
//             current_floor: '1000000000000000000000000'
//         });
//     }
// });

//testing drop empty initialization and that default values perform as expected
test('Adding Vendor Items', async t => {
    const { root, nearcon, funder, admin, merchVendor, foodVendor } = t.context.accounts;

    await nearcon.call(nearcon, 'add_admin', {account_ids: [admin.accountId]});

    let vendorMetadata = {
        name: "Benji's Homegrown Burgers!",
        description: "The greatest burgers in town.",
        cover_image: "bafybeihnb36l3xvpehkwpszthta4ic6bygjkyckp5cffxvszbcltzyjcwi",
    };
    await admin.call(nearcon, 'add_vendor', {
        vendor_id: merchVendor, 
        vendor_metadata: vendorMetadata
    });
    let metadata = await nearcon.view('get_vendor_metadata', {vendor_id: merchVendor});
    console.log('metadata: ', metadata)
    t.deepEqual(metadata, vendorMetadata);

    let items = await nearcon.view('get_items_for_vendor', {vendor_id: merchVendor});
    console.log('items: ', items)
    t.deepEqual(items, []);
});

// test('Purchase vendor items', async t => {
// });

// test('Drop tokens to users', async t => {
// });