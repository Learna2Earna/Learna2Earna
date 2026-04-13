# Quick Start Guide

Get Learna2Earna running in 5 minutes.

## Prerequisites Check

```bash
# Check Rust
rustc --version

# Check Stellar CLI
stellar --version

# Check Node.js
node --version

# Check pnpm
pnpm --version
```

If any command fails, see [docs/development-setup.md](docs/development-setup.md) for installation instructions.

## 1. Clone & Setup

```bash
git clone https://github.com/yourusername/learna2earna.git
cd learna2earna
```

## 2. Test Contracts

```bash
# Run contract tests
cargo test --workspace

# Expected: All tests pass ✅
```

## 3. Build Contracts

```bash
# Build optimized WASM
stellar contract build

# Or use the script
./scripts/build.sh
```

## 4. Start Frontend

```bash
cd frontend
pnpm install
cp .env.example .env.local
pnpm dev
```

Visit: `http://localhost:5173`

## 5. Connect Wallet

1. Install [Freighter](https://freighter.app) browser extension
2. Switch to **Testnet** network
3. Get testnet XLM from [Stellar Laboratory](https://laboratory.stellar.org/#account-creator)
4. Click "Connect Wallet" in the app

## What's Next?

- **Deploy to testnet:** See [docs/deployment.md](docs/deployment.md)
- **Contribute:** See [CONTRIBUTING.md](CONTRIBUTING.md)
- **Learn architecture:** See [docs/architecture.md](docs/architecture.md)

## Troubleshooting

### Contracts won't build
```bash
rustup target add wasm32-unknown-unknown
cargo clean
stellar contract build
```

### Frontend won't start
```bash
cd frontend
rm -rf node_modules
pnpm install
pnpm dev
```

### Wallet won't connect
- Ensure Freighter is installed
- Switch to Testnet in Freighter
- Refresh the page

## Need Help?

- [Open an issue](https://github.com/yourusername/learna2earna/issues)
- [Start a discussion](https://github.com/yourusername/learna2earna/discussions)

Happy building! 🚀
