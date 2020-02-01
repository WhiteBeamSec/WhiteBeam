#[cfg(target_os = "windows")]
use crate::platforms::windows as platform;
#[cfg(target_os = "linux")]
use crate::platforms::linux as platform;
#[cfg(target_os = "macos")]
use crate::platforms::macos as platform;
use crate::common::hash;
use crate::common::time;
use std::{env,
          error::Error,
          path::Path};
use rusqlite::{params, Connection};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct LogExecObject {
    pub program: String,
    pub hash: String,
    pub uid: u32,
    pub ts: u32,
    pub success: bool
}

#[derive(Deserialize)]
pub struct ConfigEntry {
    pub server_ip: String,
    pub server_key: String,
    pub server_type: String,
    pub enabled: String
}

pub struct WhitelistResult {
    pub program: String,
    pub allow_unsafe: bool,
    pub hash: String
}

pub fn get_config(conn: &Connection, config_param: String) -> String {
    // TODO: Log errors
    conn.query_row("SELECT config_value FROM config WHERE config_param = ?", params![config_param], |r| r.get(0))
        .expect("WhiteBeam: Could not query configuration")
}

pub fn get_dyn_whitelist(conn: &Connection) -> Result<Vec<WhitelistResult>, Box<dyn Error>> {
    let mut result_vec: Vec<WhitelistResult> = Vec::new();
    let mut stmt = conn.prepare("SELECT program, allow_unsafe, hash FROM whitelist")?;
    let result_iter = stmt.query_map(params![], |row| {
        Ok(WhitelistResult {
            program: row.get(0)?,
            allow_unsafe: row.get(1)?,
            hash: row.get(2)?
        })
    })?;
    for result in result_iter {
        result_vec.push(result?);
    }
    Ok(result_vec)
}

pub fn get_enabled(conn: &Connection) -> bool {
    get_config(conn, String::from("enabled")) == String::from("true")
}

pub fn get_valid_auth_string(conn: &Connection, auth: &str) -> bool {
    let auth_hash: String = hash::common_hash_string(auth);
    let console_secret_expiry: u32 = match get_config(conn, String::from("console_secret_expiry")).parse() {
        Ok(v) => v,
        Err(_e) => return false
    };
    let time_now = time::get_timestamp();
    if console_secret_expiry == 0 ||
       console_secret_expiry >= time_now {
            return get_config(conn, String::from("console_secret")) == String::from(auth_hash);
    }
    false
}

pub fn get_valid_auth_env(conn: &Connection) -> bool {
    match env::var("WB_AUTH") {
        Ok(val) => {
            get_valid_auth_string(conn, &val)
        }
        Err(_e) => {
            false
        }
    }
}

pub fn get_seen_nonce(conn: &Connection, nonce: &str) -> bool {
    delete_all_expired_nonce(conn);
    // TODO: Log errors
    let count: i64 = conn.query_row("SELECT count(*) FROM nonce_hist WHERE nonce = ?", params![nonce], |r| r.get(0))
                         .expect("WhiteBeam: Could not query nonce history");
    count > 0
}

pub fn insert_config(conn: &Connection, config_param: &str, config_value: &str) {
    let _res = conn.execute(
        "INSERT INTO config (config_param, config_value)
                  VALUES (?1, ?2)",
        params![config_param, config_value]
    );
}

pub fn insert_whitelist(conn: &Connection, program: &str, allow_unsafe: bool, hash: &str) {
    // TODO: Verify no duplicate value exists
    let _res = conn.execute(
        "INSERT INTO whitelist (program, allow_unsafe, hash)
                  VALUES (?1, ?2, ?3)",
        params![program, allow_unsafe, hash]
    );
}

pub fn insert_exec(conn: &Connection, exec: LogExecObject) {
    let _res = conn.execute(
        "INSERT INTO exec_logs (program, hash, uid, ts, success)
                  VALUES (?1, ?2, ?3, ?4, ?5)",
        params![exec.program, exec.hash, exec.uid, exec.ts, exec.success]
    );
}

pub fn update_config(conn: &Connection, config_param: &str, config_value: &str) {
    let _res = conn.execute(
        "UPDATE config
                  SET config_value = ?2
                  WHERE config_param = ?1",
        params![config_param, config_value]
    );
}

pub fn delete_whitelist(conn: &Connection, program: &str) {
    let _res = conn.execute("DELETE FROM whitelist WHERE program = ?1",
                 params![program]);
}

fn delete_all_expired_nonce(conn: &Connection) {
    let _res = conn.execute("DELETE FROM nonce_hist WHERE ts < strftime('%s','now')-3600",
                 rusqlite::NO_PARAMS);
}

fn db_init(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let _res = conn.execute(
        "CREATE TABLE config (
            id INTEGER PRIMARY KEY,
            config_param TEXT NOT NULL,
            config_value TEXT NOT NULL
         )",
        rusqlite::NO_PARAMS
    );
    let _res = conn.execute(
        "CREATE TABLE exec_logs (
            id INTEGER PRIMARY KEY,
            program TEXT NOT NULL,
            hash TEXT NOT NULL,
            uid UNSIGNED INTEGER NOT NULL,
            ts INTEGER NOT NULL,
            success BOOLEAN NOT NULL
         )",
        rusqlite::NO_PARAMS
    );
    let _res = conn.execute(
        "CREATE TABLE whitelist (
            id INTEGER PRIMARY KEY,
            program TEXT NOT NULL,
            allow_unsafe BOOLEAN DEFAULT FALSE,
            hash TEXT NOT NULL
         )",
        rusqlite::NO_PARAMS
    );
    let _res = conn.execute(
        "CREATE TABLE nonce_hist (
            id INTEGER PRIMARY KEY,
            nonce TEXT NOT NULL,
            ts INTEGER NOT NULL
         )",
        rusqlite::NO_PARAMS
    );
    let config_path: &Path = &platform::get_data_file_path("init.json");
    let init_config: bool = config_path.exists();
    if init_config {
        // TODO: Validate init.json, log errors
        let init_file = std::fs::File::open(config_path)?;
        let json: ConfigEntry = serde_json::from_reader(init_file)?;
        insert_config(conn, "server_ip", &json.server_ip);
        insert_config(conn, "server_key", &json.server_key);
        insert_config(conn, "server_type", &json.server_type);
        insert_config(conn, "enabled", &json.enabled);
        insert_config(conn, "console_secret", "undefined");
        insert_config(conn, "console_secret_expiry", "-1");
        std::fs::remove_file(config_path)?;
    } else {
        insert_config(conn, "server_ip", "undefined");
        insert_config(conn, "server_key", "undefined");
        insert_config(conn, "server_type", "undefined");
        insert_config(conn, "enabled", "false");
        insert_config(conn, "console_secret", "undefined");
        insert_config(conn, "console_secret_expiry", "-1");
    }
    Ok(())
}

pub fn db_open() -> Connection {
    let db_path: &Path = &platform::get_data_file_path("database.sqlite");
    // TODO: Log errors
    Connection::open(db_path).expect("WhiteBeam: Could not open database")
}

pub fn db_optionally_init() {
    let db_path: &Path = &platform::get_data_file_path("database.sqlite");
    let run_init: bool = !db_path.exists();
    let conn = db_open();
    if run_init {
        // TODO: Log errors
        db_init(&conn).expect("WhiteBeam: Failed to initialize database")
    }
}
