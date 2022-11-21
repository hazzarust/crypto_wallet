use web3::ethabi::ethereum_types::Public;

use crate::eth_wallet::public_key_address;

use anyhow::Result;
mod eth_wallet;

fn main() -> Result<()> {
    let (secret_key, pub_key) = eth_wallet::generate_keypair(); //calls the generate_keypair function from eth_wallet and stores tuple 

    println!("secret key: {}", &secret_key.to_string());
    println!("Public key: {}", &pub_key.to_string());

    let pub_address = eth_wallet::public_key_address(&pub_key);
    println!("Public Key Address {}", pub_address);

    let crypto_wallet = eth_wallet::Wallet::new(&secret_key, &pub_key);
    println!("{:?}", crypto_wallet);

    crypto_wallet.save_to_file("crypto_wallet.json")?; //makes a json file with the crypto wallet struct inside

    let wallet_file_path = "crypto_wallet.json";
    println!("{:?}", wallet_file_path);

    let loaded_wallet = eth_wallet::Wallet::from_file(&wallet_file_path)?;
    println!("Loaded Wallet: {:?}", loaded_wallet);

    Ok(())
}

