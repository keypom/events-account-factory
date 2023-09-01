import anyTest, { TestFn } from "ava";
import { NEAR, NearAccount, Worker } from "near-workspaces";
import {
  CONTRACT_METADATA,
  claimAndAssertAccountCreated,
  claimWithRequiredGas,
  generateKeyPairs,
  generatePasswordsForKey,
  hash,
} from "../utils/keypom";
import { functionCall } from "../utils/workspaces";
import { createNearconDrop, sellNFT } from "./utils";
import {
  ExtDrop,
  ExtKeyInfo,
  ExtNFTKey,
  ListingJson,
  TrialRules,
} from "../utils/types";

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
  await root.deploy(`./__tests__/ext-wasm/linkdrop.wasm`);

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
  await root.call(root, "new", {});
  await keypom.call(keypom, "new", {
    root_account: "test.near",
    owner_id: keypom,
    contract_metadata: CONTRACT_METADATA,
  });
  await mintbase.call(mintbase, "init", {
    owner: mintbase,
    mintbase_cut: 0,
    fallback_cut: 0,
    listing_lock_seconds: "0",
    keypom_contract_root: keypom.accountId,
  });
  await nearcon.call(nearcon, "new", {
    allowed_drop_id: "nearcon-drop",
    keypom_contract: keypom.accountId,
    starting_ncon_balance: NEAR.parse("1").toString(),
    starting_near_balance: NEAR.parse("1").toString(),
  });
  console.log("Initialized contracts");

  await keypom.call(
    mintbase,
    "deposit_storage",
    {},
    { attachedDeposit: NEAR.parse("10").toString() }
  );

  // Test users
  const funder = await root.createSubAccount("funder");
  const originalTicketOwner = await root.createSubAccount("og-ticket-owner");
  const newTicketBuyer = await root.createSubAccount("new-ticket-buyer");

  // Add 10k $NEAR to owner's account
  await funder.updateAccount({
    amount: NEAR.parse("10000 N").toString(),
  });
  await funder.call(
    keypom,
    "add_to_balance",
    {},
    { attachedDeposit: NEAR.parse("5000").toString() }
  );

  // Save state for test runs
  t.context.worker = worker;
  t.context.accounts = {
    root,
    keypom,
    nearcon,
    funder,
    newTicketBuyer,
    originalTicketOwner,
    mintbase,
  };
});

// If the environment is reused, use test.after to replace test.afterEach
test.afterEach(async (t) => {
  await t.context.worker.tearDown().catch((error) => {
    console.log("Failed to tear down the worker:", error);
  });
});

test("Journey 3: New to NEAR Purchases & Sells on Secondary Marketplace", async (t) => {
  const {
    keypom,
    funder,
    nearcon,
    newTicketBuyer,
    originalTicketOwner,
    root,
    mintbase,
  } = t.context.accounts;
  let sellerKeys = await createNearconDrop({
    funder,
    keypom,
    nearcon,
    originalTicketOwner,
    numKeys: 1,
    numOwners: 0,
  });

  let nfts: Array<ExtNFTKey> = await keypom.view("nft_tokens");
  console.log("nfts: ", nfts);
  t.is(nfts.length, 1);
  t.is(nfts[0].owner_id, keypom.accountId);

  const buyerKeys = await generateKeyPairs(1);
  await sellNFT({
    t,
    keypom,
    mintbase,
    seller: keypom,
    buyer: newTicketBuyer,
    sellerKeys,
    buyerKeys,
    tokenId: `nearcon-drop:0`,
  });

  // Claim seller key and create new account
  let newAccountId = `benji.${root.accountId}`;
  await claimWithRequiredGas({
    keypom,
    root,
    keyPair: sellerKeys.keys[0],
    receiverId: newAccountId,
    createAccount: true,
    useLongAccount: false,
    useImplicitAccount: false,
    shouldPanic: false,
  });
  try {
    const keyInfo = await keypom.view("get_key_information", {
      key: sellerKeys.publicKeys[0],
    });
    t.fail();
  } catch (e) {
    t.pass();
  }

  await claimAndAssertAccountCreated({
    t,
    keypom,
    nearcon,
    keyPair: buyerKeys.keys[0],
  });

  nfts = await keypom.view("nft_tokens");
  console.log("nfts: ", nfts);
  t.is(nfts.length, 0);
});

// test("Journey 2: Crypto Native Purchases & Attends Conference", async (t) => {
//   const {
//     keypom,
//     funder,
//     nearcon,
//     newTicketBuyer,
//     originalTicketOwner,
//     root,
//     mintbase,
//   } = t.context.accounts;
//   let { keys, publicKeys } = await createNearconDrop({
//     funder,
//     keypom,
//     nearcon,
//     originalTicketOwner,
//     numKeys: 1,
//     numOwners: 1,
//   });

