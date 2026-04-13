#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    PoolBalance(u64), // quest_id -> balance
    UserEarnings(Address),
}

#[contract]
pub struct RewardsContract;

#[contractimpl]
impl RewardsContract {
    /// Fund a quest pool
    pub fn fund_quest(env: Env, funder: Address, quest_id: u64, amount: i128) {
        funder.require_auth();

        let current_balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::PoolBalance(quest_id))
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&DataKey::PoolBalance(quest_id), &(current_balance + amount));
    }

    /// Distribute reward to learner
    pub fn distribute_reward(
        env: Env,
        authority: Address,
        quest_id: u64,
        learner: Address,
        amount: i128,
    ) {
        authority.require_auth();

        let pool_balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::PoolBalance(quest_id))
            .unwrap_or(0);

        if pool_balance < amount {
            panic!("Insufficient pool balance");
        }

        env.storage()
            .persistent()
            .set(&DataKey::PoolBalance(quest_id), &(pool_balance - amount));

        let user_earnings: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::UserEarnings(learner.clone()))
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&DataKey::UserEarnings(learner), &(user_earnings + amount));
    }

    /// Get quest pool balance
    pub fn get_pool_balance(env: Env, quest_id: u64) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::PoolBalance(quest_id))
            .unwrap_or(0)
    }

    /// Get user total earnings
    pub fn get_user_earnings(env: Env, user: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::UserEarnings(user))
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_fund_quest() {
        let env = Env::default();
        let contract_id = env.register_contract(None, RewardsContract);
        let client = RewardsContractClient::new(&env, &contract_id);

        let funder = Address::generate(&env);
        let quest_id = 1;

        client.fund_quest(&funder, &quest_id, &5000);
        assert_eq!(client.get_pool_balance(&quest_id), 5000);
    }

    #[test]
    fn test_distribute_reward() {
        let env = Env::default();
        let contract_id = env.register_contract(None, RewardsContract);
        let client = RewardsContractClient::new(&env, &contract_id);

        let funder = Address::generate(&env);
        let authority = Address::generate(&env);
        let learner = Address::generate(&env);
        let quest_id = 1;

        client.fund_quest(&funder, &quest_id, &5000);
        client.distribute_reward(&authority, &quest_id, &learner, &1000);

        assert_eq!(client.get_pool_balance(&quest_id), 4000);
        assert_eq!(client.get_user_earnings(&learner), 1000);
    }
}
