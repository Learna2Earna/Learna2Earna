# Learna2Earna - Project Summary

## Overview

Learna2Earna is a decentralized learn-to-earn platform built on Stellar's Soroban smart contracts, focusing on core functionality and comprehensive documentation.

## What's Implemented

### Smart Contracts (Rust/Soroban)

#### 1. Quest Contract (`contracts/quest/`)
- ✅ Create quests with metadata
- ✅ Enroll learners
- ✅ Query quest details
- ✅ Check enrollment status
- ✅ Unit tests

#### 2. Milestone Contract (`contracts/milestone/`)
- ✅ Create milestones for quests
- ✅ Verify milestone completion
- ✅ Track learner progress
- ✅ Query milestone data
- ✅ Unit tests

#### 3. Rewards Contract (`contracts/rewards/`)
- ✅ Fund quest pools
- ✅ Distribute rewards to learners
- ✅ Track pool balances
- ✅ Track user earnings
- ✅ Unit tests

### Frontend (React + TypeScript)

- ✅ Basic UI structure
- ✅ Wallet connection placeholder
- ✅ Landing page
- ✅ Tailwind CSS setup
- ✅ TypeScript configuration
- ✅ Vite build system

### Documentation

- ✅ Comprehensive README with contribution guide
- ✅ Architecture documentation
- ✅ Deployment guide
- ✅ Development setup guide
- ✅ Security policy
- ✅ Contributing guidelines
- ✅ Project roadmap
- ✅ Quick start guide

### Infrastructure

- ✅ GitHub Actions CI workflow
- ✅ Build and test scripts
- ✅ EditorConfig
- ✅ Git ignore rules
- ✅ MIT License

## What's NOT Implemented (Future Work)

### Smart Contracts
- ❌ Quest visibility settings (public/private)
- ❌ Quest categories and tags
- ❌ Enrollment caps
- ❌ Milestone deadlines
- ❌ Peer verification
- ❌ Multi-signature support
- ❌ Certificate NFTs
- ❌ Cross-contract calls

### Frontend
- ❌ Actual Freighter wallet integration
- ❌ Contract client generation
- ❌ Quest creation UI
- ❌ Quest discovery/browsing
- ❌ Milestone tracking dashboard
- ❌ Reward distribution UI
- ❌ User profile page
- ❌ Analytics dashboard

### Features
- ❌ Search and filtering
- ❌ Pagination
- ❌ Real-time notifications
- ❌ Mobile app
- ❌ Indexer for analytics
- ❌ Dispute resolution

## Project Structure

```
learna2earna/
├── contracts/              # Soroban smart contracts
│   ├── quest/             # Quest management
│   ├── milestone/         # Milestone tracking
│   └── rewards/           # Token distribution
├── frontend/              # React application
│   ├── src/
│   │   ├── App.tsx        # Main component
│   │   ├── main.tsx       # Entry point
│   │   └── index.css      # Global styles
│   └── package.json
├── docs/                  # Documentation
│   ├── architecture.md
│   ├── deployment.md
│   └── development-setup.md
├── scripts/               # Build/test scripts
├── .github/               # CI/CD workflows
├── README.md              # Main documentation
├── CONTRIBUTING.md        # Contribution guide
├── SECURITY.md            # Security policy
├── ROADMAP.md             # Development roadmap
└── QUICKSTART.md          # Quick start guide
```

## Key Features

### Three-Contract Architecture
- **Separation of concerns** — each contract has one responsibility
- **Independent upgrades** — update one without touching others
- **Smaller WASM binaries** — under Soroban's 256KB limit
- **Clear security boundaries** — scoped permissions

### No Backend Required
- All state lives on Stellar's ledger
- Zero infrastructure costs
- Full transparency
- Frontend orchestrates contract interactions

### Developer-Friendly
- Comprehensive documentation
- Clear contribution guidelines
- Automated CI/CD
- Type-safe contracts and frontend

## Getting Started

### Quick Start
```bash
# Clone repository
git clone https://github.com/learna2earna/learna2earna.git
cd learna2earna

# Test contracts
cargo test --workspace

# Build contracts
stellar contract build

# Start frontend
cd frontend && pnpm install && pnpm dev
```

See [QUICKSTART.md](QUICKSTART.md) for detailed instructions.

## Contributing

We welcome contributions! Here's how:

1. Browse [open issues](https://github.com/learna2earna/learna2earna/issues)
2. Look for `good first issue` labels
3. Read [CONTRIBUTING.md](CONTRIBUTING.md)
4. Fork, code, test, and submit PR

### Contribution Areas

- **Smart Contracts:** Add features, improve tests, optimize gas
- **Frontend:** Build UI components, integrate contracts, improve UX
- **Documentation:** Fix typos, add examples, translate
- **Testing:** Write tests, find bugs, improve coverage
- **DevOps:** Improve CI/CD, add monitoring, optimize builds

## Roadmap

| Phase | Status | Focus |
|-------|--------|-------|
| M1: Foundation | ✅ Complete | Core contracts, basic UI, docs |
| M2: Integration | 🚧 Next | Full contract-frontend wiring |
| M3: Features | 📋 Planned | Categories, deadlines, verification |
| M4: Polish | 📋 Planned | Audit, optimization, testing |
| M5: Mainnet | 📋 Planned | Production deployment |

See [ROADMAP.md](ROADMAP.md) for details.

## Tech Stack

- **Smart Contracts:** Rust + Soroban SDK v22
- **Frontend:** React 19 + TypeScript 5.9 + Vite
- **Styling:** Tailwind CSS v4
- **Wallet:** Freighter (Stellar)
- **Network:** Stellar Testnet
- **CI/CD:** GitHub Actions

## Current Implementation

### Implemented Features
- ✅ Three-contract architecture
- ✅ Quest creation and enrollment
- ✅ Milestone tracking
- ✅ Reward distribution
- ✅ Basic frontend structure

### Planned Features
- ⏳ Quest visibility settings
- ⏳ Verification modes (owner/self/submit-approve)
- ⏳ Distribution modes (flat/competitive/custom)
- ⏳ Certificate NFTs
- ⏳ Advanced UI features
- ⏳ Full wallet integration

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

- Built on [Stellar](https://stellar.org) and [Soroban](https://soroban.stellar.org)
- Powered by [React](https://react.dev) and [Tailwind CSS](https://tailwindcss.com)

## Contact

- **GitHub:** [yourusername/learna2earna](https://github.com/learna2earna/learna2earna)
- **Issues:** [Report bugs or request features](https://github.com/learna2earna/learna2earna/issues)
- **Discussions:** [Ask questions or share ideas](https://github.com/learna2earna/learna2earna/discussions)

---

**Built with 💙 by the Learna2Earna community**

[⭐ Star this repo](https://github.com/learna2earna/learna2earna) if you find it useful!
