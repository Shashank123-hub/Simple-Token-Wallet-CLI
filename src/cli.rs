
//We need these tools from the clap crate to build our command-line interface
use clap::{Parser, Subcommand};
//uuid for our wallet IDs
use uuid::Uuid;

// This is our main CLI struct
// It will parse command-line arguments into Rust types
#[derive(Parser)]

//Following set the name and desc of the application
#[command(name = "token_wallet")]
#[command(about = "A simple token wallet CLI", long_about = None)]

pub struct CLI {
    //We'll define some subcommands
    //User can type something like: create-wallet --name Alex

    #[command(subcommand)]
    pub command: Commands,
}

// This enum lists all the commands our app supports
// Each variant represents a different action the user can perform
#[derive(Subcommand)]
pub enum Commands {
    //Create the new wallet
    //This becomes help text shown to the users
    CreateWallet {
        //Optional owner name
        //short means to use -n, leng means to use --name
        #[arg(short, long)]
        name: Option<String>,  //This is optional
    },

    //List all wallets
    //This doesn't need additional args
    ListWallets,

    //View details of a wallet
    ViewWallet {
        //Wallet ID
        #[arg(short, long)]
        id: Uuid, //parse UUIDs from text
    },

    //View history for a wallet
    History {
        //Wallet ID
        #[arg(short, long)]
        wallet_id: Uuid,  //Wallet whose history 
    },

    /// Add tokens to a wallet (for testing)
    AddTokens {
        /// Wallet ID
        #[arg(short, long)]
        wallet_id: Uuid,
        
        /// Token name
        #[arg(short, long)]
        token: String, // The name of the token (like "BTC")
        
        /// Amount to add
        #[arg(short, long)]
        amount: f64, // We use f64 for decimal numbers
    },

    /// Transfer tokens between wallets
    Transfer {
        /// Sender wallet ID
        #[arg(short, long)]
        from: Uuid, // The wallet sending tokens
        
        /// Receiver wallet ID
        #[arg(short, long)]
        to: Uuid, // The wallet receiving tokens
        
        /// Token name
        #[arg(short, long)]
        token: String,
        
        /// Amount to transfer
        #[arg(short, long)]
        amount: f64,
    },
}