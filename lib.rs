use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
//serialization is used to "bundle" the contract so that it can be put on chain (allows compilation to wasm)
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

//by default, creating a new cargo package will include main.rs
//this must be renamed to lib.rs so that the file will be treated as a library

#[near_bindgen] //wraps struct to generate a NEAR compatible smart contract
#[derive(BorshDeserialize, BorshSerialize)] 
pub struct StatusMessage { //In Rust, the struct and its data fields are defined first, and methods are implemented later
    level: u8, //map to store records
}

impl Default for StatusMessage { //default can be disabled, but is expected
    fn default() -> Self {
        Self {
            level: 5,
        }
    }
}

#[near_bindgen]
impl StatusMessage {
    pub fn set_status(&mut self, input: String) {
        let account_id = env::signer_account_id(); 


        match input.trim().parse::<u8>() {
            Ok(num) => level_result = num,
            Err(nan) => level_result = 0,
        }

        //env is used to access data such as the signer account id, account balance, or other smart-contract-specific data
        match level_result {
            1 => message_result = String::from("DEVCON 1! Project in imminent danger of failure, emergency work needed!"),
            2 => message_result = String::from("DEVCON 2! Development conditions unstable, seek assistance immediately"),
            3 => message_result = String::from("DEVCON 3! Project is currently behind schedule and could require overtime work. Brew another pot of coffee."),
            4 => message_result = String::from("DEVCON 4! Keep an eye out, the intern's tik tok habit could be slowing progress"),
            5 => message_result = String::from("DEVCON 5! The development environment is at peace, happy coding!"),
            _ => message_result = String::from("Please enter a level 1-5!"),
        }
        self.records.insert(&account_id, &message_result); //passing references to LookupMap::insert(AccountId, String), doesn't require copying data
        //inserts key-value pair into map
    }

    pub fn get_status(&self, account_id: String) -> Option<String> {
        return self.records.get(&account_id); //returns the value associated with the account_id key
    }
}

#[cfg(not(target_arch = "wasm32"))] //conditional compilation flags used to specify testing
#[cfg(test)] //test macro
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    //used to set up a simulation environment
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn set_get_message() {
        let context = get_context(vec![], false); //get VMNcontext
        testing_env!(context); //set testing environment
        let mut contract = StatusMessage::default(); //instantiate default StatusMessage struct
        contract.set_status("hello".to_string()); //set status to "hello"
        assert_eq!(
            "hello".to_string(),
            contract.get_status("bob_near".to_string()).unwrap()
        ); //asserts that contract status is equal to "hello", and passes test if true
    }

    #[test]
    fn get_nonexistent_message() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = StatusMessage::default();
        assert_eq!(None, contract.get_status("francis.near".to_string())); //important to note that None has a type in Rust
    }
    //Testing Process
    //1. set up context
    //2. instantiate struct
    //3. call methods
    //4. assert equality
}
