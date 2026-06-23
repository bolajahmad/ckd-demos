use ckb_testtool::{
    ckb_crypto::secp::Privkey,
    ckb_error::Error,
    ckb_hash::{blake2b_256, new_blake2b},
    ckb_types::{
        H256,
        bytes::Bytes,
        core::{Cycle, ScriptHashType, TransactionView},
        packed::{self, Byte32, CellDep, OutPoint, Script, WitnessArgs},
        prelude::*,
    },
    context::Context,
};
use ckb_system_scripts::BUNDLED_CELL;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

#[cfg(test)]
mod tests;

// The exact same Loader code from capsule's template, except that
// now we use MODE as the environment variable
const TEST_ENV_VAR: &str = "MODE";

pub enum TestEnv {
    Debug,
    Release,
}

impl FromStr for TestEnv {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "debug" => Ok(TestEnv::Debug),
            "release" => Ok(TestEnv::Release),
            _ => Err("no match"),
        }
    }
}

pub struct Loader(PathBuf);

impl Default for Loader {
    fn default() -> Self {
        let test_env = match env::var(TEST_ENV_VAR) {
            Ok(val) => val.parse().expect("test env"),
            Err(_) => TestEnv::Release,
        };
        Self::with_test_env(test_env)
    }
}

impl Loader {
    fn with_test_env(env: TestEnv) -> Self {
        let load_prefix = match env {
            TestEnv::Debug => "debug",
            TestEnv::Release => "release",
        };
        let mut base_path = match env::var("TOP") {
            Ok(val) => {
                let mut base_path: PathBuf = val.into();
                base_path.push("build");
                base_path
            }
            Err(_) => {
                let mut base_path = PathBuf::new();
                // cargo may use a different cwd when running tests, for example:
                // when running debug in vscode, it will use workspace root as cwd by default,
                // when running test by `cargo test`, it will use tests directory as cwd,
                // so we need a fallback path
                base_path.push("build");
                if !base_path.exists() {
                    base_path.pop();
                    base_path.push("..");
                    base_path.push("build");
                }
                base_path
            }
        };

        base_path.push(load_prefix);
        Loader(base_path)
    }

    pub fn load_binary(&self, name: &str) -> Bytes {
        let mut path = self.0.clone();
        path.push(name);
        let result = fs::read(&path);
        if result.is_err() {
            panic!("Binary {path:?} is missing!");
        }
        result.unwrap().into()
    }
}

// This helper method runs Context::verify_tx, but in case error happens,
// it also dumps current transaction to failed_txs folder.
pub fn verify_and_dump_failed_tx(
    context: &Context,
    tx: &TransactionView,
    max_cycles: u64,
) -> Result<Cycle, Error> {
    let result = context.verify_tx(tx, max_cycles);
    if result.is_err() {
        let mut path = env::current_dir().expect("current dir");
        path.push("failed_txs");
        std::fs::create_dir_all(&path).expect("create failed_txs dir");
        let mock_tx = context.dump_tx(tx).expect("dump failed tx");
        let json = serde_json::to_string_pretty(&mock_tx).expect("json");
        path.push(format!("0x{:x}.json", tx.hash()));
        println!("Failed tx written to {path:?}");
        std::fs::write(path, json).expect("write");
    }
    result
}

pub struct SighashLockDeps {
    pub lock_out_point: OutPoint,
    pub secp_data_dep: CellDep,
}

pub fn deploy_sighash_lock_deps(context: &mut Context) -> SighashLockDeps {
    let secp_data_bin = BUNDLED_CELL
        .get("specs/cells/secp256k1_data")
        .expect("secp256k1_data binary");
    let sighash_bin = BUNDLED_CELL
        .get("specs/cells/secp256k1_blake160_sighash_all")
        .expect("sighash_all binary");

    let secp_data_out_point = context.deploy_cell(secp_data_bin.to_vec().into());
    let lock_out_point = context.deploy_cell(sighash_bin.to_vec().into());

    let secp_data_dep = CellDep::new_builder().out_point(secp_data_out_point).build();

    SighashLockDeps {
        lock_out_point,
        secp_data_dep,
    }
}

pub fn blake160(data: &[u8]) -> [u8; 20] {
    let mut buf = [0u8; 20];
    let hash = blake2b_256(data);
    buf.copy_from_slice(&hash[..20]);
    buf
}

pub fn build_sighash_lock_script(
    context: &mut Context,
    lock_out_point: &OutPoint,
    pubkey_hash: &[u8; 20],
) -> Script {
    context
        .build_script_with_hash_type(lock_out_point, ScriptHashType::Data, Default::default())
        .expect("sighash lock script")
        .as_builder()
        .args(pubkey_hash.to_vec().pack())
        .build()
}

pub fn script_hash(script: &Script) -> Byte32 {
    script.calc_script_hash()
}

pub fn sign_tx_sighash(tx: TransactionView, key: &Privkey) -> TransactionView {
    const SIGNATURE_SIZE: usize = 65;

    let witnesses_len = core::cmp::max(tx.witnesses().len(), tx.inputs().len());
    let tx_hash = tx.hash();
    let mut signed_witnesses: Vec<packed::Bytes> = Vec::new();
    let mut blake2b = new_blake2b();
    let mut message = [0u8; 32];
    blake2b.update(&tx_hash.raw_data());

    let witness = WitnessArgs::default();
    let zero_lock: Bytes = {
        let mut buf = Vec::new();
        buf.resize(SIGNATURE_SIZE, 0);
        buf.into()
    };
    let witness_for_digest = witness
        .clone()
        .as_builder()
        .lock(Some(zero_lock).pack())
        .build();
    let witness_len = witness_for_digest.as_bytes().len() as u64;
    blake2b.update(&witness_len.to_le_bytes());
    blake2b.update(&witness_for_digest.as_bytes());

    (1..witnesses_len).for_each(|n| {
        let witness = tx.witnesses().get(n).unwrap_or_default();
        let witness_len = witness.raw_data().len() as u64;
        blake2b.update(&witness_len.to_le_bytes());
        blake2b.update(&witness.raw_data());
    });

    blake2b.finalize(&mut message);
    let message = H256::from(message);
    let sig = key.sign_recoverable(&message).expect("sign");
    signed_witnesses.push(
        witness
            .as_builder()
            .lock(Some(Bytes::from(sig.serialize())).pack())
            .build()
            .as_bytes()
            .pack(),
    );
    for i in 1..witnesses_len {
        signed_witnesses.push(tx.witnesses().get(i).unwrap_or_default());
    }

    tx.as_advanced_builder()
        .set_witnesses(signed_witnesses)
        .build()
}
