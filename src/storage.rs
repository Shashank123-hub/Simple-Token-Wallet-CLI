//importing dependancies
use crate::models::WalletSystem;

//tools for working with files
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

//tool to create custom error messages
use thiserror::Error;

//Defining custom errors
#[derive(Error, Debug)]

pub enum StorageErrors {
    //Error if the file can't be open or written
    #[error("IO error: {0}")]
    IOerror(#[from] io::Error),

    //Data format error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

impl Storage {
    //creating a new storage for a specified file
    pub fn new(file_path: &str) -> Self {
        Storage {
            file_path: file_path.to_string(),
        }
    }

    //Saving all the wallet data to a file
    pub fn save(&self, system: &WalletSystem) -> Result<(), StorageErrors> {
        // First, convert our wallet data to a JSON string
        // The ? means "if this fails, return the error"
        let json = serde_json::to_string_pretty(system)?;

        //Making sure folder exists before saving
        if let Some(parent) = Path::new(&self.file_path).parent() {
            fs::create_dir_all(parent)?;
        }

        // Create the file (or replace it if it already exists)
        let mut file = File::create(&self.file_path)?;
        
        // Write our wallet data to the file
        file.write_all(json.as_bytes())?;
        
        // Everything worked! Return success (that's what Ok(()) means)
        Ok(())
    }

    // This loads wallet data from a file
    pub fn load(&self) -> Result<WalletSystem, StorageErrors> {
        // First check if the file exists
        if !Path::new(&self.file_path).exists() {
            // If not, just start with a new empty wallet system
            return Ok(WalletSystem::new());
        }
        
        // Open the file so we can read it
        let mut file = File::open(&self.file_path)?;
        
        // This will hold the text from the file
        let mut contents = String::new();
        
        // Read the whole file into our string
        file.read_to_string(&mut contents)?;
        
        // Convert the JSON text back into wallet data
        let system = serde_json::from_str(&contents)?;
        
        // Return the loaded wallet data
        Ok(system)
    }
}