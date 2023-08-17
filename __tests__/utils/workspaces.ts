import { BN } from "bn.js";
import { formatNearAmount } from "near-api-js/lib/utils/format";
import {
    AccountBalance,
    NearAccount,
    TransactionResult
} from "near-workspaces";
import { LARGE_GAS } from "./keypom";

export async function delay(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export async function functionCall({
  signer,
  receiver,
  methodName,
  args,
  attachedDeposit,
  gas,
  shouldLog = true,
  shouldPanic = false,
}: {
  signer: NearAccount;
  receiver: NearAccount;
  methodName: string;
  args: any;
  attachedDeposit?: string;
  gas?: string;
  shouldLog?: boolean;
  shouldPanic?: boolean;
}) {
  let rawValue = await signer.callRaw(receiver, methodName, args, {
    gas: gas || LARGE_GAS,
    attachedDeposit: attachedDeposit || "0",
  });
  parseExecutionResults(
    methodName,
    receiver.accountId,
    rawValue,
    shouldLog,
    shouldPanic
  );

  if (rawValue.SuccessValue) {
    return atob(rawValue.SuccessValue);
  } else {
    return rawValue.Failure?.error_message;
  }
}

export function parseExecutionResults(
  methodName: string,
  receiverId: string,
  transaction: TransactionResult,
  shouldLog: boolean,
  shouldPanic: boolean
) {
  console.log("");
  let logMessages: string[] = [];

  let didPanic = false;
  let panicMessages: string[] = [];

  // Loop through each receipts_outcome in the transaction's result field
  transaction.result.receipts_outcome.forEach((receipt) => {
    const logs = receipt.outcome.logs;

    if (logs.length > 0) {
      // Turn logs into a string
      let logs = receipt.outcome.logs.reduce((acc, log) => {
        return acc.concat(log).concat("\n");
      }, "");

      logs = logs.substring(0, logs.length - 1);
      logMessages.push(logs);
    } else if (
      logMessages[logMessages.length - 1] != `\n` &&
      logMessages.length > 0
    ) {
      logMessages.push(`\n`);
    }

    const status = receipt.outcome.status as any;
    if (status.Failure) {
      let failure = status.Failure.ActionError;
      let str = `Failure for method: ${methodName} Failure: ${JSON.stringify(
        failure
      )}\n`;

      panicMessages.push(str);
      didPanic = true;
    }
  });

  console.log(
    `${methodName} -> ${receiverId}. ${logMessages.length} Logs Found. ${panicMessages.length} Panics Found.`
  );

  if (shouldLog && logMessages.length > 0) {
    let logStr = logMessages.join("\n");
    // Remove the last instance of `\n` from the log string
    logStr = logStr.substring(0, logStr.length - 1);
    console.log(logStr);
  }

  if (panicMessages.length > 0) {
    console.log("Panics:");
    let panicStr = panicMessages.join("\n");
    // Remove the last instance of `\n` from the panic string
    panicStr = panicStr.substring(0, panicStr.length - 1);
    console.log(panicStr);
  }

  if (shouldPanic && !didPanic) {
    throw new Error(`Expected failure for method: ${methodName}`);
  }

  if (!shouldPanic && didPanic) {
    throw new Error("Panic found when not expected");
  }
}

export const displayBalances = (
  initialBalances: AccountBalance,
  finalBalances: AccountBalance
) => {
  const initialBalancesNear = {
    available: formatNearAmount(initialBalances.available.toString()),
    staked: formatNearAmount(initialBalances.staked.toString()),
    stateStaked: formatNearAmount(initialBalances.stateStaked.toString()),
    total: formatNearAmount(initialBalances.total.toString()),
  };

  const finalBalancesNear = {
    available: formatNearAmount(finalBalances.available.toString()),
    staked: formatNearAmount(finalBalances.staked.toString()),
    stateStaked: formatNearAmount(finalBalances.stateStaked.toString()),
    total: formatNearAmount(finalBalances.total.toString()),
  };

  let isMoreState = false;
  if (
    new BN(initialBalances.stateStaked.toString()).lt(
      new BN(finalBalances.stateStaked.toString())
    )
  ) {
    let temp = initialBalances.stateStaked;
    initialBalances.stateStaked = finalBalances.stateStaked;
    finalBalances.stateStaked = temp;
    isMoreState = true;
  }

  console.log(
    `Available: ${initialBalancesNear.available.toString()} -> ${finalBalancesNear.available.toString()}`
  );
  console.log(
    `Staked: ${initialBalancesNear.staked.toString()} -> ${finalBalancesNear.staked.toString()}`
  );
  console.log(
    `State Staked: ${initialBalancesNear.stateStaked.toString()} -> ${finalBalancesNear.stateStaked.toString()}`
  );
  console.log(
    `Total: ${initialBalancesNear.total.toString()} -> ${finalBalancesNear.total.toString()}`
  );
  console.log(``);
  console.log(`NET:`);
  console.log(
    `Available: ${formatNearAmount(
      new BN(finalBalances.available.toString())
        .sub(new BN(initialBalances.available.toString()))
        .toString()
    )}`
  );
  console.log(
    `Staked: ${formatNearAmount(
      new BN(finalBalances.staked.toString())
        .sub(new BN(initialBalances.staked.toString()))
        .toString()
    )}`
  );
  console.log(
    `State Staked ${isMoreState ? "(more)" : "(less)"}: ${formatNearAmount(
      new BN(initialBalances.stateStaked.toString())
        .sub(new BN(finalBalances.stateStaked.toString()))
        .toString()
    )}`
  );
  console.log(
    `Total: ${formatNearAmount(
      new BN(finalBalances.total.toString())
        .sub(new BN(initialBalances.total.toString()))
        .toString()
    )}`
  );
};
