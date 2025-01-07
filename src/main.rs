// Don't forget to git commit!!!!!!
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, AesGcm, Key, Nonce,
};
//https://backendengineer.io/aes-encryption-rust
fn main() {
    let plaintext = "hi".to_string();
    let key_str = "thiskeystrmustbe32charlongtowork".to_string();
    let ciphertext = encrypt_data(plaintext, key_str.clone());
    let plaintext = decrypt_data(key_str, ciphertext);
}

fn encrypt_data(plaintext: String, key_str: String) -> Vec<u8> {
    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let cipher = Aes256Gcm::new(key);
    let ciphertext = cipher.encrypt(&nonce, plaintext.as_bytes()).expect("err");
    let mut encrypted_data: Vec<u8> = nonce.to_vec();
    encrypted_data.extend_from_slice(&ciphertext);
    encrypted_data
}

fn decrypt_data(key_str: String, encrypted_data: Vec<u8>) -> String {
    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());
    let (nonce_arr, ciphertext) = encrypted_data.split_at(12);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_arr);
    let plaintext = cipher.decrypt(nonce, ciphertext).expect("err");
    let plaintext = String::from_utf8(plaintext).expect("Err");
    plaintext
}
