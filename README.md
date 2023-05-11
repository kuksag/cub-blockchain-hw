# Blockchain homework no. 2
# Fixes in stablecoin based on the [example](https://etherscan.io/address/0xf7790914dc335b20aa19d7c9c9171e14e278a134#code)

Addressed flaws:
* Re-entrancy: There doesn't seem to be any protection against re-entrancy attacks. The contract performs state-changing operations after external function calls in the validateTransfer, rejectTransfer, validateApprove, and rejectApprove functions, making it susceptible to re-entrancy attacks. Always update state variables before making external calls to avoid potential re-entrancy attacks.

* Unchecked Underflow: The contract uses the unchecked keyword to perform subtraction on balances in the validateTransfer and rejectTransfer functions. If the _transferRequest.value is greater than the _engagedAmount[_transferRequest.from], this could lead to an underflow, leaving the _engagedAmount[_transferRequest.from] with a very high balance. The unchecked keyword is used again in _availableBalance which could lead to similar issues.

* Race Condition / Front-Running: There's a potential for race conditions. For example, a user might be able to call approve or transferFrom functions multiple times before the transactions are mined, which could result in incorrect balances. This is related to the broader problem of Ethereum miners being able to decide the order of transactions.

* Lack of Input Validation: The code doesn't check if the _from and _to addresses are the same in the approve and transferFrom functions. Transfers to the same address should be prevented, as they're likely to be mistakes.

* Potential Denial of Service: The contract relies on onlyWhitelisted modifier in multiple functions. If an address was removed from the whitelist while a function call was still pending, it could cause the function to fail.

* Lack of Event Logs: The contract doesn't emit event logs in some important functions like burn, mint, and recall. Event logs are useful for tracking contract activities on the blockchain.

* Overflow Risk: There's a potential for overflow in the approve function where the allowance is incremented. Although the chances are low, it's good practice to guard against this possibility.

