# Builder's Journey - Week 1

Timeline: 11-17 of May, 2026

## Topics Covered

- Dev environment setup
  - Wallet connector SDKs - Useful for connecting to running chains, interacting with smart contracts, building dapps on CKB chains.
    Wallets exist in various SDK languages allowing for greater range and versatility.
    - CKB SDK Rust
    - Common Chain Connector (CCC) for Typescript and Javascript.
  - CLI Tools - Installed some CLI tools for interacting with running CKB nodes (or environments). Also available in various languages.
    - CKB CLI - Rust based CLI crate, usually downloaded alongside the node executable
    - [offckb/cli](https://docs.nervos.org/docs/getting-started/quick-start) - A TS based CLI tool that provides similar options to the CKB CLI.
  - Install and run [CKB node for various environments](https://docs.nervos.org/docs/node/node-overview)
  
- Learned the basic of Nervos CKB from the [CKB Learners guide](https://docs.nervos.org/docs/ckb-fundamentals/nervos-blockchain)
  - Similarities of [CKB VS Bitcoin](https://docs.nervos.org/docs/ckb-fundamentals/ckb-vs-btc)
    - Compared their notions of UTXOs (vs Cells with multi-lock)
    - Deep dive into the differences between the mining approaches
    - Transaction creation, signing and broadcast structure and differences
    - Important differences and upgrades on CKB, compared to Bitcoin, especially around [NC (vs NC-Max) Consensus](https://eprint.iacr.org/2020/1101.pdf)

- How CKB works in summary, following the [How CKB works](https://docs.nervos.org/docs/getting-started/how-ckb-works) guide
- CKB Academy courses
  - Completed lesson 1 -  [CKB Theoretical Knowledge](https://academy.ckb.dev/courses/basic-theory)
    <img src="./images/ckb-theoretical-knowledge.png" />
  - Completed lesson 2 - [CKB Practical Operations](https://academy.ckb.dev/courses/basic-operation)
    <img src="./images/ckb-practical-ops.png" />
  - Completed lesson 3 - [CKB NFT Standards](https://academy.ckb.dev/courses/nft-getting-started)
    <img src="./images/ckn-nfts.png" />

## Personal Assignment

- Implement a CKB multi-chain custodial wallet derivation + signing engine from social identity (for CKB and Bitcoin)
- Research HD wallets generation in CKB chains