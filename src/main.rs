extern crate blsttc;

use blsttc::{Ciphertext, DecryptionShare, SecretKeySet};
use std::collections::BTreeMap;
use std::str::from_utf8;

trait ToHex {
    fn to_hex(&self) -> String;
}

impl ToHex for Ciphertext {
    fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }
}

fn test_threshold_decryption() {
    let mut rng = rand::thread_rng();

    let sk_set = SecretKeySet::random(1, &mut rng);

    let sk_1 = sk_set.secret_key_share(0);
    let sk_2 = sk_set.secret_key_share(1);
    let sk_3 = sk_set.secret_key_share(2);

    println!("\nWe have three secret keys:\n");
    println!("\t{}", sk_1.reveal());
    println!("\t{}", sk_2.reveal());
    println!("\t{}", sk_3.reveal());

    let pk_set = sk_set.public_keys();
    let pk = pk_set.public_key();

    println!("\nThat together form a public key:\n");
    println!("\t{}", pk.to_hex());

    println!("\nI create a message:\n");
    let msg = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    let msg_str = from_utf8(msg).unwrap();
    println!("\t{}", msg_str);

    println!("\nThen encrypt it to the above public key:\n");
    let cipher = pk.encrypt(msg);
    println!("\t{}", cipher.to_hex());

    println!("\nI decrypt the message with two of the secret keys giving me these pieces:\n");
    let d1 = sk_1.decrypt_share(&cipher).unwrap();
    let d2 = sk_2.decrypt_share(&cipher).unwrap();
    println!("\t{}", hex::encode(d1.to_bytes()));
    println!("\t{}", hex::encode(d2.to_bytes()));

    println!("\nThen reassemble the decryption pieces:\n");

    let mut shares: BTreeMap<usize, DecryptionShare> = BTreeMap::new();
    shares.insert(0, d1.clone());
    shares.insert(1, d2.clone());

    let res = pk_set.decrypt(&shares, &cipher).unwrap();
    let res_str = from_utf8(res.as_slice()).unwrap();

    println!("\t{}", res_str);

    println!("\nI can also decrypt the message with one of the other secret keys giving me these pieces:\n");
    let d3 = sk_3.decrypt_share(&cipher).unwrap();
    println!("\t{}", hex::encode(d1.to_bytes()));
    println!("\t{}", hex::encode(d3.to_bytes()));

    println!("\nThen reassemble the decryption pieces:\n");

    let mut shares2: BTreeMap<usize, DecryptionShare> = BTreeMap::new();
    shares2.insert(0, d1.clone());
    shares2.insert(2, d3.clone());

    let res2 = pk_set.decrypt(&shares2, &cipher).unwrap();
    let res_str2 = from_utf8(res2.as_slice()).unwrap();

    println!("\t{}", res_str2);
}

fn main() {
    test_threshold_decryption()
}
