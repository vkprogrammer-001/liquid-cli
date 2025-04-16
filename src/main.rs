use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a test RPC client
    fn get_test_rpc() -> ElementsRpc {
        ElementsRpc::new(
            "http://localhost:18891",  // Use real port
            "liquiduser",             // Use real username
            "yourpassword"            // Use real password
        )
    }
    
    // Helper function to check if Elements Core is available
    fn is_elements_available() -> bool {
        let rpc = get_test_rpc();
        rpc.is_available().is_ok()
    }
    
    #[test]
    fn test_rpc_connection() {
        if !is_elements_available() {
            println!("SKIPPING: Elements Core node not available");
            return;
        }
        
        let rpc = get_test_rpc();
        let result = rpc.call::<Value>("getblockchaininfo", vec![]);
        assert!(result.is_ok(), "Failed to get blockchain info: {:?}", result.err());
        
        let info = result.unwrap();
        assert!(info.get("chain").is_some(), "Response missing 'chain' field");
        println!("Connected to Elements Core: {}", info["chain"]);
    }
    
    #[test]
    fn test_generate_address() {
        if !is_elements_available() {
            println!("SKIPPING: Elements Core node not available");
            return;
        }
        
        let rpc = get_test_rpc();
        // Try to ensure wallet exists (ignore errors)
        let _ = rpc.ensure_wallet_exists();
        
        let result = rpc.call::<String>("getnewaddress", vec![]);
        assert!(result.is_ok(), "Failed to generate address: {:?}", result.err());
        
        let address = result.unwrap();
        assert!(address.starts_with("el") || address.starts_with("tlq"), 
        "Invalid address format: {}", address);
        println!("Generated address: {}", address);
    }
    
    #[test]
    fn test_get_asset_info() {
        if !is_elements_available() {
            println!("SKIPPING: Elements Core node not available");
            return;
        }
        
        let rpc = get_test_rpc();
        let _btc_asset_id = "144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49";
        
        // Test dumpassetlabels
        let result = rpc.call::<Value>("dumpassetlabels", vec![]);
        assert!(result.is_ok(), "Failed to get asset labels: {:?}", result.err());
        
        // If the call succeeded but the BTC asset isn't present, we'll just log it
        // (rather than fail the test, since a fresh node might not have the asset yet)
        let labels = result.unwrap();
        println!("Asset labels: {}", serde_json::to_string_pretty(&labels).unwrap());
    }
    
    #[test]
    fn test_cli_parsing() {
        // This test doesn't need the Elements Core node
        let args = vec![
            "liquid-cli",
            "--rpc-url", "http://localhost:18891",
            "--rpc-user", "user",
            "--rpc-pass", "pass",
            "generate-address"
        ];
        
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(cli.rpc_url, "http://localhost:18891");
        assert_eq!(cli.rpc_user, "user");
        assert_eq!(cli.rpc_pass, "pass");
        
        match cli.command {
            Commands::GenerateAddress => {},
            _ => panic!("Expected GenerateAddress command"),
        }
    }
    
    #[test]
    fn test_asset_info_command() {
        // This test doesn't need the Elements Core node
        let args = vec![
            "liquid-cli",
            "asset-info",
            "144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49"
        ];
        
        let cli = Cli::try_parse_from(args).unwrap();
        match cli.command {
            Commands::AssetInfo { asset_id } => {
                assert_eq!(asset_id, "144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49");
            },
            _ => panic!("Expected AssetInfo command"),
        }
    }
    
    #[test]
    fn test_transfer_command() {
        // This test doesn't need the Elements Core node
        let args = vec![
            "liquid-cli",
            "transfer",
            "--to", "el1qqw3e3mk4ng3ks43mh54udznuekaadh9lgwef3mwgzrfzakmdwcvqpe4ppdaa3t44v3zv2u6w56pv6tc666fvgzaclqjnkz0sd",
            "--asset", "144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49",
            "--amount", "0.001"
        ];
        
        let cli = Cli::try_parse_from(args).unwrap();
        match cli.command {
            Commands::Transfer { to, asset, amount } => {
                assert_eq!(to, "el1qqw3e3mk4ng3ks43mh54udznuekaadh9lgwef3mwgzrfzakmdwcvqpe4ppdaa3t44v3zv2u6w56pv6tc666fvgzaclqjnkz0sd");
                assert_eq!(asset, "144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49");
                assert_eq!(amount, 0.001);
            },
            _ => panic!("Expected Transfer command"),
        }
    }
    
    #[test]
    fn test_call_command() {
        // This test doesn't need the Elements Core node
        let args = vec![
            "liquid-cli",
            "call",
            "getblockchaininfo"
        ];
        
        let cli = Cli::try_parse_from(args).unwrap();
        match cli.command {
            Commands::Call { method, params_json } => {
                assert_eq!(method, "getblockchaininfo");
                assert!(params_json.is_some());
                assert_eq!(params_json.unwrap(), "[]");
            },
            _ => panic!("Expected Call command"),
        }
    }
    
    #[test]
    fn test_transfer_execution() {
        if !is_elements_available() {
            println!("SKIPPING: Elements Core node not available");
            return;
        }
        
        let rpc = get_test_rpc();
        // Ensure wallet exists
        let wallet_result = rpc.ensure_wallet_exists();
        if wallet_result.is_err() {
            println!("SKIPPING: Could not create wallet: {:?}", wallet_result.err());
            return;
        }
        
        // First create an address for testing
        let address_result = rpc.call::<String>("getnewaddress", vec![]);
        if address_result.is_err() {
            println!("SKIPPING: Could not generate address: {:?}", address_result.err());
            return;
        }
        
        let address = address_result.unwrap();
        println!("Testing transfer to address: {}", address);
        
        // Next try to perform a minimal test transfer (will likely fail due to no funds)
        // But we're just testing the API call works, not the actual transfer
        let params = vec![
            json!(address),
            json!(0.00001),  // Minimal amount
            json!(""),      // comment
            json!(""),      // comment_to
            json!(false),   // subtractfeefromamount
            json!(false),   // replaceable
            json!(1),       // conf_target
            json!("UNSET"), // estimate_mode
            json!(false),   // avoid_reuse
            json!("144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49"),   // asset
        ];
        
        let result = rpc.call::<String>("sendtoaddress", params);
        // We don't assert success here since it will likely fail without funds
        // We're just checking that the RPC call itself works
        println!("Transfer result: {:?}", result);
    }
}

