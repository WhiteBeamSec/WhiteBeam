#[cfg(target_os = "windows")]
use crate::application::platforms::windows as platform;
#[cfg(target_os = "linux")]
use crate::application::platforms::linux as platform;
#[cfg(target_os = "macos")]
use crate::application::platforms::macos as platform;
use crate::application::common::db;
// Needed for platform::path_open_secure
use serde_json::json;
use std::{io::prelude::*,
          io::Read,
          path::Path,
          fmt::Write,
          num::ParseIntError};
use sodiumoxide::crypto::{box_,
                          box_::curve25519xsalsa20poly1305::*};

// TODO: https://skinkade.github.io/rocket-encrypted-rest/
// using hex instead of base64

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

pub fn get_server_public_key() -> PublicKey {
    let conn: rusqlite::Connection = db::db_open();
    let public_key_string: String = db::get_config(&conn, String::from("server_key"));
    let public_key_bytes: &[u8] = &decode_hex(&public_key_string).unwrap();
    PublicKey::from_slice(public_key_bytes).unwrap()
}

//pub fn generate_client_public_key { ..
//    // To hex
//    let (ourpk, oursk) = box_::gen_keypair();
//    println!("{:?}", ourpk.as_ref());

pub fn generate_ciphertext() {
    let (ourpk, oursk) = box_::gen_keypair();
    let server_public_key = get_server_public_key();
    let nonce = box_::gen_nonce();
    let plaintext = b"some data";
    let ciphertext = box_::seal(plaintext, &nonce, &server_public_key, &oursk);
}
