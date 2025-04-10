# Liquid CLI

A simple command-line tool for interacting with the Liquid Network, written in Rust.

## Features

- Generate Liquid addresses
- Get information about Liquid assets
- Transfer assets between addresses

## Elements Core Setup (Required for Online Features)

### Using Docker

```bash
# Create a directory for Elements data and configuration
mkdir -p ~/elements-data/liquidtestnet

# Create a proper configuration file
cat > ~/elements-data/liquidtestnet/elements.conf << EOL
# Global settings
chain=liquidtestnet
server=1
validatepegin=0
txindex=1
daemon=0
wallet=default

# Network specific settings
[liquidtestnet]
rpcuser=liquiduser
rpcpassword=yourpassword
rpcallowip=0.0.0.0/0
rpcbind=0.0.0.0:18891
listen=1
EOL

# Run the Elements Core container
docker run -d \
  --name liquid-testnet \
  -v ~/elements-data:/home/elements/.elements \
  -p 18891:18891 \
  -p 18892:18892 \
  blockstream/elementsd:latest \
  elementsd -conf=/home/elements/.elements/liquidtestnet/elements.conf
```

## Installation

### Prerequisites

- Rust and Cargo installed (https://rustup.rs/)
- Elements Core node (required for online features)

### Building from Source

```bash
# Clone this repository
git clone https://github.com/vkprogrammer-001/liquid-cli.git
cd liquid-cli

# Build the project
cargo build --release
```

## Usage

### Generate a Liquid Address

```bash
# Generate a new Liquid testnet address with RPC connection
./target/release/liquid-cli --rpc-url http://localhost:18891 --rpc-user liquiduser --rpc-pass yourpassword generate-address
```

### Get Asset Information

```bash
# First get labels to identify the asset
./target/release/liquid-cli --no-wallet call dumpassetlabels

# Then get issuance information
./target/release/liquid-cli --no-wallet call listissuances <asset-id>
```

### Transfer Assets

```bash
# Transfer assets between addresses
./target/release/liquid-cli --rpc-url http://localhost:18891 --rpc-user liquiduser --rpc-pass yourpassword transfer --to <address> --asset <asset-id> --amount <amount>
```

### Generic RPC Call

```bash
# Make any RPC call to Elements Core
./target/release/liquid-cli --rpc-url http://localhost:18891 --rpc-user liquiduser --rpc-pass yourpassword --no-wallet call <method> [params_json]

# Examples:
./target/release/liquid-cli --no-wallet call dumpassetlabels
./target/release/liquid-cli --no-wallet call getblockchaininfo
./target/release/liquid-cli --no-wallet call listissuances
```

## Network Options

- `--network liquid` - Use Liquid mainnet
- `--network liquidtestnet` - Use Liquid testnet (default)

## RPC Options

- `--rpc-url` - URL of the Elements Core RPC server (default: http://localhost:18891)
- `--rpc-user` - Username for RPC authentication (default: liquiduser)
- `--rpc-pass` - Password for RPC authentication (default: yourpassword)

## Technical Notes

### This tool uses the following Rust crates:

- `elements` - For Liquid-specific functionality
- `bitcoin` - For Bitcoin key handling
- `clap` - For command-line argument parsing
- `serde_json` - For JSON serialization/deserialization
- `reqwest` - For HTTP requests to the RPC server
- `anyhow` - For error handling

## Competency Test Progress

- [x] Generate a Liquid address 
- [x] Connect to Elements Core node
- [x] Display asset information
- [x] Transfer assets between addresses
