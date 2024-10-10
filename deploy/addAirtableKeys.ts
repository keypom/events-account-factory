import fs from "fs";
import path from "path";
import { addTickets } from "./addTickets";

export const addAirtableKeys = async ({
  mailingList,
  signerAccount,
  factoryAccountId,
  dropId,
}: {
  mailingList: Array<Record<string, string>>;
  signerAccount: any;
  factoryAccountId: string;
  dropId: string;
}) => {
  // STEP 1: Convert mailing list to an array of objects with only `name` and `email` fields
  const attendeeInfo = mailingList.map((entry: any) => ({
    name: entry.name,
    email: entry.email,
  }));

  // STEP 2: Add tickets and get the keyPair map
  const keyPairMap = await addTickets({
    signerAccount,
    factoryAccountId,
    dropId,
    attendeeInfo,
  });

  // Prepare to capture failed tickets
  const failedTickets: Array<Record<string, string>> = [];

  // STEP 4: Modify the original mailing list by adding `secretKey` field
  const updatedMailingList = mailingList.map((entry: any) => {
    // Find the keyPair in the keyPairMap that matches the entry
    let ticketData = "NO-DATA-FOUND";

    // Loop through the keyPairMap to find the correct key for this entry
    keyPairMap.forEach((value, key) => {
      if (value.email === entry.email) {
        ticketData = key; // Set the secretKey if emails match
      }
    });

    // If no ticketData was found, log it as a failed ticket
    if (ticketData === "NO-DATA-FOUND") {
      console.error(`Failed to find ticket data for: ${entry.email}`);
      failedTickets.push(entry); // Add to the list of failed tickets
    }

    // Return the updated entry with the added secretKey
    return {
      ...entry,
      merge_fields: {
        ...entry.merge_fields,
        KEY: ticketData,
      },
    };
  });

  return {
    keyPairMap,
    updatedMailingList,
    failedTickets,
  };
};
