mod generate;
mod load;
mod check;
use load::load_keypair;
use generate::generate_keypair;
use check::check_balance;

use std::error::Error;

fn main()-> Result<(), Box<dyn Error>>{

    // generate_keypair();
    load_keypair();


    // if let Err(e) = check_balance() {
    //     eprintln!("An error occurred: {}", e);
    //     // Optionally return an error code or handle the error in another way
    //     return Err(e);
    // }

    Ok(())
}
