use anyhow::Result;
use bitcoin::Network;
use clap::{Parser, Subcommand};
use elements::{
    Address, AddressParams,
};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[clap(author, version, about = "A simple CLI tool for Liquid Network")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    #[clap(long, default_value = "https://blockstream.info/liquid/api")]
    api_url: String,

    #[clap(long, default_value = "liquidtestnet")]
    network: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new Liquid address
    GenerateAddress,

    /// Get information about a Liquid asset
    AssetInfo { asset_id: String },
}

#[derive(Serialize, Deserialize, Debug)]
struct AssetInfo {
    asset_id: String,
    name: Option<String>,
    ticker: Option<String>,
    precision: Option<u8>,
    issued_amount: Option<String>,
    issuer_pubkey: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::GenerateAddress => {
            // Generate random keypair
            let secp = bitcoin::secp256k1::Secp256k1::new();
            let (secret_key, secp_public_key) = secp.generate_keypair(&mut rand::thread_rng());
            
            // Convert secp256k1 public key to bitcoin public key
            let bitcoin_public_key = bitcoin::PublicKey::new(secp_public_key);
            
            // Create unconfidential Liquid address - using the correct AddressParams variant directly
            let address = if cli.network == "liquid" {
                Address::p2wpkh(&bitcoin_public_key, None, &AddressParams::LIQUID)
            } else {
                Address::p2wpkh(&bitcoin_public_key, None, &AddressParams::LIQUID_TESTNET)
            };
            
            println!("Generated new Liquid address:");
            println!("Private key (WIF): {}", bitcoin::PrivateKey::new(secret_key, Network::Bitcoin).to_wif());
            println!("Public key: {}", bitcoin_public_key);
            println!("Unconfidential address: {}", address);
            
            Ok(())
        }
        Commands::AssetInfo { asset_id } => {
            // For a real implementation, this would query the Liquid node or an API
            // For demonstration, we'll simulate some data
            println!("Fetching information for asset: {}", asset_id);
            
            if asset_id == "6f0279e9ed041c3d710a9f57d0c02928416460c4b722ae3457a11eec381c526d" {
                // Liquid BTC (L-BTC) - this is a real asset ID
                println!("Asset Name: Liquid Bitcoin (L-BTC)");
                println!("Ticker: L-BTC");
                println!("Precision: 8");
                println!("Type: Pegged Asset");
                println!("Status: Verified");
            } else {
                // Try to query from a Liquid API
                let url = format!("{}/asset/{}", cli.api_url, asset_id);
                match Client::new().get(&url).send() {
                    Ok(response) => {
                        if response.status().is_success() {
                            match response.json::<serde_json::Value>() {
                                Ok(data) => {
                                    println!("Asset information:");
                                    println!("{}", serde_json::to_string_pretty(&data)?);
                                }
                                Err(_) => println!("Failed to parse asset data"),
                            }
                        } else {
                            println!("Asset not found or API error: {}", response.status());
                        }
                    }
                    Err(e) => println!("Error querying API: {}", e),
                }
            }
            
            Ok(())
        }
    }
} 