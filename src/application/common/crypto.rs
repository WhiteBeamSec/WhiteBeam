#[cfg(target_os = "windows")]
use crate::application::platforms::windows as platform;
#[cfg(target_os = "linux")]
use crate::application::platforms::linux as platform;
#[cfg(target_os = "macos")]
use crate::application::platforms::macos as platform;
use crate::application::common::db;
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use serde_json::json;
#[allow(unused_imports)]
use std::{io::prelude::*,
          io::Read,
          io::Write as IoWrite,
          path::Path,
          fmt::Write as FmtWrite,
          num::ParseIntError};
use sodiumoxide::crypto::{box_,
                          box_::curve25519xsalsa20poly1305::*};

// TODO: Verify nonce hasn't been seen in the past hour if valid server message

#[derive(Serialize, Deserialize, Debug)]
struct CryptoBox {
    pubkey: String,
    nonce: String,
    ciphertext: String
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b);
    }
    s
}

fn get_server_public_key() -> PublicKey {
    let conn: rusqlite::Connection = db::db_open();
    let public_key_string: String = db::get_config(&conn, String::from("server_key"));
    let public_key_bytes: &[u8] = &decode_hex(&public_key_string).unwrap();
    PublicKey::from_slice(public_key_bytes).unwrap()
}

fn verify_server_message() {
    ()
}

fn decrypt_server_ciphertext() {
    ()
}

fn generate_client_private_key(save_path: &Path) {
    let (_public_key, private_key) = box_::gen_keypair();
    let private_key_bytes: &[u8] = private_key.as_ref();
    let mut key_file = platform::path_open_secure(save_path);
    key_file.write_all(private_key_bytes).unwrap();
}

fn key_array_from_slice(bytes: &[u8]) -> [u8; 32] {
    let mut array = [0; 32];
    let bytes = &bytes[..array.len()]; // panics if not enough data
    array.copy_from_slice(bytes);
    array
}

fn read_client_keypair() -> (PublicKey, SecretKey) {
    let key_file_path: &Path = &platform::get_data_file_path("client.key");
    let gen_key: bool = !key_file_path.exists();
    if gen_key {
        generate_client_private_key(key_file_path);
    }
    let mut key_file = std::fs::File::open(key_file_path).unwrap();
    let mut private_key_bytes: Vec<u8> = Vec::new();
    key_file.read_to_end(&mut private_key_bytes).unwrap();
    let private_key_array: [u8; 32] = key_array_from_slice(&private_key_bytes);
    let private_key = SecretKey(private_key_array);
    let public_key = private_key.public_key();
    (public_key, private_key)
}

pub fn generate_sealed_ciphertext(plaintext: &[u8]) {
    let (public_key, private_key) = box_::gen_keypair();
    let server_public_key = get_server_public_key();
    let nonce = box_::gen_nonce();
    let ciphertext = box_::seal(plaintext, &nonce, &server_public_key, &private_key);
}

// TODO: For uploading logs
//pub fn generate_ciphertext() {
//    ()
//}
