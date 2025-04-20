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