// Description: This file contains the operations for the wallet system, including creating wallets, adding transactions, and getting wallet balances
use crate::models::{WSystem, Wallet, Transaction};
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
impl WalletSys {
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
        name: &str,
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
        if !self.wallet.contains_key(from_wallet_id) {
            return Err(WError::WalletNotFound(*from_wallet_id))
        }

        // Next to validate if reciever's wallet exist
        if !self.wallet.contains_key(to_wallet_id) {
            return Err(WError::WalletNotFound(*to_wallet_id))
        } 

        // Finally to check if the wallet have sufficient balance 
        // Safe because we just checked
        let from_wallet = self.wallets.get(from_wallet_id).unwrap();
        let current_bal = from_wallet.balances.get(token).unwrap_or(&0.0);
        // Unwrap(0.0) handles the case where token doesn't exists

        if *current_bal < amount {
            return Err(WalletError::InsufficientBalance {
                wallet_id: *from_wallet_id,
                token: token.to_string(),
                balance: *current_bal,
                amount,
            });
        }

        
    }
}
