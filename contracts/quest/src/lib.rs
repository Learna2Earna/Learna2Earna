#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

#[contracttype]
#[derive(Clone)]
pub struct Quest {
    pub id: u64,
    pub owner: Address,
    pub name: String,
    pub description: String,
    pub token_addr: Address,
}

#[contracttype]
pub enum DataKey {
    QuestCounter,
    Quest(u64),
    Enrollees(u64),
}

#[contract]
pub struct QuestContract;

#[contractimpl]
impl QuestContract {
    /// Create a new quest
    pub fn create_quest(
        env: Env,
        owner: Address,
        name: String,
        description: String,
        token_addr: Address,
    ) -> u64 {
        owner.require_auth();

        let counter_key = DataKey::QuestCounter;
        let quest_id: u64 = env.storage().instance().get(&counter_key).unwrap_or(0) + 1;
        env.storage().instance().set(&counter_key, &quest_id);

        let quest = Quest {
            id: quest_id,
            owner: owner.clone(),
            name,
            description,
            token_addr,
        };

        env.storage().persistent().set(&DataKey::Quest(quest_id), &quest);
        
        let enrollees: Vec<Address> = Vec::new(&env);
        env.storage().persistent().set(&DataKey::Enrollees(quest_id), &enrollees);

        quest_id
    }

    /// Add an enrollee to a quest
    pub fn add_enrollee(env: Env, quest_id: u64, enrollee: Address) {
        let quest: Quest = env.storage().persistent().get(&DataKey::Quest(quest_id)).unwrap();
        quest.owner.require_auth();

        let mut enrollees: Vec<Address> = env
            .storage()
            .persistent()
            .get(&DataKey::Enrollees(quest_id))
            .unwrap();
        
        enrollees.push_back(enrollee);
        env.storage().persistent().set(&DataKey::Enrollees(quest_id), &enrollees);
    }

    /// Get quest details
    pub fn get_quest(env: Env, quest_id: u64) -> Quest {
        env.storage().persistent().get(&DataKey::Quest(quest_id)).unwrap()
    }

    /// Check if user is enrolled
    pub fn is_enrollee(env: Env, quest_id: u64, user: Address) -> bool {
        let enrollees: Vec<Address> = env
            .storage()
            .persistent()
            .get(&DataKey::Enrollees(quest_id))
            .unwrap();
        
        for enrollee in enrollees.iter() {
            if enrollee == user {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_create_quest() {
        let env = Env::default();
        let contract_id = env.register_contract(None, QuestContract);
        let client = QuestContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        let token = Address::generate(&env);

        let quest_id = client.create_quest(
            &owner,
            &String::from_str(&env, "Learn Rust"),
            &String::from_str(&env, "Master Rust programming"),
            &token,
        );

        assert_eq!(quest_id, 1);
    }

    #[test]
    fn test_add_enrollee() {
        let env = Env::default();
        let contract_id = env.register_contract(None, QuestContract);
        let client = QuestContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        let learner = Address::generate(&env);
        let token = Address::generate(&env);

        let quest_id = client.create_quest(
            &owner,
            &String::from_str(&env, "Learn Rust"),
            &String::from_str(&env, "Master Rust programming"),
            &token,
        );

        client.add_enrollee(&quest_id, &learner);
        assert!(client.is_enrollee(&quest_id, &learner));
    }
}