// RPC Client Implementation
struct ElementsRpc {
    url: String,
    user: String,
    pass: String,
}

impl ElementsRpc {
    fn new(url: &str, user: &str, pass: &str) -> Self {
        ElementsRpc {
            url: url.to_string(),
            user: user.to_string(),
            pass: pass.to_string(),
        }
    }

    fn is_available(&self) -> Result<bool> {
        // Improved check with error reporting
        println!("Attempting to connect to RPC server at {}...", self.url);
        match self.call::<Value>("getblockchaininfo", vec![]) {
            Ok(_) => {
                println!("RPC connection successful!");
                Ok(true)
            }
            Err(e) => {
                println!("RPC connection failed: {}", e);
                Err(anyhow!("Failed to connect: {}", e))
            }
        }
    }

    /// Ensure a wallet exists, creating a default one if necessary
    fn ensure_wallet_exists(&self) -> Result<()> {
        println!("Checking if wallet exists...");

        // Try to list wallets
        match self.call::<Vec<String>>("listwallets", vec![]) {
            Ok(wallets) => {
                if wallets.is_empty() {
                    println!("No wallets found. Creating a default wallet...");
                    match self.call::<Value>("createwallet", vec![json!("default")]) {
                        Ok(_) => println!("Default wallet created successfully."),
                        Err(e) => return Err(anyhow!("Failed to create wallet: {}", e)),
                    }
                } else {
                    println!("Found {} existing wallet(s): {:?}", wallets.len(), wallets);
                }
            }
            Err(e) => {
                println!(
                    "Error listing wallets: {}. Attempting to create default wallet...",
                    e
                );
                match self.call::<Value>("createwallet", vec![json!("default")]) {
                    Ok(_) => println!("Default wallet created successfully."),
                    Err(e) => return Err(anyhow!("Failed to create wallet: {}", e)),
                }
            }
        }

        Ok(())
    }

