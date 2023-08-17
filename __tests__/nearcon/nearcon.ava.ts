import anyTest, { TestFn } from "ava";
import { NEAR, NearAccount, Worker } from "near-workspaces";
import { CONTRACT_METADATA, generateKeyPairs, generatePasswordsForKey } from "../utils/keypom";
import { functionCall } from "../utils/workspaces";

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
  console.log("Starting test");
  // Init the worker and start a Sandbox server
  const worker = await Worker.init();

  // Prepare sandbox for tests, create accounts, deploy contracts, etc.
  const root = worker.rootAccount;

  const keypom = await root.createSubAccount("keypom");
  await keypom.deploy(`./__tests__/ext-wasm/keypom.wasm`);
  console.log("Deployed Keypom");

  const mintbase = await root.createSubAccount("mintbase");
  await mintbase.deploy(`./__tests__/ext-wasm/mintbase-new.wasm`);
  console.log("Deployed Mintbase");

  const nearcon = await root.createSubAccount("nearcon");
  await nearcon.deploy(`./out/factory.wasm`);
  console.log("Deployed Nearcon");

  // Init empty/default linkdrop contract
  await keypom.call(keypom, "new", {
    root_account: "test.near",
    owner_id: keypom,
    contract_metadata: CONTRACT_METADATA,
  });
  await mintbase.call(mintbase, 'init', { owner: mintbase, mintbase_cut: 0, fallback_cut: 0, listing_lock_seconds: "0", keypom_contract_root: keypom.accountId });
  await nearcon.call(nearcon, 'new', {
      allowed_drop_id: 'nearcon-drop',
      keypom_contract: keypom.accountId,
      starting_ncon_balance: NEAR.parse("1").toString(),
      starting_near_balance: NEAR.parse("1").toString(),
  });
  console.log("Initialized contracts");

  await keypom.call(mintbase, 'deposit_storage', {},{attachedDeposit: NEAR.parse("10").toString()});

  // Test users
  const funder = await root.createSubAccount('funder');
  const bob = await root.createSubAccount('bob');

  // Add 10k $NEAR to owner's account
  await funder.updateAccount({
      amount: NEAR.parse('10000 N').toString()
  })
  await funder.call(keypom, 'add_to_balance', {}, {attachedDeposit: NEAR.parse("5000").toString()});

  // Save state for test runs
  t.context.worker = worker;
  t.context.accounts = { root, keypom, nearcon, funder, bob, mintbase };
});

// If the environment is reused, use test.after to replace test.afterEach
test.afterEach(async (t) => {
  await t.context.worker.tearDown().catch((error) => {
    console.log("Failed to tear down the worker:", error);
  });
});

test("Creating & Claiming Drop", async (t) => {
  const { keypom, funder, nearcon, bob, root, mintbase } = t.context.accounts;
  const dropId = "nearcon-drop";
  let assetData = [
      {uses: 1, assets: [null], config: {permissions: "claim"}}, // Password protected scan into the event
      {uses: 1, assets: [null], config: {permissions: "create_account_and_claim", account_creation_keypom_args: {drop_id_field: "drop_id"}, root_account_id: nearcon.accountId}},
        // Create their trial account, deposit their fungible tokens, deploy the contract & call setup
    ];
  await functionCall({
      signer: funder,
      receiver: keypom,
      methodName: 'create_drop',
      args: {
          drop_id: dropId,
          key_data: [],
          drop_config: {
              delete_empty_drop: false
          },
          asset_data: assetData,
          keep_excess_deposit: true
      },
      attachedDeposit: NEAR.parse("21").toString()
  })
  let numKeys = 20;
  let originalOwnerKeys = await generateKeyPairs(numKeys);
  let keyData: Array<any> = [];
  let basePassword = "nearcon23-password"
  let idx = 0;
  for (var pk of originalOwnerKeys.publicKeys) {
      let password_by_use = generatePasswordsForKey(pk, [1], basePassword);
      keyData.push({
          public_key: pk,
          password_by_use,
          key_owner: idx > numKeys / 2 ? funder.accountId : null
      })
      idx += 1;
  }
  await functionCall({
      signer: funder,
      receiver: keypom,
      methodName: 'add_keys',
      args: {
          drop_id: dropId,
          key_data: keyData,
      },
      attachedDeposit: NEAR.parse("21").toString()
  })
});
