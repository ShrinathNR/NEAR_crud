use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

// collection provides different ways to store data on blockchain more efficently
use near_sdk::collections::UnorderedMap;

near_sdk::setup_alloc!(); //allocators are used to obtain memory from the system at runtime
// wee_alloc (setup_alloc!() comes from this crate) is a allocator designed for web assembly
//to generate less than a kilobyte of uncompressed wen assembly

//  <!!!! Main Struct !!!!!!>

#[near_bindgen] //makes the following struct compatible with NEAR blockchain

#[derive(BorshDeserialize, BorshSerialize)] //while sending and receiving data 
//we need to serialize and deserialize in array of int

// the above to tags are attributes to convey specific information about different elements 
//like classes and methods..  during runtime

pub struct KeyValue {
    pairs:UnorderedMap<String, String>,
}

// 2. Default Implementation

//creating a default implementation for KeyValue struct
// with a deafult method which returns Self(current type)
impl Default for KeyValue {
    fn default() -> Self {
        Self {
            pairs: UnorderedMap::new(b"r".to_vec()) 
            //we need to convert the byte array(b"r") pass ID as Vec<u8> type
            //to_vec() method is used for the above purpose 
        }
    }
}

// <!!!!!! logics !!!!!>
//creating method for the struct KeyValue

//CRUD operation are implemnted using the methods given below

#[near_bindgen]
impl KeyValue {
    pub fn create_update(&mut self, key: String, value: String) {
        env::log(b"create or updated");
        self.pairs.insert(&key, &value);
    }

    pub fn read (&self, key: String) -> Option<String> {
        env::log(b"read");
        return self.pairs.get(&key);
    }

    pub fn delete (&mut self, key: String) {
        env::log(b"delete");
        self.pairs.remove(&key);
    }
}


// <!!!!!! tests !!!!!!>

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    //here we set up the context for the unit tests

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

    // Test 1
    #[test]
    fn create_read_pair () {
        let context  = get_context(vec![], false);
        testing_env!(context); //creates a testing environment for the context using the givem parameters

        //declared as mutable since we are adding or updating the elements
        let mut contract = KeyValue::default(); 
        contract.create_update("key1".to_string(), "hello".to_string());

        assert_eq!("hello".to_string(), contract.read("key1".to_string()).unwrap());
    }

    // Test 2
    #[test]
    fn read_nonexistent_pair() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = KeyValue::default();
        assert_eq!(None, contract.read("key1".to_string()));
    }
}