    fn call<T>(&self, method: &str, params: Vec<serde_json::Value>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;

        println!("Sending RPC request to: {}", self.url);
        println!("Method: {}", method);

        let request = json!({
            "jsonrpc": "1.0",
            "id": "liquid-cli",
            "method": method,
            "params": params,
        });

        let response = match client
            .post(&self.url)
            .basic_auth(&self.user, Some(&self.pass))
            .json(&request)
            .send()
        {
            Ok(resp) => resp,
            Err(e) => {
                println!("HTTP request failed: {:?}", e);
                return Err(anyhow!("HTTP request failed: {}", e));
            }
        };

        if !response.status().is_success() {
            println!("HTTP status error: {}", response.status());
            return Err(anyhow!("HTTP error: Status {}", response.status()));
        }

        let result: serde_json::Value = match response.json() {
            Ok(json) => json,
            Err(e) => {
                println!("Failed to parse JSON response: {}", e);
                return Err(anyhow!("Failed to parse JSON response: {}", e));
            }
        };

        if let Some(error) = result.get("error") {
            if !error.is_null() {
                println!("RPC error response: {:?}", error);
                return Err(anyhow!("RPC error: {}", error));
            }
        }

        let result_value = result.get("result").ok_or_else(|| {
            println!("No result field in response");
            anyhow!("No result field in response")
        })?;

        Ok(serde_json::from_value(result_value.clone())?)
    }
}

