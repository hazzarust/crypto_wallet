use secp256k1::{
    rand::{rngs, SeedableRng},
    PublicKey, SecretKey,
};

pub fn generate_keypair() -> (SecretKey, PublicKey) { //return a tuple of a secret and public key 
    let secp = secp256k1::Secp256k1::new(); //create instance of secp256k1 and store under secp
    let mut rng = rngs::StdRng::seed_from_u64(111); //create a random number generator using a fixed seed number of 111
    secp.generate_keypair(&mut rng) //creates the key pair
}