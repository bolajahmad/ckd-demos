use std::str::FromStr;

use bip32::{DerivationPath, ExtendedPrivateKey, secp256k1::SecretKey as Bip32SecretKey};
use bip39::Seed;
use ckb_hash::blake2b_256;
use ckb_sdk::{Address, AddressPayload, NetworkType};
use ckb_types::{H256, bytes::Bytes, core::ScriptHashType, packed::Script, prelude::*};
use hmac::{Hmac, Mac};
use p256::{SecretKey as P256SecretKey, ecdsa::SigningKey as P256SigningKey};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha2::Sha512;

// ─── Derivation paths ────────────────────────────────────────────────────────

/// BIP44 / CKB derivation path (secp256k1)
const SECP256K1_PATH: &str = "m/44'/309'/0'/0/0";

// ─── Lock type ───────────────────────────────────────────────────────────────

/// Which CKB lock script the derived key will be used with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockType {
    /// Standard CKB secp256k1 single-sig lock (`lock args` = blake160(pubkey)).
    Secp256k1,
    /// P-256 (secp256r1 / NIST P-256) lock.
    /// `lock args` = blake160(compressed pubkey).
    /// Requires a deployed P-256 lock script on the target network.
    P256,
}

// ─── Key containers ──────────────────────────────────────────────────────────

pub struct Secp256k1Key {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
}

pub struct P256Key {
    pub signing_key: P256SigningKey,
    pub verifying_key: p256::ecdsa::VerifyingKey,
}

// ─── Secp256k1 derivation ────────────────────────────────────────────────────

/// Derive a secp256k1 keypair from a BIP39 seed using the CKB derivation path.
fn derive_secp256k1(seed: Seed) -> Result<Secp256k1Key, String> {
    let path = DerivationPath::from_str(SECP256K1_PATH)
        .map_err(|e| format!("Invalid derivation path: {e}"))?;

    let child = ExtendedPrivateKey::<Bip32SecretKey>::derive_from_path(seed, &path)
        .map_err(|e| format!("BIP32 derivation failed: {e}"))?;

    let secret_key = SecretKey::from_slice(&child.private_key().to_bytes())
        .map_err(|e| format!("Invalid secret key: {e}"))?;

    let secp = Secp256k1::new();
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    Ok(Secp256k1Key {
        private_key: secret_key,
        public_key,
    })
}

/// First 20 bytes of blake2b-256 of a byte slice — the standard CKB `lock args`.
fn blake160(data: &[u8]) -> [u8; 20] {
    let hash = blake2b_256(data);
    let mut out = [0u8; 20];
    out.copy_from_slice(&hash[..20]);
    out
}

/// Build the secp256k1 lock CKB address.
fn secp256k1_address(key: &Secp256k1Key, network: NetworkType) -> Address {
    let payload = AddressPayload::from_pubkey(&key.public_key);
    Address::new(network, payload, true)
}

// ─── P-256 derivation (SLIP-0010) ────────────────────────────────────────────

/// SLIP-0010 master key generation for P-256.
///
/// Produces a 32-byte private scalar from the seed bytes. Full child-key
/// derivation along the path is not yet implemented; this gives a unique,
/// deterministic master key per seed.
fn derive_p256(seed: &[u8]) -> Result<P256Key, String> {
    // SLIP-0010 master key generation: HMAC-SHA512("Nist256p1 seed", seed)
    type HmacSha512 = Hmac<Sha512>;
    let mut mac = HmacSha512::new_from_slice(b"Nist256p1 seed")
        .map_err(|e| format!("HMAC init failed: {e}"))?;
    mac.update(seed);
    let result = mac.finalize().into_bytes();

    // IL = first 32 bytes = private key scalar
    let il = &result[..32];

    let secret_key =
        P256SecretKey::from_slice(il).map_err(|e| format!("Invalid P-256 key: {e}"))?;
    let signing_key = P256SigningKey::from(&secret_key);
    let verifying_key = *signing_key.verifying_key();

    Ok(P256Key {
        signing_key,
        verifying_key,
    })
}

/// Build a full-format CKB address for a P-256 key.
///
/// The `code_hash` used here is the **testnet** P-256 lock script deployed by
/// the CKB ecosystem. Update `P256_LOCK_CODE_HASH` with the mainnet hash when
/// targeting mainnet.
///
/// `lock args` = blake160(compressed 33-byte P-256 public key)
fn p256_address(key: &P256Key, network: NetworkType) -> Address {
    // CKB testnet P-256 (secp256r1) lock code hash.
    // Source: https://github.com/nervosnetwork/ckb-p256k1 or omni-lock docs.
    // TODO: Replace with correct deployed hash for your target network.
    const P256_LOCK_CODE_HASH: &str =
        "9b819793a64463aed77c615d6cb226eea5487ccfc0783043a587254cda2b6f26";

    let compressed_pubkey = key.verifying_key.to_encoded_point(true);
    let args = blake160(compressed_pubkey.as_bytes());

    let code_hash = H256::from_str(P256_LOCK_CODE_HASH).expect("Valid code hash hex");

    let script = Script::new_builder()
        .code_hash(code_hash.pack())
        .hash_type(ckb_types::packed::Byte::from(ScriptHashType::Type as u8))
        .args(Bytes::from(args.to_vec()).pack())
        .build();

    let payload = AddressPayload::from(script);
    Address::new(network, payload, true)
}

// ─── Public API ──────────────────────────────────────────────────────────────

/// Derive a CKB address from a BIP39 seed, using the requested lock type.
pub fn generate_ckb_address(
    seed: Seed,
    lock_type: LockType,
    network: NetworkType,
) -> Result<Address, String> {
    match lock_type {
        LockType::Secp256k1 => {
            let key = derive_secp256k1(seed)?;
            Ok(secp256k1_address(&key, network))
        }
        LockType::P256 => {
            let key = derive_p256(seed.as_bytes())?;
            Ok(p256_address(&key, network))
        }
    }
}