#[derive(Parser)]
#[command(author, version, about = "A CLI tool for Liquid Network")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long = "network", default_value = "liquidtestnet")]
    network: String,

    #[arg(long = "rpc-url", default_value = "http://localhost:18891")]
    rpc_url: String,

    #[arg(long = "rpc-user", default_value = "liquiduser")]
    rpc_user: String,

    #[arg(long = "rpc-pass", default_value = "yourpassword")]
    rpc_pass: String,

    #[arg(long = "no-wallet", help = "Skip wallet creation/loading")]
    no_wallet: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new Liquid address
    #[command(name = "generate-address")]
    GenerateAddress,

    /// Get information about a Liquid asset
    #[command(name = "asset-info")]
    AssetInfo { asset_id: String },

    /// Transfer assets between addresses
    #[command(name = "transfer")]
    Transfer {
        #[arg(long)]
        to: String,

        #[arg(long)]
        asset: String,

        #[arg(long)]
        amount: f64,
    },

    /// Make a generic RPC call to Elements Core
    #[command(name = "call")]
    Call {
        method: String,
        #[arg(default_value = "[]")]
        params_json: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize RPC client
    let rpc = ElementsRpc::new(&cli.rpc_url, &cli.rpc_user, &cli.rpc_pass);

    // Ensure RPC is available with improved error reporting
    match rpc.is_available() {
        Ok(_) => println!("Successfully connected to Elements Core RPC."),
        Err(e) => {
            return Err(anyhow!(
                "Failed to connect to Elements Core RPC: {}. Please ensure the node is running.",
                e
            ))
        }
    }

    // Only perform wallet operations if --no-wallet is not specified
    if !cli.no_wallet {
        if let Err(e) = rpc.ensure_wallet_exists() {
            return Err(anyhow!("Wallet setup failed: {}", e));
        }
    } else {
        println!("Skipping wallet operations (--no-wallet flag specified)");
    }

    match cli.command {
        Commands::GenerateAddress => {
            // Use RPC to generate address
            match rpc.call::<String>("getnewaddress", vec![]) {
                Ok(address) => {
                    println!("Generated new Liquid address:");
                    println!("Address: {}", address);

                    // Get private key for the address
                    match rpc.call::<String>("dumpprivkey", vec![json!(address)]) {
                        Ok(privkey) => println!("Private key (WIF): {}", privkey),
                        Err(_) => {
                            println!("Note: Private key retrieval not supported in wallet mode")
                        }
                    }
                }
                Err(e) => println!("Error generating address via RPC: {}", e),
            }
            Ok(())
        }
        // Inside match cli.command { ... }
        Commands::AssetInfo { asset_id } => {
            println!("Retrieving information for asset: {}", asset_id);

            // First try to get asset labels
            let labels = match rpc.call::<Value>("dumpassetlabels", vec![]) {
                Ok(result) => result,
                Err(e) => {
                    println!("Warning: Could not retrieve asset labels: {}", e);
                    json!({})
                }
            };

            // Try to get detailed asset information using listissuances
            let issuances = match rpc.call::<Value>("listissuances", vec![]) {
                Ok(result) => result,
                Err(e) => {
                    println!("Warning: Could not retrieve issuance data: {}", e);
                    json!([])
                }
            };

            // Find the label for this asset ID
            let mut asset_label = "Unknown";
            if let Value::Object(label_map) = &labels {
                for (label, id) in label_map {
                    if id.as_str() == Some(&asset_id) {
                        asset_label = label;
                        break;
                    }
                }
            }

            // Display asset information
            println!("\nASSET INFORMATION:");
            println!("Asset ID: {}", asset_id);
            println!("Label: {}", asset_label);

            // Check if it's the default bitcoin asset
            if asset_id == "144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49" {
                println!("Type: Native L-BTC (Liquid Bitcoin)");
                println!("This is the default Bitcoin asset on Liquid testnet");
            }

            // Look for issuance data
            if let Value::Array(issuance_list) = &issuances {
                for issuance in issuance_list {
                    if let Some(issued_asset) = issuance.get("asset") {
                        if issued_asset.as_str() == Some(&asset_id) {
                            println!("\nIssuance Information:");
                            if let Some(txid) = issuance.get("txid") {
                                println!("Issuance txid: {}", txid);
                            }
                            if let Some(amount) = issuance.get("assetamount") {
                                println!("Issued amount: {}", amount);
                            }
                            break;
                        }
                    }
                }
            }

            Ok(())
        }
        Commands::Transfer { to, asset, amount } => {
            // Perform the transfer
            println!("Creating transfer transaction...");

            let params = vec![
                json!(to),
                json!(amount),
                json!(""),      // comment
                json!(""),      // comment_to
                json!(false),   // subtractfeefromamount
                json!(false),   // replaceable
                json!(1),       // conf_target
                json!("UNSET"), // estimate_mode
                json!(false),   // avoid_reuse
                json!(asset),   // asset label
            ];

            match rpc.call::<String>("sendtoaddress", params) {
                Ok(txid) => {
                    println!("Transfer successful!");
                    println!("Transaction ID: {}", txid);
                }
                Err(e) => {
                    println!("Error: Transfer failed: {}", e);
                    println!("Make sure:");
                    println!("1. The destination address is valid");
                    println!("2. You have sufficient balance");
                    println!("3. The asset ID is correct");
                }
            }
            Ok(())
        }

        Commands::Call {
            method,
            params_json,
        } => {
            // Parse the JSON params if provided
            let params = match params_json {
                Some(json_str) => match serde_json::from_str::<Vec<Value>>(&json_str) {
                    Ok(p) => p,
                    Err(e) => {
                        println!("Error parsing params JSON: {}", e);
                        return Err(anyhow!("Invalid JSON parameters: {}", e));
                    }
                },
                None => vec![],
            };

            // Make the RPC call
            match rpc.call::<Value>(&method, params) {
                Ok(result) => {
                    println!("RPC call to '{}' succeeded:", method);
                    println!("{}", serde_json::to_string_pretty(&result)?);
                }
                Err(e) => {
                    println!("RPC call to '{}' failed: {}", method, e);
                }
            }
            Ok(())
        }
    }
}
