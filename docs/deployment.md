# Deployment Guide

This guide covers deploying Learna2Earna smart contracts to Stellar testnet.

## Prerequisites

- Stellar CLI installed (`stellar-cli` v25.x+)
- Testnet account with XLM balance
- Contracts built (`stellar contract build`)

## Step 1: Configure Network

```bash
# Add testnet network
stellar network add testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"
```

## Step 2: Generate Identity

```bash
# Generate a new identity for deployment
stellar keys generate deployer --network testnet

# Fund the account (get testnet XLM)
# Visit: https://laboratory.stellar.org/#account-creator
```

## Step 3: Deploy Contracts

### Deploy Quest Contract

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/quest.wasm \
  --source deployer \
  --network testnet
```

Save the returned contract ID as `QUEST_CONTRACT_ID`.

### Deploy Milestone Contract

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/milestone.wasm \
  --source deployer \
  --network testnet
```

Save the returned contract ID as `MILESTONE_CONTRACT_ID`.

### Deploy Rewards Contract

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/rewards.wasm \
  --source deployer \
  --network testnet
```

Save the returned contract ID as `REWARDS_CONTRACT_ID`.

## Step 4: Configure Frontend

Update `frontend/.env.local`:

```env
VITE_QUEST_CONTRACT_ID=<your_quest_contract_id>
VITE_MILESTONE_CONTRACT_ID=<your_milestone_contract_id>
VITE_REWARDS_CONTRACT_ID=<your_rewards_contract_id>
```

## Step 5: Test Deployment

```bash
# Invoke a contract function to verify deployment
stellar contract invoke \
  --id $QUEST_CONTRACT_ID \
  --source deployer \
  --network testnet \
  -- \
  get_quest \
  --quest_id 1
```

## Troubleshooting

### Contract Not Found

- Verify contract ID is correct
- Check network configuration
- Ensure account has sufficient XLM

### Transaction Failed

- Check account balance
- Verify function parameters
- Review contract logs

## Next Steps

- Test contract interactions via frontend
- Monitor contract performance
- Plan mainnet deployment after audit

## Mainnet Deployment

**DO NOT deploy to mainnet without:**
- Complete security audit
- Thorough testing on testnet
- Community review
- Legal compliance check

---

For questions, open an issue or discussion on GitHub.
