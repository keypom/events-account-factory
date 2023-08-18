import { BN } from "bn.js";
import { KeyPair, NEAR, NearAccount } from "near-workspaces";
import { ExtDrop, ExtKeyInfo, ListingJson } from "../utils/types";
import { addKeys, generateKeyPairs, generatePasswordsForKey, getKeyInformation } from "../utils/keypom";
import { functionCall } from "../utils/workspaces";

export const sellNFT = async ({
    keypom, 
    mintbase, 
    seller, 
    buyer, 
    sellerKeys, 
    buyerKeys, 
    t, 
    tokenId
}: {
    keypom: NearAccount;
    mintbase: NearAccount;
    seller: NearAccount;
    buyer: NearAccount;
    sellerKeys: { keys: KeyPair[]; publicKeys: string[] };
    buyerKeys: { keys: KeyPair[]; publicKeys: string[] };
    t: any;
    tokenId: string;
}) => {
    await keypom.setKey(sellerKeys.keys[0]);
    let new_mintbase_args = JSON.stringify({
        price: NEAR.parse('1').toString(),
        owner_pub_key: seller == keypom ? sellerKeys.publicKeys[0] : undefined
    })
    console.log('new_mintbase_args: ', new_mintbase_args)
    await keypom.call(keypom, 'nft_approve', {account_id: mintbase.accountId, msg: new_mintbase_args});
    let listing: ListingJson = await mintbase.view('get_listing', {nft_contract_id: keypom, token_id: tokenId});
    t.assert(listing.nft_token_id === tokenId);
    t.assert(listing.price === NEAR.parse('1').toString());
    t.assert(listing.nft_owner_id === seller.accountId);
    t.assert(listing.nft_contract_id === keypom.accountId);
    t.assert(listing.currency === 'near');

    /// Buyer purchases the key
    await buyer.call(mintbase, 'buy', {nft_contract_id: keypom.accountId, token_id: tokenId, new_pub_key: buyerKeys.publicKeys[0]}, {attachedDeposit: NEAR.parse('1').toString(), gas: '300000000000000'});

    // Now that buyer bought the key, his key should have the same allowance as what seller left off with and should have all remaining uses
    let keyInfo = await getKeyInformation(keypom, buyerKeys.publicKeys[0]);
    t.is(keyInfo.owner_id, buyer.accountId);
    t.is(keyInfo.uses_remaining, 2);

    try {
        // Seller should now have a simple $NEAR drop with 0.05 $NEAR less than the 1 $NEAR purchase price
        let sellerNewDrop: ExtDrop = await keypom.view('get_drop_information', {key: sellerKeys.publicKeys[0]});
        console.log('sellerNewDrop: ', sellerNewDrop)
        if (seller == keypom) {
            t.is(sellerNewDrop.asset_data.length, 1);
            t.is(sellerNewDrop.asset_data[0].uses, 1);

            let sellerNewKey: ExtKeyInfo = await keypom.view('get_key_information', {key: sellerKeys.publicKeys[0]});
            console.log('sellerNewKey: ', sellerNewKey)
            t.is(sellerNewKey.uses_remaining, 1);
            t.is(sellerNewKey.owner_id, keypom.accountId);
            t.is(sellerNewKey.yoctonear, NEAR.parse('0.95').toString())
        } else {
            t.fail();
        }
    } catch(e) {
        seller == keypom ? t.fail() : t.pass();
    }
}

export const createNearconDrop = async ({
    funder,
    keypom,
    nearcon,
    originalTicketOwner,
    numKeys,
    numOwners
  }: {
    funder: NearAccount;
    keypom: NearAccount;
    nearcon: NearAccount;
    originalTicketOwner: NearAccount;
    numKeys: number;
    numOwners: number;
  }): Promise<{ keys: KeyPair[]; publicKeys: string[] }> => {
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
              delete_empty_drop: false,
              extra_allowance_per_key: NEAR.parse("0.02")
          },
          asset_data: assetData,
          keep_excess_deposit: true
      },
      attachedDeposit: NEAR.parse("21").toString()
  })

  let keyData = {
    keys: [],
    publicKeys: []
  };
  // Loop through from 0 -> numKeys 50 at a time
    for (let i = 0; i < numKeys; i += 50) {
        let {keys, publicKeys} = await addKeys({
            funder,
            keypom,
            originalTicketOwner,
            numKeys: Math.min(numKeys - i, 50),
            numOwners: Math.min(numOwners - i, 50),
            dropId
        })

        keyData.keys = keyData.keys.concat(keys as never[]);
        keyData.publicKeys = keyData.publicKeys.concat(publicKeys as never[]);
    }
    return keyData;
}