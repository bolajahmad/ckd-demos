Builder's Journey - Week 5

Timeline: 12-23 June, 2026

## Topics Covered

- Studying available Scripting resources and tooling available
  - Documentation of (most) [CKB Scripts](https://docs.nervos.org/docs/ecosystem-scripts/introduction)
- Read up on CKB Scripts, how they work
  - A Script can be Lock (Determine who can spend/use the Cell) / Type (Controls how the Cell is to be used in TX)
  - Script structure is mainly:
    ```rust
        pub struct Script {
            pub code_hash: H256,    
            pub hash_type: ScriptHashType,  // how the code_hash is intepreted (data / data1 / data2 / type)
            pub args: JsonBytes // arguments passed to script
        }
    ```

- Available Tooling (for Script development)
  - [CKB-STD](https://github.com/nervosnetwork/ckb-std): Several modules that can help to write CKB Scripts using Rust.
  - [CKB Test Tools](https://docs.rs/ckb-testtool/latest/ckb_testtool): Used for testing CKB Scripts
  - [CKB Script Templates](https://github.com/cryptape/ckb-script-templates)
    - Writing a Giveaway Script in CKB