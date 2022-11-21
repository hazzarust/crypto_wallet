mod eth_wallet;

fn main() {
    let (secret_key, pub_key) = eth_wallet::generate_keypair(); //calls the generate_keypair function from eth_wallet and stores tuple 

    println!("secret key: {}", &secret_key.to_string());
    println!("Public key: {}", &pub_key.to_string());
}
