use bip39::{Language, Mnemonic, Seed};
use dotenv::dotenv;
use std::env;

use hex;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

mod ckb;
mod session;

pub use ckb::LockType;
pub use session::Session;

const IDENTITIES_JSON: &str = include_str!("../identities.json");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub provider: String,
    pub id: String,
    pub email: String,
    pub username: String,
}

impl Identity {
    pub fn canonical_id(&self) -> String {
        format!("{}:{}", self.provider, self.id)
    }
}

// ─── Identity ────────────────────────────────────────────────────────────────

/// Look up an identity by provider + identifier (id, email, or username).
/// If found, derive entropy and persist a session so subsequent commands
/// (like `derive`) can use the same keys without re-authenticating.
pub async fn login(
    provider: &str,
    id: Option<&str>,
) -> Result<Option<Identity>, serde_json::Error> {
    let identities: Vec<Identity> = serde_json::from_str(IDENTITIES_JSON)?;

    let found = identities.into_iter().find(|entry| {
        if entry.provider != provider {
            return false;
        }
        id.map_or(false, |v| {
            entry.id == v || entry.email == v || entry.username == v
        })
    });

    if let Some(ref identity) = found {
        let entropy = generate_entropy(identity).await;
        let session = Session {
            canonical_id: identity.canonical_id(),
            entropy_hex: hex::encode(entropy),
        };
        session.save().expect("Failed to persist session");
        println!("Session saved for {}", identity.canonical_id());
    }

    Ok(found)
}

// ─── Wallet derivation ───────────────────────────────────────────────────────

/// Derive a CKB address for the currently logged-in session.
///
/// Reads the persisted session written by `login`, reconstructs the seed
/// deterministically, and returns the CKB address for the requested lock type
/// and network.
pub fn derive_ckb(
    lock_type: LockType,
    network: ckb_sdk::NetworkType,
) -> Result<ckb_sdk::Address, String> {
    let session = Session::load()?;

    let entropy_bytes = hex::decode(&session.entropy_hex)
        .map_err(|e| format!("Invalid entropy in session: {e}"))?;

    let entropy_arr: [u8; 32] = entropy_bytes
        .try_into()
        .map_err(|_| "Entropy is not 32 bytes".to_string())?;

    let seed = seed_from_entropy(&entropy_arr);

    let address = ckb::generate_ckb_address(seed, lock_type, network)?;
    Ok(address)
}

// ─── Entropy + seed ──────────────────────────────────────────────────────────

/// Derive deterministic 32-byte entropy for an identity via HMAC-SHA256.
///
/// `entropy = HMAC_SHA256(SERVER_SECRET, canonical_identity)`
pub async fn generate_entropy(identity: &Identity) -> [u8; 32] {
    dotenv().ok();

    let server_secret = env::var("SERVER_SECRET").expect("SERVER_SECRET must be set");

    type HmacSha256 = Hmac<Sha256>;
    let mut mac =
        HmacSha256::new_from_slice(server_secret.as_bytes()).expect("HMAC accepts any key size");
    mac.update(identity.canonical_id().as_bytes());

    mac.finalize().into_bytes().into()
}

/// Convert 32-byte entropy into a BIP39 mnemonic and then into a BIP39 seed.
///
/// The entropy is the *source* of the mnemonic (not the passphrase), ensuring
/// the same entropy always produces the same mnemonic and therefore the same
/// derived keys.
fn seed_from_entropy(entropy: &[u8; 32]) -> Seed {
    dotenv().ok();
    // `from_entropy` requires 16 / 20 / 24 / 28 / 32 bytes — 32 is valid.
    let mnemonic = Mnemonic::from_entropy(entropy, Language::English)
        .expect("32-byte entropy is always valid for BIP39");

    let secret = env::var("MNEMONIC_SECRET").expect("Mnemonic secret in vars");
    Seed::new(&mnemonic, &secret)
}
