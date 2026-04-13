# Learna2Earna

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Stellar](https://img.shields.io/badge/Stellar-Soroban-blue)](https://stellar.org)

> **Learn to earn, on-chain.** A decentralized learning platform where educators create skill-building quests, learners complete milestones, and smart contracts handle the rewards. No middlemen, just code and commitment.

## 🎯 Vision

Learna2Earna transforms education through blockchain-based incentives. Create learning paths, set milestones, fund them with tokens, and watch learners earn as they grow. Built on Stellar's Soroban smart contracts for transparency, security, and zero infrastructure costs.

## 🚀 Why Learna2Earna?

Traditional learning platforms rely on motivation alone. Learna2Earna adds **financial commitment** — real tokens locked in smart contracts that learners earn by proving their progress.

| Use Case | How It Works |
|----------|--------------|
| **Corporate Training** | Onboard developers with milestone-based token rewards |
| **DAO Education** | Fund community learning with verifiable outcomes |
| **Mentorship** | Back your mentee's journey with real financial stakes |
| **Self-Learning** | Commit your own funds to stay accountable |

## 🏗️ Architecture

Learna2Earna uses a **three-contract architecture** on Stellar Soroban:

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

**Why three contracts?**
- **Separation of concerns** — each handles one responsibility
- **Independent upgrades** — update rewards without touching quests
- **Smaller WASM** — each stays under Soroban's 256KB limit
- **Clear security boundaries** — scoped permissions per contract

**Why no backend?** The blockchain IS the backend. All state lives on Stellar's ledger. Zero servers, zero databases, full transparency.

## 📦 Project Structure

```
learna2earna/
├── contracts/              # Soroban smart contracts (Rust)
│   ├── quest/             # Quest creation & enrollment
│   ├── milestone/         # Milestone tracking & verification
│   └── rewards/           # Token pools & distribution
├── frontend/              # React + TypeScript UI
│   ├── src/
│   │   ├── components/    # Reusable UI components
│   │   ├── pages/         # Main application pages
│   │   ├── hooks/         # Custom React hooks
│   │   └── lib/           # Utilities & contract clients
│   └── public/            # Static assets
├── scripts/               # Deployment & utility scripts
├── docs/                  # Documentation
└── .github/               # CI/CD workflows
```

## 🛠️ Tech Stack

| Layer | Technology |
|-------|------------|
| **Smart Contracts** | Rust + Soroban SDK v22 |
| **Frontend** | React 19 + TypeScript 5.9 + Vite |
| **UI Framework** | Tailwind CSS + shadcn/ui |
| **Wallet** | Freighter (Stellar browser wallet) |
| **Network** | Stellar Testnet (Soroban-enabled) |
| **CI/CD** | GitHub Actions |

## 🚦 Getting Started

### Prerequisites

| Tool | Version | Installation |
|------|---------|--------------|
| **Rust** | Latest stable | [rustup.rs](https://rustup.rs) |
| **WASM target** | - | `rustup target add wasm32-unknown-unknown` |
| **Stellar CLI** | 25.x+ | `brew install stellar-cli` or [docs](https://developers.stellar.org/docs/tools/developer-tools/cli/install-cli) |
| **Node.js** | 24.12.0+ | [nodejs.org](https://nodejs.org) |
| **pnpm** | Latest | `npm install -g pnpm` |
| **Freighter** | Latest | [freighter.app](https://freighter.app) |

### Quick Start

```bash
# Clone the repository
git clone https://github.com/learna2earna/learna2earna.git
cd learna2earna

# Smart Contracts
cargo test --workspace              # Run tests
stellar contract build              # Build optimized WASM

# Frontend
cd frontend
pnpm install
cp .env.example .env.local          # Configure environment
pnpm dev                            # Start dev server → localhost:5173
```

### Wallet Setup

1. Install [Freighter](https://freighter.app) browser extension
2. Switch to **Testnet** network
3. Create or import a wallet
4. Get testnet XLM from [Stellar Laboratory](https://laboratory.stellar.org/#account-creator)

## 📚 Smart Contracts

### Quest Contract

Manages quest creation and learner enrollment.

| Function | Description |
|----------|-------------|
| `create_quest(owner, name, description, token_addr)` | Create a new learning quest |
| `add_enrollee(quest_id, learner)` | Enroll a learner (owner only) |
| `get_quest(quest_id)` | Retrieve quest details |
| `is_enrollee(quest_id, user)` | Check enrollment status |

### Milestone Contract

Tracks learning milestones and verifies completion.

| Function | Description |
|----------|-------------|
| `create_milestone(owner, quest_id, title, reward_amount)` | Add milestone to quest |
| `verify_completion(owner, quest_id, milestone_id, learner)` | Mark milestone complete |
| `get_milestones(quest_id)` | List all milestones |
| `is_completed(quest_id, milestone_id, learner)` | Check completion status |

### Rewards Contract

Manages token pools and distributes rewards.

| Function | Description |
|----------|-------------|
| `fund_quest(funder, quest_id, amount)` | Deposit tokens into quest pool |
| `distribute_reward(authority, quest_id, learner, amount)` | Send reward to learner |
| `get_pool_balance(quest_id)` | Check quest pool balance |
| `get_user_earnings(user)` | Get total learner earnings |

## 🎨 Frontend Features

- **Quest Discovery** — Browse available learning quests
- **Quest Creation** — Set up custom learning paths
- **Milestone Tracking** — Monitor learner progress
- **Wallet Integration** — Connect with Freighter
- **Reward Distribution** — Automated token payouts
- **Profile Dashboard** — View earnings and completed quests

## 🧪 Development

### Running Tests

```bash
# Smart contracts
cargo test --workspace
cargo clippy --all-targets

# Frontend
cd frontend
pnpm test
pnpm lint
```

### Building for Production

```bash
# Contracts (optimized WASM)
stellar contract build

# Frontend
cd frontend
pnpm build
```

### Deploying to Testnet

See [docs/deployment.md](docs/deployment.md) for detailed deployment instructions.

## 🤝 Contributing

We welcome contributions! Here's how to get involved:

### 1. Find an Issue

Browse [open issues](https://github.com/learna2earna/learna2earna/issues) or look for `good first issue` labels for beginner-friendly tasks.

### 2. Set Up Your Environment

```bash
# Fork the repository on GitHub
# Clone your fork
git clone https://github.com/YOUR_USERNAME/learna2earna.git
cd learna2earna

# Create a feature branch
git checkout -b feat/your-feature-name
```

### 3. Development Workflow

```bash
# Make your changes
# Run tests
cargo test --workspace
cd frontend && pnpm test

# Format code
cargo fmt
cd frontend && pnpm lint --fix

# Commit with conventional commits
git commit -m "feat: add milestone deadline feature"
```

### 4. Submit a Pull Request

- Push your branch: `git push origin feat/your-feature-name`
- Open a PR on GitHub
- Reference the issue: `closes #123`
- Fill out the PR template
- Wait for review

### Branch Naming Conventions

| Prefix | Purpose | Example |
|--------|---------|---------|
| `feat/` | New features | `feat/add-quest-categories` |
| `fix/` | Bug fixes | `fix/wallet-disconnect-error` |
| `docs/` | Documentation | `docs/update-deployment-guide` |
| `refactor/` | Code refactoring | `refactor/extract-reward-logic` |
| `test/` | Tests | `test/add-milestone-edge-cases` |
| `chore/` | Maintenance | `chore/update-dependencies` |

### Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org):

```
<type>: <description>

[optional body]

[optional footer]
```

**Examples:**
```
feat: add quest visibility settings
fix: resolve token transfer overflow
docs: add contract deployment guide
refactor: simplify milestone verification logic
test: add reward distribution edge cases
chore: update soroban-sdk to v22
```

### Code Style Guidelines

#### Rust Contracts

- Run `cargo fmt` before committing
- Address all `cargo clippy` warnings
- Every public function returns `Result<T, Error>`
- Add tests for new features
- Use existing storage patterns (Instance/Persistent/Temporary)
- Document complex logic with comments

#### Frontend (TypeScript/React)

- TypeScript strict mode — no `any` types
- Use existing shadcn/ui components
- Kebab-case for file names: `quest-card.tsx`
- Tailwind for styling — no inline styles
- Extract reusable logic into custom hooks
- Add JSDoc comments for complex functions

### Pull Request Checklist

Before submitting your PR, ensure:

- [ ] Code follows project style guidelines
- [ ] All tests pass (`cargo test` and `pnpm test`)
- [ ] New features include tests
- [ ] Documentation is updated
- [ ] Commit messages follow conventional commits
- [ ] PR description references related issue
- [ ] No merge conflicts with main branch

### What We Review

- **Functionality** — Does it work as intended?
- **Tests** — Are edge cases covered?
- **Code quality** — Is it readable and maintainable?
- **Documentation** — Are changes documented?
- **Security** — Are there potential vulnerabilities?

### Getting Help

- **Questions?** Open a [discussion](https://github.com/learna2earna/learna2earna/discussions)
- **Stuck?** Comment on the issue you're working on
- **Found a bug?** Open an issue with reproduction steps

## 🏆 Recognition

Every contribution matters. Here's how we recognize contributors:

- **Release Credits** — Named in every GitHub release
- **README Gallery** — Your avatar appears after first merged PR
- **Founding Contributors** — Pre-v1.0 contributors get special recognition
- **Maintainer Path** — Active contributors invited as collaborators
- **Roadmap Input** — Contributors participate in architecture decisions

## 📋 Roadmap

| Milestone | Status | Focus |
|-----------|--------|-------|
| **M1** Foundation | ✅ Complete | Core contracts, basic UI |
| **M2** Integration | 🚧 In Progress | Full contract-frontend wiring |
| **M3** Features | 📋 Planned | Quest categories, deadlines, peer verification |
| **M4** Polish | 📋 Planned | Security audit, performance optimization |
| **M5** Mainnet | 📋 Planned | Production deployment |

See the [project board](https://github.com/learna2earna/learna2earna/projects) for detailed progress.

## 🔒 Security

Security is critical for smart contract platforms. If you discover a vulnerability:

1. **DO NOT** open a public issue
2. Email security@learna2earna.com with details
3. Include steps to reproduce
4. Allow 90 days for patching before disclosure

See [SECURITY.md](SECURITY.md) for our full security policy.

## 📄 License

This project is licensed under the MIT License - see [LICENSE](LICENSE) for details.

## 🌟 Acknowledgments

Built with:
- [Stellar](https://stellar.org) — Fast, low-cost blockchain
- [Soroban](https://soroban.stellar.org) — Smart contracts on Stellar
- [React](https://react.dev) — UI framework
- [Tailwind CSS](https://tailwindcss.com) — Utility-first CSS

## 📞 Contact

- **Website:** [learna2earna.com](https://learna2earna.com)
- **Twitter:** [@learna2earna](https://twitter.com/learna2earna)
- **Discord:** [Join our community](https://discord.gg/learna2earna)
- **Email:** hello@learna2earna.com

---

**Built with 💙 by the Learna2Earna community**

[⭐ Star this repo](https://github.com/learna2earna/learna2earna) if you find it useful!
