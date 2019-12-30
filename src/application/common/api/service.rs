// Database
use crate::common::db;
// Public key encryption and signatures
use crate::common::crypto;

fn set_console_secret(secret: &str) -> String {
    let conn: rusqlite::Connection = db::db_open();
    db::update_config(&conn, "console_secret", secret);
    String::from("OK")
}

fn invalid_request() -> String {
    // Avoid providing a cryptographic oracle
    String::from("OK")
}

// GET /service/public_key
pub fn public_key() -> impl warp::Reply {
    let client_public_key = crypto::get_client_public_key();
    return hex::encode(client_public_key);
}

// POST /service/encrypted
pub fn encrypted(crypto_box_object: crypto::CryptoBox) -> impl warp::Reply {
    let server_message = crypto::process_request(crypto_box_object);
    match server_message.action.as_ref() {
        "set_console_secret" => set_console_secret(&server_message.parameters[0]),
        "invalid" => invalid_request(),
        _ => invalid_request()
    }
}
