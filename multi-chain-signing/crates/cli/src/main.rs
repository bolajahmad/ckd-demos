use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cli", about = "Multi-chain signing CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check whether an identity exists for the given provider and identifier.
    Login {
        /// The identity provider (e.g. "google", "github")
        #[arg(long)]
        provider: String,

        /// Match by provider-scoped user ID
        #[arg(long, group = "identifier")]
        id: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Login { provider, id } => {
            if id.is_none() {
                eprintln!("error: at least one of --id, --email, or --username must be provided");
                std::process::exit(1);
            }

            match sdk::lookup_identity(&provider, id.as_deref()).await {
                Ok(Some(identity)) => {
                    println!("Identity found:");
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
    }
}
