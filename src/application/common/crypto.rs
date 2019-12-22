#[cfg(target_os = "windows")]
use crate::platforms::windows as platform;
#[cfg(target_os = "linux")]
use crate::platforms::linux as platform;
#[cfg(target_os = "macos")]
use crate::platforms::macos as platform;
use crate::common::db;
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use serde_json::json;
#[allow(unused_imports)]
use std::{io::prelude::*,
          io::Read,
          io::Write as IoWrite,
          time::SystemTime,
          path::Path,
          fmt::Write as FmtWrite,
          num::ParseIntError};
use sodiumoxide::crypto::{box_,
                          box_::curve25519xsalsa20poly1305::*};

/*
Keys
*/

fn key_array_from_slice(bytes: &[u8]) -> [u8; SECRETKEYBYTES] {
    let mut array = [0; SECRETKEYBYTES];
    let bytes = &bytes[..array.len()]; // Panics if not enough data
    array.copy_from_slice(bytes);
    array
}

fn generate_client_private_key(save_path: &Path) {
    let (_public_key, private_key) = box_::gen_keypair();
    let private_key_bytes: &[u8] = private_key.as_ref();
    let mut key_file = platform::path_open_secure(save_path);
    key_file.write_all(private_key_bytes).unwrap();
}

fn get_server_public_key() -> PublicKey {
    let conn: rusqlite::Connection = db::db_open();
    let public_key_string: String = db::get_config(&conn, String::from("server_key"));
    let public_key_bytes: &[u8] = &hex::decode(&public_key_string).unwrap();
    PublicKey::from_slice(public_key_bytes).unwrap()
}

fn get_client_public_private_key() -> (PublicKey, SecretKey) {
    let key_file_path: &Path = &platform::get_data_file_path("client.key");
    let gen_key: bool = !key_file_path.exists();
    if gen_key {
        generate_client_private_key(key_file_path);
    }
    let mut key_file = std::fs::File::open(key_file_path).unwrap();
    let mut private_key_bytes: Vec<u8> = Vec::new();
    key_file.read_to_end(&mut private_key_bytes).unwrap();
    let private_key_array: [u8; SECRETKEYBYTES] = key_array_from_slice(&private_key_bytes);
    let private_key = SecretKey(private_key_array);
    let public_key = private_key.public_key();
    (public_key, private_key)
}

/*
Encoding
*/

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    expires: u32,
    version: String,
    action: String,
    parameters: Vec<String>
}

fn json_encode_message(action: String, parameters: Vec<String>) -> String {
    let expires = SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_secs() as u32;
    let version = env!("CARGO_PKG_VERSION").to_string();
    let message_object = Message {
        expires: expires,
        version: version,
        action: action,
        parameters: parameters
    };
    serde_json::to_string(&message_object).unwrap()
}

fn json_decode_message(json: String) -> Message {
    let message_object: Message = serde_json::from_str(&json).unwrap();
    message_object
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CryptoBox {
    pubkey: String,
    nonce: String,
    ciphertext: String
}

fn json_encode_crypto_box(pubkey: String, nonce: String, ciphertext: String) -> String {
    let crypto_box_object = CryptoBox {
        pubkey: pubkey,
        nonce: nonce,
        ciphertext: ciphertext
    };
    serde_json::to_string(&crypto_box_object).unwrap()
}

fn json_decode_crypto_box(json: String) -> CryptoBox {
    let crypto_box_object: CryptoBox = serde_json::from_str(&json).unwrap();
    crypto_box_object
}

/*
Encryption
*/

fn generate_sealed_ciphertext(plaintext: &[u8]) -> Vec<u8> {
    let (_public_key, private_key) = box_::gen_keypair();
    let server_public_key = get_server_public_key();
    let nonce = box_::gen_nonce();
    box_::seal(plaintext, &nonce, &server_public_key, &private_key)
}

fn nonce_array_from_slice(bytes: &[u8]) -> [u8; NONCEBYTES] {
    let mut array = [0; NONCEBYTES];
    let bytes = &bytes[..array.len()]; // Panics if not enough data
    array.copy_from_slice(bytes);
    array
}

fn decrypt_server_ciphertext(ciphertext: &[u8], nonce_bytes: &[u8]) -> Vec<u8> {
    let (client_public_key, client_private_key) = get_client_public_private_key();
    let server_public_key = get_server_public_key();
    let nonce_array: [u8; NONCEBYTES] = nonce_array_from_slice(nonce_bytes);
    let nonce = Nonce(nonce_array);
    box_::open(ciphertext, &nonce, &server_public_key, &client_private_key).unwrap()
}

fn verify_server_message() {
    // TODO: Verify nonce hasn't been seen in the past hour if valid server message
    // db::
    // Then, crypto_sign_open
    ()
}

pub fn process_server_message(request: String) {
    // JSON decode
    // Verify server message
    ()
}

// TODO: For uploading logs
//pub fn generate_ciphertext() {
//    ()
//}
