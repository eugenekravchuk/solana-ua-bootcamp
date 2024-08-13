use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use std::error::Error;
use std::str::FromStr;

pub fn check_balance() -> Result<(), Box<dyn Error>> {
    let public_key_str = "3s2Lffxm1cAMepju8uQAasX3fTMmoqKXuYaXAYDJwCeP";

    let url = "https://api.devnet.solana.com";
    let client = RpcClient::new(url.to_string());

    let public_key = Pubkey::from_str(public_key_str)?;

    let balance_in_lamports = client.get_balance(&public_key)?;

    let balance_in_sol = balance_in_lamports as f64 / LAMPORTS_PER_SOL as f64;

    println!("⚡️ Connected to devnet");
    println!("The balance of the wallet at address {} is: {}", public_key_str, balance_in_sol);

    Ok(())
}
