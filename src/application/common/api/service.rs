// Database
use crate::common::db;
// Public key encryption and signatures
use crate::common::crypto;

fn managed_initialize() -> String {
    // TODO
    String::from("OK")
}

fn set_console_secret() -> String {
    // TODO
    String::from("OK")
}

fn log_invalid_request() -> String {
    // TODO
    // Avoid providing a cryptographic oracle
    String::from("OK")
}

// GET /service/public_key
pub fn public_key() -> impl warp::Reply {
    let client_public_key = crypto::get_client_public_key();
    return hex::encode(client_public_key);
}

// GET /service/encrypted
pub fn encrypted(crypto_box_object: crypto::CryptoBox) -> impl warp::Reply {
    let server_message = crypto::process_request(crypto_box_object);
    match server_message.action.as_ref() {
        "managed_initialize" => managed_initialize(),
        "set_console_secret" => set_console_secret(),
        "invalid" => log_invalid_request(),
        _ => log_invalid_request()
    }
}
