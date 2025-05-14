// Import the modules
mod cli;
mod models;
mod operations;
mod storage;

// Import the libraries
use clap::Parser;
use cli::{CLI, Commands};
use models::Wallet;
use operations::WError;
use storage::Storage;
use std::process;
use uuid::Uuid;

// Path where we'll save the data
const STORAGE_FILE: &str = "wallet_data.json";

fn main() {

    //parse CLI args using clap
    let cli = CLI::parse();

    //create the storage handler
    let storage = Storage::new(STORAGE_FILE);

    //loading existing wallet, or start with a fresh one
    let mut system = match storage.load() {
        Ok(sys) => sys,
        Err(err) => {
            //if loading fails, show an error
            eprintln!("Failed to load wallet data: {}", err);
            process::exit(1);
        }
    }
}