use ethers::{
    core::k256::ecdsa::VerifyingKey,
    utils::hex::{self, ToHexExt},
};
use rand::{thread_rng, Rng};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;

fn main() {
    let start = std::time::Instant::now();
    let target_address_prefix = "000000";
    let found = Arc::new(AtomicBool::new(false));

    let mut handles = vec![];

    for _ in 0..8 {
        let found_clone = found.clone();
        let handle = thread::spawn(move || {
            let mut rng = thread_rng();
            let mut i = 0;
            while !found_clone.load(Ordering::Relaxed) {
                let private_key: [u8; 32] = rng.gen();
                let public_key = PublicKey::from_secret_key(
                    &Secp256k1::new(),
                    &SecretKey::from_slice(&private_key).unwrap(),
                );
                let public_key_bytes = public_key.serialize();

                let verifying_key = VerifyingKey::from_sec1_bytes(&public_key_bytes).unwrap();
                let address = ethers::utils::public_key_to_address(&verifying_key);
                if address.encode_hex().starts_with(target_address_prefix) {
                    found_clone.store(true, Ordering::Relaxed);
                    println!("Found a vanity address after {} iterations:", i);
                    println!("private key: {}", hex::encode(&private_key));
                    println!("public key: {}", hex::encode(public_key_bytes));
                    println!("address: {}", address.encode_hex());
                    break;
                }
                i += 1;
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed = start.elapsed();
    println!("Time elapsed: {:?}", elapsed);
}
