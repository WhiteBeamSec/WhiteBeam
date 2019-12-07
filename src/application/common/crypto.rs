#[cfg(target_os = "windows")]
use crate::application::platforms::windows as platform;
#[cfg(target_os = "linux")]
use crate::application::platforms::linux as platform;
#[cfg(target_os = "macos")]
use crate::application::platforms::macos as platform;
use chrono::prelude::*;
use serde_json::json;
use std::io::prelude::*;
use std::io::Read;
use std::path::Path;
use ring::rand::SystemRandom;
use ring::signature::Ed25519KeyPair;

pub fn generate_client_ed25519_pkcs8() {
    let sys_rand = SystemRandom::new();
    let key_pkcs8 = Ed25519KeyPair::generate_pkcs8(&sys_rand)
      .expect("WhiteBeam: Failed to generate client PKCS#8 key");
    let key_pkcs8_bytes: &[u8] = key_pkcs8.as_ref();
    let key_file_path: &Path = &platform::get_data_file_path("client.key");
    let mut key_file = platform::path_open_secure(key_file_path);
    key_file.write_all(key_pkcs8_bytes).unwrap();
}

pub fn load_ed25519_pkcs8() -> Vec<u8> {
    let key_path: &Path = &platform::get_data_file_path("client.key");
    let gen_key: bool = !key_path.exists();
    if gen_key {
        generate_client_ed25519_pkcs8();
    }
    let mut key_file = std::fs::File::open(key_path).unwrap();
    let mut key_pkcs8_bytes: Vec<u8> = Vec::new();
    key_file.read_to_end(&mut key_pkcs8_bytes).unwrap();
    key_pkcs8_bytes
}

pub fn generate_paseto() -> String {
  let current_date_time = Utc::now();
  let dt = Utc.ymd(current_date_time.year() + 1, 7, 8).and_hms(9, 10, 11);
  let key_pkcs8_bytes = load_ed25519_pkcs8();
  let as_key = Ed25519KeyPair::from_pkcs8(&key_pkcs8_bytes).expect("Failed to parse keypair");

  let token = paseto::tokens::PasetoBuilder::new()
    .set_ed25519_key(as_key)
    .set_issued_at(None)
    .set_expiration(dt)
    .set_issuer(String::from("instructure"))
    .set_audience(String::from("wizards"))
    .set_jti(String::from("gandalf0"))
    .set_not_before(Utc::now())
    .set_subject(String::from("gandalf"))
    .set_claim(String::from("go-to"), json!(String::from("mordor")))
    .set_footer(String::from("key-id:gandalf0"))
    .build()
    .expect("Failed to construct paseto token w/ builder!");
  token
}

pub fn verify_paseto(token: String, server_key: Ed25519KeyPair) {
    let verified_token = paseto::tokens::validate_public_token(
      token,
      Some(String::from("key-id:gandalf0")),
      paseto::tokens::PasetoPublicKey::ED25519KeyPair(server_key),
    )
    .expect("Failed to validate token!");
    println!("{:?}", verified_token);
}
