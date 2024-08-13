use solana_sdk::signature::{Keypair, Signer};
use std::env;
use std::process;
use dotenv::dotenv;
use serde_json;

pub fn load_keypair() {
    dotenv().ok();

    let private_key = match env::var("SECRET_KEY") {
        Ok(val) => val,
        Err(_) => {
            println!("Add SECRET_KEY to .env!");
            process::exit(1);
        }
    };

    let private_key_bytes: Vec<u8> = match serde_json::from_str(&private_key) {
        Ok(val) => val,
        Err(_) => {
            println!("Failed to parse SECRET_KEY!");
            process::exit(1);
        }
    };

    let keypair = match Keypair::from_bytes(&private_key_bytes) {
        Ok(val) => val,
        Err(_) => {
            println!("Invalid SECRET_KEY format!");
            process::exit(1);
        }
    };

    println!("Public key: {}", keypair.pubkey());
}