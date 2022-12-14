use crate::utils;
use anyhow::Result;
use secp256k1::{
    rand::{rngs, SeedableRng},
    PublicKey, SecretKey,
};
use serde::{Deserialize, Serialize};
use std::io::BufWriter;
use std::str::FromStr;
use std::{fs::OpenOptions, io::BufReader};
use tiny_keccak::keccak256;
use web3::{
    transports::WebSocket,
    types::{Address, U256, TransactionParameters},
    Web3,
};
use web3::types::H256;



#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet{
    pub secret_key: String,
    pub public_key: String,
    pub public_address: String, 

}
impl Wallet{
    pub fn new(secret_key: &SecretKey, public_key: &PublicKey) -> Self{
        let addr: Address = public_key_address(&public_key);
        Wallet{
            secret_key: secret_key.to_string(), 
            public_key: public_key.to_string(),
            public_address: format!{"{:?}", addr}, //we use this to get the full hex string otherwise it would be shortened

        }
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<()>{ //this is a object function 
        let file = OpenOptions::new().write(true).create(true).open(file_path)?; //OpenOptions allows us to create a file if it dosent exist yet or overwrite if it does. 
        let buf_writer = BufWriter::new(file);

        serde_json::to_writer_pretty(buf_writer, self)?; //writes a formatted JSON structure to the file, self is what we are calling on, buf writer keeps an in-memory buffer of data and writes it to an underlying writer in large, infrequent batches.

        Ok(())
    }

    pub fn from_file(file_path: &str) -> Result<Wallet>{
        let file = OpenOptions::new().read(true).open(file_path)?;
        let buf_reader = BufReader::new(file); // Bufreader performs large, infrequent reads on the underlying Read and maintains an in-memory buffer of the results.

        let wallet: Wallet = serde_json::from_reader(buf_reader)?; //tell serde_json that type is Wallet so it knows what to deserialize into
        Ok(wallet)
    }

    pub fn get_secret_key(&self) -> Result<SecretKey>{
        let secret_key = SecretKey::from_str(&self.secret_key)?; //gets the secret key of wallet struct
        Ok(secret_key)
    }

    pub fn get_public_key(&self) -> Result<PublicKey>{
        let public_key = PublicKey::from_str(&self.public_key)?; //gets the public key of wallet struct
        Ok(public_key)
    }

    pub async fn get_balance(&self, web3_connection: &Web3<WebSocket>) -> Result<f64>{
        let wallet_address = Address::from_str(&self.public_address)?; //saves our address from our json file under wallet_address. wallet_address gets its address from our Wallet struct address
        let balance = &web3_connection.eth().balance(wallet_address, None).await?; //connects to websocket address, will check balance of our wallet address.
        Ok(utils::wei_to_eth(*balance))
    }

}

//Default fills in the rest of the struct for us with default values, such as gas and gas price. 
//This struct lays out the transaction parameters
pub fn create_eth_transaction(to: Address, eth_value: f64) -> TransactionParameters{
    TransactionParameters { 
        to: Some(to),
        value: utils::eth_to_wei(eth_value),
        ..Default::default()
     }
}

//This function takes in a web3 connection reference, the transaction parameters and my secret key.
//It returns a H256 which is the transaction ID.
//we sign first, and then we send. 
//a U256 is a fixed-size uninterpreted hash type with 32 bytes (256 bits) size.
pub async fn sign_and_send(web3: &Web3<WebSocket>, transaction: TransactionParameters, secret_key: &SecretKey) -> Result<H256>{
    let signed = web3.accounts().sign_transaction(transaction, secret_key).await?;
    let transaction_result = web3.eth().send_raw_transaction(signed.raw_transaction).await?;
    println!("{}", transaction_result);
    Ok(transaction_result)
}

pub fn generate_keypair() -> (SecretKey, PublicKey) { //return a tuple of a secret and public key 
    let secp = secp256k1::Secp256k1::new(); //create instance of secp256k1 and store under secp
    let mut rng = rngs::JitterRng::new_with_timer(utils::get_nstime); //create a random number generator every time we run program based off current time
    secp.generate_keypair(&mut rng) //creates the key pair
}

pub fn public_key_address(public_key: &PublicKey) -> Address{ //will take in a reference of type PublicKey and return an Address
    let public_key = public_key.serialize_uncompressed(); //serialize pub key in uncompressed form to get byte encoded pair of values
    debug_assert_eq!(public_key[0], 0x04); //used to check if first byte is hex 4 or 0x04
    let hash = keccak256(&public_key[1..]); //we want to disinclude the 0x04 as its not part of the address, so we do public_key[1..]
    //keccak creates a 32-byte string of letters and numbers 

    Address::from_slice(&hash[12..]) //we only keep the last 20bytes, these are the least significant bytes
    //we return the Address type, aka H160, H160 is a hash type with 160 bits in length (20bytes) || 20bytes * 8 = 160 bits
}

pub async fn establish_web3_connection(url: &str) -> Result<Web3<WebSocket>> { //takes url string slice as parameter and returns Web3 socket connection
    let transport = web3::transports::WebSocket::new(url).await?; //makes a new websocket using url slice
    Ok(web3::Web3::new(transport)) //returns the web socket here

}