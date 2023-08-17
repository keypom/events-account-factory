import anyTest, { TestFn } from "ava";
import { NEAR, NearAccount, Worker } from "near-workspaces";
import { CONTRACT_METADATA, claimWithRequiredGas, generateKeyPairs, generatePasswordsForKey, hash } from "../utils/keypom";
import { functionCall } from "../utils/workspaces";
import { createNearconDrop } from "./utils";
import { TrialRules } from "../utils/types";

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
  let {keys, publicKeys} = await createNearconDrop({
      funder,
      keypom,
      nearcon,
      numKeys: 75,
      numOwners: 25
  });

  let numNfts = await keypom.view('nft_total_supply');
  console.log('numNfts: ', numNfts)
  t.is(numNfts, "75");

  let nftsOwned = await keypom.view('nft_supply_for_owner', {account_id: funder.accountId});
  console.log('nftsOwned: ', nftsOwned)
  t.is(nftsOwned, "25");

  // This should panic because no password is passed in
  await claimWithRequiredGas({
    keypom,
    root: keypom,
    keyPair: keys[0],
    receiverId: bob.accountId,
    shouldPanic: true
  });

  await claimWithRequiredGas({
    keypom,
    root: keypom,
    keyPair: keys[0],
    receiverId: bob.accountId,
    password: hash("nearcon23-password" + publicKeys[0] + "1"),
    shouldPanic: false
  });

  let keyInfo: {uses_remaining: number} = await keypom.view('get_key_information', {key: publicKeys[0]});
  console.log('keyInfo: ', keyInfo)
  t.is(keyInfo.uses_remaining, 1);

  let newAccountId = `benji.${nearcon.accountId}`
  await claimWithRequiredGas({
    keypom,
    root: keypom,
    keyPair: keys[0],
    receiverId: newAccountId,
    createAccount: true,
    useLongAccount: false,
    useImplicitAccount: false,
    shouldPanic: false
  });

  try {
    keyInfo = await keypom.view('get_key_information', {key: publicKeys[0]});
    t.fail();
  } catch(e) {
    t.pass()
  }

  let account = root.getAccount(newAccountId);
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
});
