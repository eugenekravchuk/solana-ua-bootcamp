use solana_sdk::signature::{Keypair, Signer};
use bs58;

pub fn generate_keypair() {
    let keypair = Keypair::new();

    let public_key = keypair.pubkey();
    let private_key_bytes = keypair.secret().as_ref();

    let private_key = private_key_bytes.to_vec();

    println!("Public key: {}", public_key);
    println!("Private key: {}", bs58::encode(private_key).into_string());
}
