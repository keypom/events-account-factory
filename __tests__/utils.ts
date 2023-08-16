import { KeyPair } from "near-api-js";
import { TransactionResult } from "near-workspaces";

export async function generateKeyPairs(
    numKeys: number,
  ): Promise<{ keys: KeyPair[]; publicKeys: string[] }> {
    // Generate NumKeys public keys
    let kps: KeyPair[] = [];
    let pks: string[] = [];
    for (let i = 0; i < numKeys; i++) {
      let keyPair = await KeyPair.fromRandom('ed25519');
      kps.push(keyPair);
      pks.push(keyPair.getPublicKey().toString());
    }
    return {
      keys: kps,
      publicKeys: pks
    }
  }

export interface TrialRules {
    amounts: string, 
    contracts: string, 
    floor: string, 
    funder: string, 
    methods: string, 
    repay: string, 
    current_floor: string 
}

export function parseExecutionResults(
    methodName: string,
    receiverId: string,
    transaction: TransactionResult,
    shouldLog: boolean,
    shouldPanic: boolean
) {
    console.log('');
    let logMessages: string[] = [];

    let didPanic = false;
    let panicMessages: string[] = [];

    // Loop through each receipts_outcome in the transaction's result field
    transaction.result.receipts_outcome.forEach((receipt) => { 
        const logs = receipt.outcome.logs;

        if (logs.length > 0) {
        // Turn logs into a string
        let logs = receipt.outcome.logs.reduce((acc, log) => {
            return acc.concat(log).concat('\n');
        }, '');

        logs = logs.substring(0, logs.length - 1);
        logMessages.push(logs);

        } else if (logMessages[logMessages.length - 1] != `\n` && logMessages.length > 0) {
        logMessages.push(`\n`);
        }

        const status = (receipt.outcome.status as any);
        if (status.Failure) {
        let failure = status.Failure.ActionError;
        let str = `Failure for method: ${methodName} Failure: ${JSON.stringify(failure)}\n`

        panicMessages.push(str);
        didPanic = true;
        }
    })


    console.log(`${methodName} -> ${receiverId}. ${logMessages.length} Logs Found. ${panicMessages.length} Panics Found.`);

    if (shouldLog && logMessages.length > 0) {
        let logStr = logMessages.join('\n');
        // Remove the last instance of `\n` from the log string
        logStr = logStr.substring(0, logStr.length - 1);
        console.log(logStr);
    }

    if (panicMessages.length > 0) { 
        console.log("Panics:")
        let panicStr = panicMessages.join('\n');
        // Remove the last instance of `\n` from the panic string
        panicStr = panicStr.substring(0, panicStr.length - 1);
        console.log(panicStr)
    }

    if (shouldPanic && !didPanic) {
        throw new Error(`Expected failure for method: ${methodName}`)
    }

    if (!shouldPanic && didPanic) {
        throw new Error("Panic found when not expected");    
    }
}