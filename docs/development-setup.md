# Development Setup Guide

This guide will help you set up your local development environment for Learna2Earna.

## Prerequisites

Before you begin, ensure you have the following installed:

### Required Tools

1. **Rust** (latest stable)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **WASM target**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. **Stellar CLI** (v25.x or later)
   ```bash
   # macOS
   brew install stellar-cli
   
   # Linux/WSL
   cargo install --locked stellar-cli --features opt
   ```

4. **Node.js** (v24.12.0 or later)
   ```bash
   # Using nvm (recommended)
   nvm install 24
   nvm use 24
   
   # Or download from https://nodejs.org
   ```

5. **pnpm** (latest)
   ```bash
   npm install -g pnpm
   ```

6. **Freighter Wallet**
   - Install from [freighter.app](https://freighter.app)
   - Browser extension for Chrome/Firefox/Edge

## Project Setup

### 1. Clone the Repository

```bash
git clone https://github.com/learna2earna/learna2earna.git
cd learna2earna
```

### 2. Smart Contracts Setup

```bash
# Run tests to verify setup
cargo test --workspace

# Build contracts
stellar contract build

# Run linter
cargo clippy --all-targets

# Format code
cargo fmt
```

### 3. Frontend Setup

```bash
cd frontend

# Install dependencies
pnpm install

# Copy environment variables
cp .env.example .env.local

# Start development server
pnpm dev
```

The frontend will be available at `http://localhost:5173`

## Wallet Setup

### Configure Freighter for Testnet

1. Open Freighter extension
2. Click the network dropdown (top right)
3. Select **Testnet**
4. Create a new wallet or import existing

### Get Testnet XLM

1. Visit [Stellar Laboratory](https://laboratory.stellar.org/#account-creator)
2. Click "Generate keypair"
3. Copy your public key
4. Click "Get test network lumens"
5. Import the secret key into Freighter

## Verify Installation

### Test Contracts

```bash
# Run all tests
cargo test --workspace

# Expected output:
# running X tests
# test result: ok. X passed; 0 failed
```

### Test Frontend

```bash
cd frontend
pnpm dev
```

Visit `http://localhost:5173` and verify:
- Page loads without errors
- "Connect Wallet" button is visible
- Clicking it attempts to connect to Freighter

## Common Issues

### Rust/Cargo Issues

**Problem:** `cargo: command not found`
```bash
# Solution: Add cargo to PATH
source $HOME/.cargo/env
```

**Problem:** WASM target not found
```bash
# Solution: Add WASM target
rustup target add wasm32-unknown-unknown
```

### Stellar CLI Issues

**Problem:** `stellar: command not found`
```bash
# Solution: Install via cargo
cargo install --locked stellar-cli --features opt
```

### Frontend Issues

**Problem:** `pnpm: command not found`
```bash
# Solution: Install pnpm globally
npm install -g pnpm
```

**Problem:** Port 5173 already in use
```bash
# Solution: Kill the process or use different port
pnpm dev --port 3000
```

### Wallet Issues

**Problem:** Freighter not detected
- Ensure extension is installed and enabled
- Refresh the page
- Check browser console for errors

**Problem:** Transaction fails
- Verify you're on Testnet
- Check account has sufficient XLM
- Review transaction details in Freighter

## Development Workflow

### Making Changes

1. Create a feature branch
   ```bash
   git checkout -b feat/your-feature
   ```

2. Make your changes

3. Run tests
   ```bash
   cargo test --workspace
   cd frontend && pnpm lint
   ```

4. Commit with conventional commits
   ```bash
   git commit -m "feat: add your feature"
   ```

5. Push and open PR
   ```bash
   git push origin feat/your-feature
   ```

### Hot Reload

- **Contracts:** Rebuild after changes (`stellar contract build`)
- **Frontend:** Auto-reloads on save (Vite HMR)

## IDE Setup

### VS Code (Recommended)

Install these extensions:
- `rust-analyzer` — Rust language support
- `Tailwind CSS IntelliSense` — Tailwind autocomplete
- `ESLint` — JavaScript/TypeScript linting
- `Prettier` — Code formatting

### Settings

Add to `.vscode/settings.json`:
```json
{
  "rust-analyzer.cargo.target": "wasm32-unknown-unknown",
  "editor.formatOnSave": true,
  "editor.defaultFormatter": "esbenp.prettier-vscode"
}
```

## Next Steps

- Read [CONTRIBUTING.md](../CONTRIBUTING.md) for contribution guidelines
- Check [docs/architecture.md](architecture.md) for system design
- Browse [open issues](https://github.com/learna2earna/learna2earna/issues) for tasks

## Getting Help

- **Questions?** Open a [discussion](https://github.com/learna2earna/learna2earna/discussions)
- **Bug?** Open an [issue](https://github.com/learna2earna/learna2earna/issues)
- **Stuck?** Comment on the issue you're working on

Happy coding! 🚀
