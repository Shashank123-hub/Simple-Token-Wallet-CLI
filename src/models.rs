// External Libraries
use serde::{Deserialize, Serialize};  //For serialization and deserialization
use chrono::{DateTime, Utc};          //For date and time handling
use std::collections::HashMap;        //For using hash maps
use uuid::Uuid;                       //For generating unique identifiers

// We define a struct to represent a user's wallet in our system.
// The derive attributes automatically implement important traits:
// Serialize and Deserialize for converting to/from JSON
// Debug for easy printing
// Clone for creating copies of the struct

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Wallet {
    pub id: Uuid,   
    // Unique identifier for the wallet
    pub balances: HashMap<String, f64>,
    // A hash map to store balances of different cryptocurrencies
    pub created_at: DateTime<Utc>,
    // Timestamp of when the wallet was created
    pub owner_name: Option<String>,
    // Optional name of the wallet owner
}


// Now we define a struct to represent a transaction in our system.
// Similar to the Wallet struct, we use derive attributes for serialization, deserialization, and debugging.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: Uuid,
    // Unique identifier for the transaction
    pub from_wallet_id: Uuid,
    // ID of the wallet from which the transaction is made
    pub to_wallet_id: Uuid,
    // ID of the wallet to which the transaction is made
    pub amount: f64,
    // Amount of cryptocurrency being transferred
    pub token: String,
    // Type of cryptocurrency being transferred (e.g., Bitcoin, Ethereum)
    pub created_at: DateTime<Utc>,
    // Timestamp of when the transaction was created
}


// Finally, we define a struct to represent the state of our application.
// This struct will hold the wallets and transactions in memory.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WalletSystem {
    pub wallets: HashMap<Uuid, Wallet>,
    // A hash map to store wallets, using their unique IDs as keys
    pub transactions: Vec<Transaction>,
    // A vector to store transactions in the order they were created
}


// The structs are defined, now we can implement methods for them to handle various operations.
impl Wallet {
    // Method to create a new wallet
    // Returns a new wallet instance (self) 
    pub fn new(owner_name: Option<String>) -> Self {
        Wallet {
            id: Uuid::new_v4(), 
            // Generate a new unique ID
            balances: HashMap::new(), 
            // Initialize an empty hash map for balances
            created_at: Utc::now(), 
            // Set the creation time to the current time
            owner_name, 
            // Set the owner name if provided
        }
    }

    // Method to add tokens to the wallet
    // Takes a token name and amount as parameters
    pub fn add_tokens(&mut self, token: &str, amount: f64) {

    let balance = self.balances.entry(token.to_string()).or_insert(0.0);
    // entry() looks up the token in the balances map
    // or_insert() inserts a new entry with a default value of 0.0 if the token is not found

    *balance += amount;
    // Dereference the balance to update the value
    }


    pub fn get_balance(&self, token: &str, amount: f64) -> f64 {
        // Method to get the balance of a specific token in the wallet
        // Takes a token name and amount as parameters
        match self.balances.get(token) {
            Some(&balance) => balance,
            // If the token is found, return its balance
            None => 0.0,
            // If the token is not found, return 0.0
        }
    }
}


//Now we implement methods for the Transaction struct
impl Transaction {
    // Method to create a new transaction
    pub fn new(from_wallet_id: Uuid, to_wallet_id: Uuid, amount: f64, token: String) -> Self {
        Transaction {
            id: Uuid::new_v4(), 
            // Generate a new unique ID for the transaction
            from_wallet_id,
            // Set the ID of the wallet from which the transaction is made
            to_wallet_id,
            // Set the ID of the wallet to which the transaction is made
            amount,
            // Set the amount being transferred
            token,
            // Set the type of cryptocurrency being transferred
            created_at: Utc::now(), 
            // Set the creation time to the current time
        }
    }
}


// Now we implement methods for the WSystem struct
impl WalletSystem {
    // Method to create a new system state
    pub fn new() -> Self {
        WalletSystem {
            wallets: HashMap::new(), 
            // Initialize an empty hash map for wallets
            transactions: Vec::new(), 
            // Initialize an empty vector for transactions
        }
    }
}