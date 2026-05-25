# Builder's Journey - Week 1

Timeline: 18-25 of May, 2026

## Topics Covered

- Studying existing (wallet) tooling projects available on CKB
  - Joy ID: A passkey-based wallet generation library for JS/TS development
    - Studied how it works under the hood and building a replica logic locally 
    - Read full docs on Joy ID and what's misising at the moment
    - Is this project still being maintained?
  - CKB Auth: A Rust-based smart contract script that can be used on-chain to verify the auth mechanism of a CKB address
    - Exploring how it works (and can be extended).
    - Exploring the codebase of CKb auth
  - Building a demo project to generate CKB address using various lock scripts
    - Helps with account abstraction implementations

## Open Ended Questions

- Open-source libraries like ckb-auth, ckb-tools and JoyID are not being maontained currently, why?
- JoyID could be done with development, but some of their docs suggest ongoing development, but Github repo remain inactive.
- ckb-auth can still grow with addition of other cryptigraphic signature schemes, what is the process of making these additions?

Not a lot of new docs as I spent time reading available implementations and tooling around cross-chain wallet derivation on CKB.