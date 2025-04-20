# Bitcoin Wallet Demo using BDK

This is a simple Bitcoin wallet implementation using the Bitcoin Dev Kit (BDK) library. The project demonstrates basic wallet functionality including address generation and balance checking.

## Features

- Creates a Bitcoin wallet using BDK
- Generates new addresses
- Checks wallet balance
- Connects to Electrum server for blockchain data

## Dependencies

- bitcoin = "0.30.0"
- bdk = "0.28.2"
- anyhow = "1.0"

## Usage

1. Clone the repository:
```bash
git clone https://github.com/guptapratykshh/pg-bdk.git
cd pg-bdk
```

2. Build and run:
```bash
cargo build
cargo run
```

## Implementation Details

The wallet implementation uses:
- Testnet network for development
- Electrum server for blockchain data
- Memory database for wallet storage
- BIP84 derivation path for address generation

## Contributing

This project is open for contributions. Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 