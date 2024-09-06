import { sendTransaction } from "./utils";
import nacl from "tweetnacl";
import bs58 from "bs58"; // Library for decoding base58
import { KeyPair, utils } from "near-api-js";
import { encryptAndStoreData } from "./encryptionUtils";

export const addTickets = async ({
  signerAccount,
  factoryAccountId,
  dropId,
  attendeeInfo,
}: {
  signerAccount: any;
  factoryAccountId: string;
  dropId: string;
  attendeeInfo: Array<Record<string, string>>;
}) => {
  // Map to store the KeyPair -> Attendee Info relationship
  const keyPairMap: Map<string, Record<string, string>> = new Map();
  // Loop through 50 at a time and add the tickets
  for (let i = 0; i < attendeeInfo.length; i += 50) {
    let keyData: Array<Record<string, any>> = [];

    for (let j = i; j < i + 50; j++) {
      const curInfo = attendeeInfo[j];
      if (curInfo) {
        const keyPair = KeyPair.fromRandom("ed25519");

        // Encrypt the attendee's sensitive info using the defined helper function
        const attendeeMetadata = JSON.stringify(curInfo); // Sensitive data

        // Use the helper function to encrypt the metadata
        const encryptedMetadata = encryptAndStoreData(
          keyPair.toString(),
          attendeeMetadata,
        ); // This will return the JSON including encrypted data

        // Push key data with the encrypted metadata
        keyData.push({
          public_key: keyPair.getPublicKey().toString(),
          metadata: encryptedMetadata, // Already stringified JSON with encrypted data
        });

        // Map the keypair's public key to the corresponding attendee info
        keyPairMap.set(keyPair.toString(), curInfo);
      }
    }

    // Send the transaction in batches of 50 tickets
    await sendTransaction({
      signerAccount,
      receiverId: factoryAccountId,
      methodName: "add_tickets",
      args: {
        drop_id: dropId,
        key_data: keyData,
      },
      deposit: "0",
      gas: "300000000000000", // Set gas limit
    });
  }

  // Return the map of key pairs to attendee info
  return keyPairMap;
};
