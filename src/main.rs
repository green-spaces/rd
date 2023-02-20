use clap::{Parser, Subcommand};
use rand::RngCore;
use uuid::Uuid;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Clone, Subcommand)]
enum Commands {
    /// Random bytes, base64 encoded
    #[command(alias = "b")]
    Bytes {
        #[arg(long, short, default_value_t = 32)]
        len: usize,
    },
    /// A random uuid v4
    #[command(alias = "u")]
    Uuid,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Bytes { len } => {
            use base64::{engine::general_purpose, Engine as _};

            let mut bytes = vec![0u8; len];
            let mut rng = rand::thread_rng();
            rng.fill_bytes(&mut bytes);

            let b64 = general_purpose::STANDARD.encode(&bytes);
            println!("{b64}")
        }
        Commands::Uuid => {
            let id = Uuid::new_v4();
            println!("{id}")
        }
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
