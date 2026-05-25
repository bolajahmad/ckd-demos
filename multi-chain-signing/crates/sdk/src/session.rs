use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

/// Persisted session state written after a successful login.
/// Stored at `~/.config/multi-chain-signing/session.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Canonical identity string, e.g. `"google:alice@gmail.com"`.
    pub canonical_id: String,
    /// HMAC-derived 32-byte entropy encoded as lowercase hex.
    pub entropy_hex: String,
}

fn session_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let mut path = PathBuf::from(home);
    path.push(".config");
    path.push("multi-chain-signing");
    path.push("session.json");
    path
}

impl Session {
    pub fn save(&self) -> std::io::Result<()> {
        let path = session_path();
        fs::create_dir_all(path.parent().expect("Path has parent"))?;
        fs::write(&path, serde_json::to_string_pretty(self).expect("Serializable"))?;
        Ok(())
    }

    pub fn load() -> Result<Self, String> {
        let path = session_path();
        let data = fs::read_to_string(&path)
            .map_err(|_| "No active session. Run `login` first.".to_string())?;
        serde_json::from_str(&data).map_err(|e| format!("Corrupt session file: {e}"))
    }
}
