# Contributing to Learna2Earna

Thanks for your interest in contributing! Learna2Earna is an open-source learn-to-earn platform on Stellar, and we welcome all types of contributions.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork:**
   ```bash
   git clone https://github.com/YOUR_USERNAME/learna2earna.git
   cd learna2earna
   ```
3. **Create a feature branch:**
   ```bash
   git checkout -b feat/your-feature-name
   ```
4. **Make your changes**
5. **Push and open a pull request**

## Development Setup

### Smart Contracts (Rust/Soroban)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
brew install stellar-cli

# Run tests
cargo test --workspace

# Build contracts
stellar contract build
```

### Frontend (React/TypeScript)

```bash
cd frontend
cp .env.example .env.local
pnpm install
pnpm dev          # Start dev server
pnpm build        # Production build
pnpm lint         # Run linter
```

## Branch Naming Conventions

Use conventional prefixes:

- `feat/` — New features
- `fix/` — Bug fixes
- `docs/` — Documentation changes
- `refactor/` — Code refactoring
- `test/` — Adding or updating tests
- `chore/` — Maintenance tasks

## Commit Message Format

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

## Code Style Guidelines

### Rust Contracts

- Run `cargo fmt` before committing
- Address all `cargo clippy` warnings
- Every public function returns `Result<T, Error>`
- Add tests for new features
- Use existing storage patterns (Instance/Persistent/Temporary)
- Document complex logic with comments

### Frontend (TypeScript/React)

- TypeScript strict mode — no `any` types
- Use existing shadcn/ui components before creating custom ones
- Kebab-case for file names: `quest-card.tsx`
- Tailwind for styling — no inline styles
- Extract reusable logic into custom hooks
- Add JSDoc comments for complex functions

## Pull Request Checklist

Before submitting your PR:

- [ ] Code follows project style guidelines
- [ ] All tests pass (`cargo test` and `pnpm test`)
- [ ] New features include tests
- [ ] Documentation is updated
- [ ] Commit messages follow conventional commits
- [ ] PR description references related issue
- [ ] No merge conflicts with main branch

## What We Review

- **Functionality** — Does it work as intended?
- **Tests** — Are edge cases covered?
- **Code quality** — Is it readable and maintainable?
- **Documentation** — Are changes documented?
- **Security** — Are there potential vulnerabilities?

## Issues

Before creating a new issue, search existing issues to avoid duplicates.

### Bug Reports

Include:
- Steps to reproduce
- Expected behavior
- Actual behavior
- Browser/OS/wallet version
- Screenshots if applicable

### Feature Requests

Include:
- Clear description of the feature
- Why it's useful
- How it fits the existing architecture

## Good First Issues

Look for issues labeled `good first issue`. These are scoped, well-documented, and designed for new contributors.

## Getting Help

- **Questions?** Open a [discussion](https://github.com/learna2earna/learna2earna/discussions)
- **Stuck?** Comment on the issue you're working on
- **Found a bug?** Open an issue with reproduction steps

## Code of Conduct

Be respectful and constructive. We're building a welcoming community.

## Recognition

Every contribution matters:

- **Release credits** — Named in every GitHub release
- **README contributors gallery** — Your avatar appears after first merged PR
- **Founding contributors** — Pre-v1.0 contributors get special recognition
- **Maintainer path** — Active contributors invited as collaborators
- **Roadmap input** — Contributors participate in architecture decisions

Thank you for contributing to Learna2Earna! 🚀
