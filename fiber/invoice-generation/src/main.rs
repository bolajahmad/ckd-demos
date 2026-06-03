use ckb_sdk::rpc::ckb_indexer::SearchKey;
use ckb_sdk::CkbRpcClient;
use rand::RngCore;
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

const CKB_RPC_URL: &str = "https://testnet.ckb.dev";
const FNN_RPC_URL: &str = "http://127.0.0.1:8227";

const SOURCE_LOCK_ARG: &str = "0x40916e5efd56d0af37c095b226cabc30ed03e093";
const SOURCE_PRIVKEY_PATH: &str = "./ckb-key";

const LIGHTNING_KEY_PATH: &str = "./lightning-wallet.key";
const OUT_FILE: &str = "out.txt";

const TRANSFER_CAPACITY_CKB: &str = "200";
const INVOICE_AMOUNT_SHANNONS: &str = "1000";

const SECP256K1_CODE_HASH: &str =
    "0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8";

fn main() -> Result<(), Box<dyn Error>> {
    let ckb_client = CkbRpcClient::new(CKB_RPC_URL);

    let source_capacity = get_capacity_by_lock_arg(&ckb_client, SOURCE_LOCK_ARG)?;
    println!("Source wallet capacity: {source_capacity} shannons");

    let (lightning_address, lightning_lock_arg) = ensure_lightning_wallet()?;
    println!("Lightning wallet address: {lightning_address}");

    let lightning_capacity = get_capacity_by_lock_arg(&ckb_client, &lightning_lock_arg)?;
    println!("Lightning wallet capacity: {lightning_capacity} shannons");

    if lightning_capacity == 0 {
        println!("Lightning wallet is empty. Funding from source wallet...");
        fund_lightning_wallet(&lightning_address)?;
    } else {
        println!("Lightning wallet already funded, skipping transfer.");
    }

    let invoice_address = generate_invoice()?;
    println!("Generated invoice: {invoice_address}");

    let decoded_invoice = parse_invoice(&invoice_address)?;
    fs::write(OUT_FILE, serde_json::to_string_pretty(&decoded_invoice)?)?;
    println!("Decoded invoice written to {OUT_FILE}");

    Ok(())
}

fn ensure_lightning_wallet() -> Result<(String, String), Box<dyn Error>> {
    if !Path::new(LIGHTNING_KEY_PATH).exists() {
        let mut key = [0u8; 32];
        rand::rngs::OsRng.fill_bytes(&mut key);
        fs::write(LIGHTNING_KEY_PATH, hex::encode(key))?;
    }

    let output = run_command(
        "ckb-cli",
        &[
            "--url",
            CKB_RPC_URL,
            "util",
            "key-info",
            "--privkey-path",
            LIGHTNING_KEY_PATH,
            "--local-only",
            "--output-format",
            "json",
        ],
    )?;

    let parsed: Value = serde_json::from_str(&output)?;
    let address = parsed
        .get("address")
        .and_then(|v| v.get("testnet"))
        .and_then(Value::as_str)
        .ok_or("missing testnet address from ckb-cli key-info")?
        .to_string();
    let lock_arg = parsed
        .get("lock_arg")
        .and_then(Value::as_str)
        .ok_or("missing lock_arg from ckb-cli key-info")?
        .to_string();

    Ok((address, lock_arg))
}

fn get_capacity_by_lock_arg(
    ckb_client: &CkbRpcClient,
    lock_arg: &str,
) -> Result<u64, Box<dyn Error>> {
    let search_key: SearchKey = serde_json::from_value(serde_json::json!({
        "script": {
            "code_hash": SECP256K1_CODE_HASH,
            "hash_type": "type",
            "args": lock_arg,
        },
        "script_type": "lock",
        "script_search_mode": "exact",
    }))?;

    let result = ckb_client.get_cells_capacity(search_key)?;
    let capacity_hex = result
        .ok_or("get_cells_capacity returned no result")?
        .capacity
        .to_string();

    Ok(parse_hex_u64(&capacity_hex)?)
}

fn parse_hex_u64(value: &str) -> Result<u64, Box<dyn Error>> {
    let clean = value.trim_start_matches("0x");
    Ok(u64::from_str_radix(clean, 16)?)
}

fn fund_lightning_wallet(to_address: &str) -> Result<(), Box<dyn Error>> {
    let transfer_output = run_command(
        "ckb-cli",
        &[
            "--url",
            CKB_RPC_URL,
            "wallet",
            "transfer",
            "--to-address",
            to_address,
            "--capacity",
            TRANSFER_CAPACITY_CKB,
            "--fee-rate",
            "1200",
            "--privkey-path",
            SOURCE_PRIVKEY_PATH,
            "--output-format",
            "json",
        ],
    )?;

    println!("Transfer submitted: {transfer_output}");
    Ok(())
}

fn generate_invoice() -> Result<String, Box<dyn Error>> {
    let output = run_command(
        "fnn-cli",
        &[
            "-u",
            FNN_RPC_URL,
            "--raw-data",
            "-o",
            "json",
            "invoice",
            "new_invoice",
            "--amount",
            INVOICE_AMOUNT_SHANNONS,
            "--currency",
            "Fibt",
            "--description",
            "invoice-generation-demo",
        ],
    )?;

    let parsed: Value = serde_json::from_str(&output)?;
    let invoice = parsed
        .get("invoice_address")
        .and_then(Value::as_str)
        .ok_or("missing invoice_address in new_invoice response")?
        .to_string();
    Ok(invoice)
}

fn parse_invoice(invoice_address: &str) -> Result<Value, Box<dyn Error>> {
    let output = run_command(
        "fnn-cli",
        &[
            "-u",
            FNN_RPC_URL,
            "--raw-data",
            "-o",
            "json",
            "invoice",
            "parse_invoice",
            "--invoice",
            invoice_address,
        ],
    )?;

    Ok(serde_json::from_str(&output)?)
}

fn run_command(program: &str, args: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = Command::new(program).args(args).output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("{program} failed: {stderr}").into());
    }

    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}
