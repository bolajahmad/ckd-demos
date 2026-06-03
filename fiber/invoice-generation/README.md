# Mastering Fiber Network - Generating an Invoice

## Overview

Fiber network can be considered a Layer 2 solution for scaling the CKB blockchain. It is equivalent of what a Lightning network is, to the Bitcoin blockchain. On Fiber, payments are instantaneous and happen between 2 different participants, maybe including various intermediaries.

Ideally, a node on the Fiber network can create a Payment channel with another Node by locking some CKB into a multisig account and this is how the channel gets created. To receive payments across a fnn (fiber node network?), the receiver shares an invoice which contains information about the recipient and amount of payment.

This example now does the following end-to-end:

1. Connect to CKB testnet RPC and read source wallet capacity via CKB indexer RPC (`get_cells_capacity`).
2. Generate a lightning wallet key and derive a testnet CKB address.
3. Check lightning wallet capacity.
4. If capacity is 0, fund it from the source wallet using `ckb-cli wallet transfer`.
5. Generate a Fiber invoice using `fnn-cli invoice new_invoice`.
6. Verify/decode the invoice using `fnn-cli invoice parse_invoice`.
7. Write the decoded invoice JSON to `out.txt`.

No payment channel is opened in this flow.

## Node Interaction 

* Create or provide a funded CKB source wallet key in `./ckb-key`
* Ensure your FNN node is running and reachable at `http://127.0.0.1:8227`
* Generate a lightning wallet address locally
* Fund the lightning wallet only when its balance is zero
* Generate and parse invoice through FNN RPC

## Run

```bash
cargo run
```

## Output

* Decoded invoice file: `out.txt`
* Generated lightning wallet key file: `lightning-wallet.key`