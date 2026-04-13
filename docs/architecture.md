# Architecture Overview

Learna2Earna uses a three-contract architecture on Stellar Soroban for separation of concerns and modularity.

## Contract Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Frontend (React)                      │
│  - Wallet integration (Freighter)                           │
│  - Quest discovery & creation UI                            │
│  - Milestone tracking dashboard                             │
└──────────────┬──────────────┬──────────────┬────────────────┘
               │              │              │
               ▼              ▼              ▼
┌──────────────────┐ ┌──────────────────┐ ┌──────────────────┐
│  Quest Contract  │ │Milestone Contract│ │ Rewards Contract │
│                  │ │                  │ │                  │
│ - Create quests  │ │ - Add milestones │ │ - Fund pools     │
│ - Enroll users   │ │ - Verify work    │ │ - Distribute $   │
│ - Quest metadata │ │ - Track progress │ │ - Track earnings │
└──────────────────┘ └──────────────────┘ └──────────────────┘
               │              │              │
               └──────────────┴──────────────┘
                              │
                              ▼
                    ┌──────────────────┐
                    │  Stellar Ledger  │
                    │  (Persistent)    │
                    └──────────────────┘
```

## Why Three Contracts?

### 1. Separation of Concerns
Each contract has a single, well-defined responsibility:
- **Quest**: Identity and enrollment management
- **Milestone**: Progress tracking and verification
- **Rewards**: Financial operations

### 2. Independent Upgradability
Update reward distribution logic without touching quest management or milestone verification.

### 3. Smaller WASM Binaries
Each contract stays well under Soroban's 256KB limit, enabling faster deployment and lower costs.

### 4. Clear Security Boundaries
Authorization and permissions are scoped per contract, reducing attack surface.

## Data Flow

### Quest Creation Flow

```
1. Owner creates quest via Quest Contract
   ├─> Quest metadata stored on-chain
   └─> Quest ID generated

2. Owner funds quest via Rewards Contract
   ├─> Tokens transferred to contract
   └─> Pool balance updated

3. Owner adds milestones via Milestone Contract
   ├─> Milestone metadata stored
   └─> Linked to quest ID
```

### Learning & Earning Flow

```
1. Owner enrolls learner via Quest Contract
   └─> Learner address added to enrollees list

2. Learner completes work (off-chain)

3. Owner verifies completion via Milestone Contract
   └─> Completion status recorded

4. Owner distributes reward via Rewards Contract
   ├─> Pool balance decreased
   ├─> Learner earnings increased
   └─> Tokens transferred to learner
```

## Storage Patterns

### Instance Storage
- Counters (quest_id, milestone_id)
- Fast access, lower cost
- Suitable for frequently updated data

### Persistent Storage
- Quest metadata
- Milestone details
- Enrollment records
- Reward balances
- Long-term data with TTL management

### Temporary Storage
- Not currently used
- Future: Rate limiting, cooldowns

## Security Model

### Authorization
- `address.require_auth()` for all state-changing operations
- Owner-only functions for quest management
- Funder authorization for pool deposits

### Validation
- Quest existence checks before operations
- Sufficient balance checks before distributions
- Enrollment verification before rewards

### Future Enhancements
- Multi-signature support for large quests
- Time-locked rewards
- Dispute resolution mechanism

## Frontend Integration

The frontend orchestrates contract interactions:

1. **Wallet Connection**: Freighter API for Stellar account access
2. **Contract Clients**: Generated TypeScript bindings from WASM
3. **Transaction Building**: Stellar SDK for transaction construction
4. **State Management**: React hooks for UI state

## Scalability Considerations

- **Pagination**: Quest and milestone lists support offset/limit
- **Indexing**: Future: Off-chain indexer for analytics
- **Caching**: Frontend caches contract reads
- **Batch Operations**: Future: Bulk enrollment and distributions

## Testing Strategy

- **Unit Tests**: Each contract function tested in isolation
- **Integration Tests**: Multi-contract workflows tested
- **Frontend Tests**: UI component and hook testing
- **E2E Tests**: Future: Full user journey testing

---

For implementation details, see the contract source code in `contracts/`.
