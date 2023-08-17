import { BN } from "bn.js";
import { KeyPair, NEAR, NearAccount } from "near-workspaces";
import { JsonDrop, JsonKeyInfo, ListingJson } from "../utils/types";
import { getKeyInformation } from "../utils/keypom";

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
    // Now with migration out of the way, we can test the new mintbase contract and sell access keys
    let initialAllowance = (await getKeyInformation(keypom, sellerKeys.publicKeys[0])).allowance;
    console.log('initialAllowance: ', initialAllowance)

    await keypom.setKey(sellerKeys.keys[0]);
    let new_mintbase_args = JSON.stringify({
        price: NEAR.parse('1').toString(),
        owner_pub_key: seller == keypom ? sellerKeys.publicKeys[0] : undefined
    })
    await keypom.call(keypom, 'nft_approve', {account_id: mintbase.accountId, msg: new_mintbase_args});
    let listing: ListingJson = await mintbase.view('get_listing', {nft_contract_id: keypom, token_id: tokenId});
    t.assert(listing.nft_token_id === tokenId);
    t.assert(listing.price === NEAR.parse('1').toString());
    t.assert(listing.nft_owner_id === seller.accountId);
    t.assert(listing.nft_contract_id === keypom.accountId);
    t.assert(listing.currency === 'near');

    // After key is put for sale, its allowance should have decremented
    let keyInfo: JsonKeyInfo = await getKeyInformation(keypom, sellerKeys.publicKeys[0]);
    t.assert(new BN(initialAllowance).gt(new BN(keyInfo.allowance)));
    initialAllowance = keyInfo.allowance;

    /// Buyer purchases the key
    await buyer.call(mintbase, 'buy', {nft_contract_id: keypom.accountId, token_id: tokenId, new_pub_key: buyerKeys.publicKeys[0]}, {attachedDeposit: NEAR.parse('1').toString(), gas: '300000000000000'});

    // Now that buyer bought the key, his key should have the same allowance as what seller left off with and should have all remaining uses
    keyInfo = await getKeyInformation(keypom, buyerKeys.publicKeys[0]);
    t.is(keyInfo.owner_id, buyer.accountId);
    t.is(keyInfo.allowance, initialAllowance)
    t.is(keyInfo.remaining_uses, 2);

    try {
        // Seller should now have a simple $NEAR drop with 0.05 $NEAR less than the 1 $NEAR purchase price
        let sellerNewDrop: JsonDrop = await keypom.view('get_drop_information', {key: sellerKeys.publicKeys[0]});
        if (seller == keypom) {
            t.is(sellerNewDrop.deposit_per_use, NEAR.parse('0.95').toString());
            t.is(sellerNewDrop.fc, undefined);
            t.is(sellerNewDrop.ft, undefined);
            t.is(sellerNewDrop.nft, undefined);
            t.assert(sellerNewDrop.simple !== undefined);
        } else {
            t.fail();
        }
    } catch(e) {
        seller == keypom ? t.fail() : t.pass();
    }
}