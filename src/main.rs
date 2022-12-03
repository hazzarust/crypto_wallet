use std::env;
use web3::types::Address;
use std::str::FromStr;
use tokio;
use anyhow::Result;
mod eth_wallet;
mod utils;
use dotenv; //loads enviornmental variables from .env file, mashes variables with the actual environmental variables provided by the OS.
            //environmental variables are essentially global variables and can be created and available for reference at a point in time. 



#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
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


    //ALLOWS US TO CONNECT TO GOERLI TESTNET
    // takes the environmental variable under .env "ALCHEMY_GOERLI" and stores it under endpoint
    // we then make a future to establish connection of the endpoint (which is saved under my .env file)
    //a websocket opens a connection to allow two users to communicate between the user and the server
    let endpoint = std::env::var("ALCHEMY_GOERLI_WS")?;
    let web3_con = eth_wallet::establish_web3_connection(&endpoint).await?;

    let block_number = web3_con.eth().block_number().await?; //allows eth methods to be called on web3_con, here we are retrieving current block number.
    println!("block number: {}", &block_number); //here we print to see if connection is working. 

    let balance = loaded_wallet.get_balance(&web3_con).await?;
    println!("Balance: {}", &balance);


    //create transaction object
    let transaction = eth_wallet::create_eth_transaction(Address::from_str("0xdA6b77Cbb14A789B5dc6Db1B4fA28212EedaC2d3")?, 0.001);
    //call sign and send which will use the transaction object above and my secret key to sign and send the transaction. 
    //Remembering that web3_con is just the connection that allows us to communicate with the blockchain. 
    let transaction_hash = eth_wallet::sign_and_send(&web3_con, transaction, &loaded_wallet.get_secret_key()?).await?;

    println!("transaction hash: {:?}", transaction_hash);

    Ok(())
}

