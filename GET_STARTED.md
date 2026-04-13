# 🚀 Get Started with Learna2Earna

Welcome to Learna2Earna! This guide will get you up and running in minutes.

## 📋 What You'll Need

Before starting, make sure you have:

- [ ] **Rust** (latest stable) - [Install](https://rustup.rs)
- [ ] **Stellar CLI** (v25.x+) - [Install](https://developers.stellar.org/docs/tools/developer-tools/cli/install-cli)
- [ ] **Node.js** (v24.12.0+) - [Install](https://nodejs.org)
- [ ] **pnpm** - Install with `npm install -g pnpm`
- [ ] **Freighter Wallet** - [Install](https://freighter.app)

## 🎯 Quick Start (5 minutes)

### Step 1: Clone the Repository

```bash
git clone https://github.com/yourusername/learna2earna.git
cd learna2earna
```

### Step 2: Test Smart Contracts

```bash
# Run all contract tests
cargo test --workspace

# You should see:
# ✅ test_create_quest ... ok
# ✅ test_add_enrollee ... ok
# ✅ test_create_milestone ... ok
# ✅ test_verify_completion ... ok
# ✅ test_fund_quest ... ok
# ✅ test_distribute_reward ... ok
```

### Step 3: Build Contracts

```bash
# Build optimized WASM binaries
stellar contract build

# Or use the helper script
./scripts/build.sh
```

### Step 4: Start Frontend

```bash
cd frontend
pnpm install
cp .env.example .env.local
pnpm dev
```

Visit: **http://localhost:5173** 🎉

### Step 5: Connect Wallet

1. Install [Freighter](https://freighter.app) browser extension
2. Switch to **Testnet** network
3. Get testnet XLM from [Stellar Laboratory](https://laboratory.stellar.org/#account-creator)
4. Click "Connect Wallet" in the app

## 📚 What to Read Next

### For Developers
- **[QUICKSTART.md](QUICKSTART.md)** - Detailed quick start guide
- **[docs/development-setup.md](docs/development-setup.md)** - Full dev environment setup
- **[docs/architecture.md](docs/architecture.md)** - System architecture

### For Contributors
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - How to contribute
- **[ROADMAP.md](ROADMAP.md)** - Project roadmap
- **[PROJECT_SUMMARY.md](PROJECT_SUMMARY.md)** - What's implemented

### For Deployers
- **[docs/deployment.md](docs/deployment.md)** - Deploy to Stellar testnet

## 🎓 Understanding the Project

### Three-Contract Architecture

```
┌─────────────────┐
│  Quest Contract │  ← Create quests, manage enrollments
└────────┬────────┘
         │
┌────────▼────────────┐
│ Milestone Contract  │  ← Define milestones, verify completion
└────────┬────────────┘
         │
┌────────▼────────────┐
│  Rewards Contract   │  ← Fund pools, distribute tokens
└─────────────────────┘
```

### How It Works

1. **Educator creates a quest** (e.g., "Learn Rust Programming")
2. **Educator funds the quest** with tokens
3. **Educator adds milestones** (e.g., "Complete Chapter 1")
4. **Educator enrolls learners**
5. **Learner completes work** (off-chain)
6. **Educator verifies completion** (on-chain)
7. **Smart contract distributes reward** (automatic)

## 🛠️ Common Commands

### Smart Contracts

```bash
# Run tests
cargo test --workspace

# Build contracts
stellar contract build

# Run linter
cargo clippy --all-targets

# Format code
cargo fmt
```

### Frontend

```bash
cd frontend

# Install dependencies
pnpm install

# Start dev server
pnpm dev

# Build for production
pnpm build

# Run linter
pnpm lint
```

## 🐛 Troubleshooting

### Rust Issues

**Problem:** `cargo: command not found`
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Problem:** WASM target not found
```bash
rustup target add wasm32-unknown-unknown
```

### Frontend Issues

**Problem:** Port 5173 already in use
```bash
pnpm dev --port 3000
```

**Problem:** Dependencies won't install
```bash
rm -rf node_modules
pnpm install
```

### Wallet Issues

**Problem:** Freighter not detected
- Ensure extension is installed and enabled
- Refresh the page
- Check browser console for errors

## 🤝 Getting Help

- **Questions?** Open a [discussion](https://github.com/yourusername/learna2earna/discussions)
- **Bug?** Open an [issue](https://github.com/yourusername/learna2earna/issues)
- **Want to contribute?** Read [CONTRIBUTING.md](CONTRIBUTING.md)

## 🎉 You're Ready!

You now have:
- ✅ Smart contracts tested and built
- ✅ Frontend running locally
- ✅ Wallet configured for testnet
- ✅ Understanding of the architecture

**Next steps:**
1. Explore the code in `contracts/` and `frontend/src/`
2. Read the documentation in `docs/`
3. Pick an issue from the [project board](https://github.com/yourusername/learna2earna/issues)
4. Make your first contribution!

---

**Built with 💙 on Stellar Soroban**

[⭐ Star this repo](https://github.com/yourusername/learna2earna) if you find it useful!
