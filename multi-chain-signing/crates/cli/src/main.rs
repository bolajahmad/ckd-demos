use clap::{Parser, Subcommand, ValueEnum};
use sdk::LockType;

#[derive(Parser)]
#[command(name = "cli", about = "Multi-chain signing CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Authenticate with a social identity and open a session.
    /// Keys are stored locally so `derive` and future commands can use them.
    Login {
        /// The identity provider (e.g. "google", "github")
        #[arg(long)]
        provider: String,

        /// Match by provider-scoped user ID, email, or username
        #[arg(long, group = "identifier")]
        id: Option<String>,
    },

    /// Derive a CKB wallet address from the active session.
    Derive {
        /// Chain to derive for (only "ckb" is supported for now)
        #[arg(long, default_value = "ckb")]
        chain: String,

        /// Which CKB lock script to use for the address
        #[arg(long, default_value = "secp256k1")]
        lock: LockArg,

        /// Network to generate the address for
        #[arg(long, default_value = "testnet")]
        network: NetworkArg,
    },
}

#[derive(Clone, ValueEnum)]
enum LockArg {
    Secp256k1,
    P256,
}

#[derive(Clone, ValueEnum)]
enum NetworkArg {
    Mainnet,
    Testnet,
}

impl From<LockArg> for LockType {
    fn from(l: LockArg) -> Self {
        match l {
            LockArg::Secp256k1 => LockType::Secp256k1,
            LockArg::P256 => LockType::P256,
        }
    }
}

impl From<NetworkArg> for ckb_sdk::NetworkType {
    fn from(n: NetworkArg) -> Self {
        match n {
            NetworkArg::Mainnet => ckb_sdk::NetworkType::Mainnet,
            NetworkArg::Testnet => ckb_sdk::NetworkType::Testnet,
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Login { provider, id } => {
            if id.is_none() {
                eprintln!("error: --id is required");
                std::process::exit(1);
            }

            match sdk::login(&provider, id.as_deref()).await {
                Ok(Some(identity)) => {
                    println!("Logged in:");
                    println!("  provider : {}", identity.provider);
                    println!("  id       : {}", identity.id);
                    println!("  email    : {}", identity.email);
                    println!("  username : {}", identity.username);
                }
                Ok(None) => {
                    eprintln!(
                        "No identity found for provider '{}' with the given identifier.",
                        provider
                    );
                    std::process::exit(1);
                }
                Err(e) => {
                    eprintln!("Failed to read identities: {e}");
                    std::process::exit(1);
                }
            }
        }

        Commands::Derive { chain, lock, network } => {
            if chain != "ckb" {
                eprintln!("Only --chain ckb is supported at this time.");
                std::process::exit(1);
            }

            match sdk::derive_ckb(lock.into(), network.into()) {
                Ok(address) => {
                    println!("CKB address : {address}");
                }
                Err(e) => {
                    eprintln!("Derivation failed: {e}");
                    std::process::exit(1);
                }
            }
        }
    }
}