//   let nfts: Array<ExtNFTKey> = await keypom.view("nft_tokens");
//   console.log("nfts: ", nfts);
//   t.is(nfts.length, 1);
//   t.is(nfts[0].owner_id, originalTicketOwner.accountId);

//   // This should panic because no password is passed in
//   await claimWithRequiredGas({
//     keypom,
//     root: keypom,
//     keyPair: keys[0],
//     receiverId: "foo",
//     shouldPanic: true,
//   });

//   // This should pass because the password is correct
//   await claimWithRequiredGas({
//     keypom,
//     root: keypom,
//     keyPair: keys[0],
//     receiverId: "foo",
//     password: hash("nearcon23-password" + publicKeys[0] + "1"),
//     shouldPanic: false,
//   });

//   let keyInfo: ExtKeyInfo = await keypom.view("get_key_information", {
//     key: publicKeys[0],
//   });
//   console.log("keyInfo: ", keyInfo);
//   t.is(keyInfo.uses_remaining, 1);

//   await claimAndAssertAccountCreated({
//     t,
//     keypom,
//     nearcon,
//     keyPair: keys[0],
//   });

//   nfts = await keypom.view("nft_tokens");
//   console.log("nfts: ", nfts);
//   t.is(nfts.length, 0);
// });

// test("Journey 1: New to NEAR Purchases & Attends Conference", async (t) => {
//   const {
//     keypom,
//     funder,
//     nearcon,
//     newTicketBuyer,
//     originalTicketOwner,
//     root,
//     mintbase,
//   } = t.context.accounts;
//   let { keys, publicKeys } = await createNearconDrop({
//     funder,
//     keypom,
//     originalTicketOwner,
//     nearcon,
//     numKeys: 1,
//     numOwners: 0,
//   });

//   let nfts: Array<ExtNFTKey> = await keypom.view("nft_tokens");
//   console.log("nfts: ", nfts);
//   t.is(nfts.length, 1);
//   t.is(nfts[0].owner_id, keypom.accountId);

//   // This should panic because no password is passed in
//   await claimWithRequiredGas({
//     keypom,
//     root: keypom,
//     keyPair: keys[0],
//     receiverId: "foo",
//     shouldPanic: true,
//   });

//   // This should pass because the password is correct
//   await claimWithRequiredGas({
//     keypom,
//     root: keypom,
//     keyPair: keys[0],
//     receiverId: "foo",
//     password: hash("nearcon23-password" + publicKeys[0] + "1"),
//     shouldPanic: false,
//   });

//   let keyInfo: ExtKeyInfo = await keypom.view("get_key_information", {
//     key: publicKeys[0],
//   });
//   console.log("keyInfo: ", keyInfo);
//   t.is(keyInfo.uses_remaining, 1);

//   await claimAndAssertAccountCreated({
//     t,
//     keypom,
//     nearcon,
//     keyPair: keys[0],
//   });
//   nfts = await keypom.view("nft_tokens");
//   console.log("nfts: ", nfts);
//   t.is(nfts.length, 0);
// });

// test("Creating Lots of Keys & Claiming", async (t) => {
//   const {
//     keypom,
//     funder,
//     nearcon,
//     newTicketBuyer,
//     originalTicketOwner,
//     root,
//     mintbase,
//   } = t.context.accounts;
//   let { keys, publicKeys } = await createNearconDrop({
//     funder,
//     keypom,
//     nearcon,
//     originalTicketOwner,
//     numKeys: 75,
//     numOwners: 25,
//   });

//   let numNfts = await keypom.view("nft_total_supply");
//   console.log("numNfts: ", numNfts);
//   t.is(numNfts, "75");

//   let nftsOwned = await keypom.view("nft_supply_for_owner", {
//     account_id: originalTicketOwner.accountId,
//   });
//   console.log("nftsOwned: ", nftsOwned);
//   t.is(nftsOwned, "25");

//   // This should panic because no password is passed in
//   await claimWithRequiredGas({
//     keypom,
//     root: keypom,
//     keyPair: keys[0],
//     receiverId: "foo",
//     shouldPanic: true,
//   });

//   await claimWithRequiredGas({
//     keypom,
//     root: keypom,
//     keyPair: keys[0],
//     receiverId: "foo",
//     password: hash("nearcon23-password" + publicKeys[0] + "1"),
//   });

//   await claimAndAssertAccountCreated({
//     t,
//     keypom,
//     nearcon,
//     keyPair: keys[0],
//   });
// });
