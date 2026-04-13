#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

#[contracttype]
#[derive(Clone)]
pub struct Milestone {
    pub id: u64,
    pub quest_id: u64,
    pub title: String,
    pub reward_amount: i128,
}

#[contracttype]
pub enum DataKey {
    MilestoneCounter,
    Milestone(u64, u64), // (quest_id, milestone_id)
    Milestones(u64),     // quest_id -> Vec<milestone_id>
    Completion(u64, u64, Address), // (quest_id, milestone_id, learner)
}

#[contract]
pub struct MilestoneContract;

#[contractimpl]
impl MilestoneContract {
    /// Create a milestone for a quest
    pub fn create_milestone(
        env: Env,
        owner: Address,
        quest_id: u64,
        title: String,
        reward_amount: i128,
    ) -> u64 {
        owner.require_auth();

        let counter_key = DataKey::MilestoneCounter;
        let milestone_id: u64 = env.storage().instance().get(&counter_key).unwrap_or(0) + 1;
        env.storage().instance().set(&counter_key, &milestone_id);

        let milestone = Milestone {
            id: milestone_id,
            quest_id,
            title,
            reward_amount,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Milestone(quest_id, milestone_id), &milestone);

        let mut milestones: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::Milestones(quest_id))
            .unwrap_or(Vec::new(&env));
        
        milestones.push_back(milestone_id);
        env.storage().persistent().set(&DataKey::Milestones(quest_id), &milestones);

        milestone_id
    }

    /// Verify milestone completion
    pub fn verify_completion(
        env: Env,
        owner: Address,
        quest_id: u64,
        milestone_id: u64,
        learner: Address,
    ) {
        owner.require_auth();

        env.storage()
            .persistent()
            .set(&DataKey::Completion(quest_id, milestone_id, learner), &true);
    }

    /// Get all milestones for a quest
    pub fn get_milestones(env: Env, quest_id: u64) -> Vec<Milestone> {
        let milestone_ids: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::Milestones(quest_id))
            .unwrap_or(Vec::new(&env));

        let mut milestones = Vec::new(&env);
        for id in milestone_ids.iter() {
            if let Some(milestone) = env
                .storage()
                .persistent()
                .get::<_, Milestone>(&DataKey::Milestone(quest_id, id))
            {
                milestones.push_back(milestone);
            }
        }
        milestones
    }

    /// Check if milestone is completed by learner
    pub fn is_completed(env: Env, quest_id: u64, milestone_id: u64, learner: Address) -> bool {
        env.storage()
            .persistent()
            .get(&DataKey::Completion(quest_id, milestone_id, learner))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_create_milestone() {
        let env = Env::default();
        let contract_id = env.register_contract(None, MilestoneContract);
        let client = MilestoneContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        let quest_id = 1;

        let milestone_id = client.create_milestone(
            &owner,
            &quest_id,
            &String::from_str(&env, "Complete Chapter 1"),
            &1000,
        );

        assert_eq!(milestone_id, 1);
    }

    #[test]
    fn test_verify_completion() {
        let env = Env::default();
        let contract_id = env.register_contract(None, MilestoneContract);
        let client = MilestoneContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        let learner = Address::generate(&env);
        let quest_id = 1;

        let milestone_id = client.create_milestone(
            &owner,
            &quest_id,
            &String::from_str(&env, "Complete Chapter 1"),
            &1000,
        );

        client.verify_completion(&owner, &quest_id, &milestone_id, &learner);
        assert!(client.is_completed(&quest_id, &milestone_id, &learner));
    }
}
