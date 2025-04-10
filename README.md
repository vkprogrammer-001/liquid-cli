# Liquid CLI

A simple command-line tool for interacting with the Liquid Network, written in Rust.

## Features

- Generate Liquid addresses (unconfidential only)
- Display information about Liquid assets

## Installation

### Prerequisites

- Rust and Cargo installed (https://rustup.rs/)

### Building from Source

```bash
# Clone this repository
git clone https://github.com/vkprogrammer-001/liquid-cli.git
cd liquid-cli

# Build the project
cargo build --release

# The binary will be available at target/release/liquid-cli
```

## Usage

### Running the CLI

You can run the CLI in two ways:

#### Option 1: Using Cargo (recommended)
```bash
cargo run --release -- generate-address
cargo run --release -- --network liquid generate-address
```

#### Option 2: Running the executable directly
```bash
# Navigate to the release directory
cd target/release

# Run the executable
./liquid-cli generate-address
./liquid-cli --network liquid generate-address
```

### Generate a Liquid Address

```bash
# Generate a new Liquid address (testnet by default)
cargo run --release -- generate-address

# Generate a new Liquid mainnet address
cargo run --release -- --network liquid generate-address
```

The generated output includes:
- Private key in WIF format
- Public key
- Unconfidential address

### Get Asset Information

```bash
# Get information about a Liquid asset
cargo run --release -- asset-info 6f0279e9ed041c3d710a9f57d0c02928416460c4b722ae3457a11eec381c526d

# Use a custom API endpoint
cargo run --release -- --api-url https://blockstream.info/liquidtestnet/api asset-info <asset-id>
```

## Network Options

- `--network liquid` - Use Liquid mainnet
- `--network liquidtestnet` - Use Liquid testnet (default)

## API Endpoint Options

- `--api-url <url>` - Specify a custom API endpoint for asset information

## Technical Notes

This tool uses the following Rust crates:
- `elements` - For Liquid-specific functionality
- `bitcoin` - For Bitcoin key handling
- `clap` - For command-line argument parsing
- `reqwest` - For API requests

## Future Enhancements

Potential future improvements:
- Add support for confidential transactions
- Implement wallet functionality
- Add transaction building capabilities 