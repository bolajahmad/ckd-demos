use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

const IDENTITIES_JSON: &str = include_str!("../identities.json");
const SERVER_SECRET: &str = "my_secret_key_for_demo_purposes_only";

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

/// Looks up an identity by provider and an id (which could be the ID, email, or username).
/// Returns the matching [`Identity`] if found, or `None` if no match exists.
pub async fn lookup_identity(
    provider: &str,
    id: Option<&str>,
) -> Result<Option<Identity>, serde_json::Error> {
    let identities: Vec<Identity> = serde_json::from_str(IDENTITIES_JSON)?;

    let result = identities.into_iter().find(|entry| {
        if entry.provider != provider {
            return false;
        }
        let id_match = id.map_or(false, |v| {
            entry.id == v || entry.email == v || entry.username == v
        });
        id_match
    });

    match result {
        Some(id) => {
            // Generate entropy based on the ID,
            // entropy must always be the same for same ID
            let entropy = generate_entropy(&id).await;
            println!(
                "Generated entropy for identity {}: {:x?}",
                id.canonical_id(),
                entropy
            );
            Ok(Some(id))
        }
        None => {
            println!(
                "No identity found for provider '{}' with the given identifier.",
                provider
            );
            Ok(None)
        }
    }
}

/// Generate randomness for wallet. I use a SERVER_SECRET for demo purposes.
/// Takes an Identity and returns a deterministic "random" string based on the identity and the server secret.
pub async fn generate_entropy(identity: &Identity) -> [u8; 32] {
    let id = identity.canonical_id();

    type HmacSha256 = Hmac<Sha256>;
    let mut mac =
        HmacSha256::new_from_slice(SERVER_SECRET.as_bytes()).expect("Expects key of any size!");

    mac.update(id.as_bytes());
    let result = mac.finalize();
    let bytes = result.into_bytes();

    bytes.into()
}

/// Generates a mnemonic seed phradse based on standards
/// Currently thinking of the Bitcoin's HD wallet approach (BIP-39 + BIP-32),
/// but I intend to research how to make this support CKB (if it doesn't yet)
pub async fn generate_mnemonic(entropy: &[u8; 32]) -> String {
    // Placeholder implementation, in a real implementation you would convert the entropy to a mnemonic
    // using a library like `bip39` or similar.
    format!("mnemonic-for-entropy-{:x?}", entropy)
}
