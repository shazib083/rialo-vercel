use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use ed25519_dalek::SigningKey as Keypair;
use rialo_cdk::keyring::{mnemonic, Keyring};
use rialo_cdk::rpc::types::Pubkey;
use rialo_cdk::sdk::{Rialo, RialoConfig};
use serde_json::json;
use std::str::FromStr;

const TESTNET_RPC: &str = "https://testnet.rialo.io/";

#[derive(Parser)]
#[command(name = "rialo-tester")]
struct Cli {
    /// Emit machine-readable JSON on stdout instead of human-readable text
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a brand new wallet, entirely in memory. Nothing touches disk.
    WalletCreate,
    /// Check the RLO balance of any pubkey. No secret material needed.
    Balance { pubkey: String },
    /// Request a testnet airdrop to any pubkey. No secret material needed.
    Airdrop { pubkey: String, amount_rlo: f64 },
    /// Transfer RLO. Requires the sender's private key (hex), used only for
    /// this single in-memory call and never written to disk.
    Transfer {
        private_key_hex: String,
        to_pubkey: String,
        amount_rlo: f64,
    },
}

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn from_hex(s: &str) -> Result<Vec<u8>> {
    let s = s.trim();
    if s.len() % 2 != 0 {
        return Err(anyhow!("private key hex must have even length"));
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| anyhow!("invalid hex: {e}")))
        .collect()
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let result = run(&cli).await;

    match result {
        Ok(value) => {
            if cli.json {
                println!("{value}");
            }
            Ok(())
        }
        Err(e) => {
            if cli.json {
                println!("{}", json!({ "ok": false, "error": e.to_string() }));
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

async fn run(cli: &Cli) -> Result<serde_json::Value> {
    match &cli.command {
        Commands::WalletCreate => {
            let mnemonic_phrase = mnemonic::generate_mnemonic_with_strength(128)?;
            let keypair = mnemonic::derive_keypair(&mnemonic_phrase, None, 0)?;
            let private_key_hex = to_hex(&keypair.to_bytes());

            let keyring = Keyring::new(
                "wallet".to_string(),
                keypair,
                Some(mnemonic_phrase.clone()),
                None,
            );
            let pubkey = keyring.pubkey().to_string();

            if !cli.json {
                println!("Wallet created (nothing was written to disk).");
                println!("Pubkey: {pubkey}");
                println!("Private key: {private_key_hex}");
                println!("Mnemonic: {mnemonic_phrase}");
            }
            Ok(json!({
                "ok": true,
                "pubkey": pubkey,
                "private_key": private_key_hex,
                "mnemonic": mnemonic_phrase
            }))
        }
        Commands::Balance { pubkey } => {
            // Throwaway in-memory keyring, just to satisfy RialoConfig's
            // type — balance lookups don't require real ownership.
            let keyring = Keyring::new_empty("scratch");
            let config = RialoConfig::new(TESTNET_RPC.to_string(), keyring);
            let sdk = Rialo::new(config);
            let target = Pubkey::from_str(pubkey).map_err(|e| anyhow!("{e}"))?;
            let balance = sdk.get_account_balance(Some(target)).await?;
            if !cli.json {
                println!("Balance: {balance} RLO");
            }
            Ok(json!({ "ok": true, "pubkey": pubkey, "balance_rlo": balance }))
        }
        Commands::Airdrop { pubkey, amount_rlo } => {
            let keyring = Keyring::new_empty("scratch");
            let config = RialoConfig::new(TESTNET_RPC.to_string(), keyring);
            let sdk = Rialo::new(config);
            let target = Pubkey::from_str(pubkey).map_err(|e| anyhow!("{e}"))?;
            let sig = sdk.airdrop(*amount_rlo, Some(target)).await?;
            if !cli.json {
                println!("Airdrop of {amount_rlo} RLO requested for {pubkey}. Signature: {sig}");
            }
            Ok(json!({ "ok": true, "pubkey": pubkey, "amount_rlo": amount_rlo, "signature": sig.to_string() }))
        }
        Commands::Transfer { private_key_hex, to_pubkey, amount_rlo } => {
            let raw = from_hex(private_key_hex)?;
            let seed: [u8; 32] = raw
                .try_into()
                .map_err(|_| anyhow!("private key must be exactly 32 bytes (64 hex characters)"))?;
            let keypair = Keypair::from_bytes(&seed);
            let keyring = Keyring::new("sender".to_string(), keypair, None, None);

            let config = RialoConfig::new(TESTNET_RPC.to_string(), keyring);
            let sdk = Rialo::new(config);
            let recipient = Pubkey::from_str(to_pubkey).map_err(|e| anyhow!("{e}"))?;
            let sig = sdk.transfer(recipient, *amount_rlo).await?;
            if !cli.json {
                println!("Transferred {amount_rlo} RLO to {to_pubkey}. Signature: {sig}");
            }
            Ok(json!({ "ok": true, "to": to_pubkey, "amount_rlo": amount_rlo, "signature": sig.to_string() }))
        }
    }
}
