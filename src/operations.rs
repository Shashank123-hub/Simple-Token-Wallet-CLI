// Description: This file contains the operations for the wallet system, including creating wallets, adding transactions, and getting wallet balances
use crate::models::{WalletSystem, Wallet, Transaction};
use thiserror::Error;
use uuid::Uuid;

// Error handling for the wallet system
#[derive(Error, Debug)]
pub enum WError {
    #[error("Wallet not found.")]
    WalletNotFound(Uuid),

    #[error("Insufficient funds for {token} in wallet {wallet_id}.")]
    InsufficientBalance {
        wallet_id: Uuid,
        token: String,
        balance: f64,
        amount: f64,
    },

    #[error("Cannot transfer to the same wallet.")]
    SelfTransfer,

    #[error("Invalid amount.")]
    InvalidAmount(f64),
}

// This will the main structure of the wallet system
impl WalletSystem {
    // Create a new wallet with a unique ID and initial balance
    pub fn add_wallet(&mut self, wallet: Wallet) {
        self.wallets.insert(wallet.id, wallet);
    }

    // Now we add a fn to get the wallet by id
    pub fn get_wallet(&self, id: &Uuid) -> Option<&Wallet> {
        self.wallets.get(id);
    }

    // A third fn, similar to the get_wallet but retures a modifiable reference to the wallet
    pub fn get_wallet_mut(&mut self, id: &Uuid) -> Option<&mut Wallet> {
        self.wallets.get_mut(id);
    }


    // Next will be core fn to add a transaction to a wallet
    // Returns Result<Transaction, WalletError> which will be:
    // - Ok(transaction) if successful
    // - Err(error) if anything goes wrong
    pub fn transfer (
        &mut self,
        from_wallet_id: &Uuid,
        to_wallet_id: &Uuid,
        token: &str,
        amount: f64,
    ) -> Result< Transaction, WError> {

        // We will validate different condition now
        // First to see that amount can't be transferred to the same wallet
        if from_wallet_id == to_wallet_id {
            return Err(WError::SelfTransfer);
        }

        // Next will be to check if the amount is not equal or less than zero (<0.0)
        if amount <= 0.0 {
            return Err(WError::InvalidAmount(amount));
        }

        // Next one to validate if sender's wallet exist
        // contains_key is a method on HashMap to check if a specific key exists in a map
        if !self.wallets.contains_key(from_wallet_id) {
            return Err(WError::WalletNotFound(*from_wallet_id))
        }

        // Next to validate if reciever's wallet exist
        if !self.wallets.contains_key(to_wallet_id) {
            return Err(WError::WalletNotFound(*to_wallet_id))
        } 

        // Finally to check if the wallet have sufficient balance 
        // Safe because we just checked
        let from_wallet = self.wallets.get(from_wallet_id).unwrap();
        let current_bal = from_wallet.balances.get(token).unwrap_or(&0.0);
        // Unwrap(0.0) handles the case where token doesn't exists

        if *current_bal < amount {
            return Err(WError::InsufficientBalance {
                wallet_id: *from_wallet_id,
                token: token.to_string(),
                balance: *current_bal,
                amount,
            });
        }

        //Create a new transaction record
        let transaction = Transaction::new(
            *from_wallet_id,
            *to_wallet_id,
            amount,
            token.to_string(),
        );

        // Update sender's balance (subtract tokens)
        // Note: We use a separate block with { } to limit the scope of the mutable borrow
        // This is necessary because Rust's borrow checker won't allow multiple mutable 
        // borrows of the same collection (self.wallets) at the same time
        {
            let from_wallet = self.wallets.get_mut(from_wallet_id).unwrap();
            let balance = from_wallet.balances.get_mut(token).unwrap();
            *balance -= amount;
        }
        
        // Update receiver's balance (add tokens)
        // Again in a separate block to avoid multiple mutable borrows
        {
            let to_wallet = self.wallets.get_mut(to_wallet_id).unwrap();
            // entry() and or_insert() handle the case where the token doesn't exist yet
            let balance = to_wallet.balances.entry(token.to_string()).or_insert(0.0);
            *balance += amount;
        }
        
        // Record the transaction in the system's history
        self.transactions.push(transaction.clone());
        
        // Return the transaction record to the caller
        Ok(transaction)
    }
    
    // Method to get all transactions related to a specific wallet
    // Returns a Vec containing references to matching transactions
    pub fn get_wallet_transactions(&self, wallet_id: &Uuid) -> Vec<&Transaction> {
        // Filter the transactions list to find ones where this wallet is either sender or receiver
        self.transactions
            .iter() // Create an iterator over all transactions
            .filter(|tx| tx.from_wallet_id == *wallet_id || tx.to_wallet_id == *wallet_id)
            .collect() // Collect the matching transactions into a Vec
    }
}
