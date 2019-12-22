#[cfg(target_os = "windows")]
use crate::platforms::windows as platform;
#[cfg(target_os = "linux")]
use crate::platforms::linux as platform;
#[cfg(target_os = "macos")]
use crate::platforms::macos as platform;
use std::path::Path;
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

pub fn get_config(conn: &Connection, config_param: String) -> String {
    conn.query_row("SELECT config_value FROM config WHERE config_param = ?", params![config_param], |r| r.get(0)).unwrap()
}

// TODO: get_seen_nonce (and corresponding nonce_hist table once datatype is determined)
//pub fn get_seen_nonce(conn: &Connection, nonce: String) -> bool {
//    conn.query_row("SELECT nonce FROM nonce_hist WHERE nonce = ?", params![nonce], |r| r.get(0)).unwrap()
//    // If results..
//    // Return bool
//}

pub fn insert_config(conn: &Connection, config_param: &str, config_value: &str) {
    let _res = conn.execute(
        "INSERT INTO config (config_param, config_value)
                  VALUES (?1, ?2)",
        params![config_param, config_value],
    );
}

pub fn insert_exec(conn: &Connection, exec: LogExecObject) {
    let _res = conn.execute(
        "INSERT INTO logs (program, hash, uid, ts, success)
                  VALUES (?1, ?2, ?3, ?4, ?5)",
        params![exec.program, exec.hash, exec.uid, exec.ts, exec.success],
    );
}

fn db_init(conn: &Connection) {
    let _res = conn.execute(
        "CREATE TABLE config (
            id INTEGER PRIMARY KEY,
            config_param TEXT NOT NULL,
            config_value TEXT NOT NULL
         )",
        rusqlite::NO_PARAMS,
    );
    let _res = conn.execute(
        "CREATE TABLE logs (
            id INTEGER PRIMARY KEY,
            program TEXT NOT NULL,
            hash TEXT NOT NULL,
            uid UNSIGNED INTEGER NOT NULL,
            ts INTEGER NOT NULL,
            success BOOLEAN NOT NULL
         )",
        rusqlite::NO_PARAMS,
    );
    let _res = conn.execute(
        "CREATE TABLE whitelist (
            id INTEGER PRIMARY KEY,
            program TEXT NOT NULL,
            allow_unsafe BOOLEAN DEFAULT FALSE,
            hash TEXT NOT NULL
         )",
        rusqlite::NO_PARAMS,
    );
    let config_path: &Path = &platform::get_data_file_path("init.json");
    let init_config: bool = config_path.exists();
    if init_config {
        // TODO: Validate init.json
        let init_file = std::fs::File::open(config_path).unwrap();
        let json: ConfigEntry = serde_json::from_reader(init_file).unwrap();
        insert_config(conn, "server_ip", &json.server_ip);
        insert_config(conn, "server_key", &json.server_key);
        insert_config(conn, "server_type", &json.server_type);
        insert_config(conn, "enabled", &json.enabled);
        std::fs::remove_file(config_path).unwrap();
    } else {
        insert_config(conn, "server_ip", "none");
        insert_config(conn, "server_key", "none");
        insert_config(conn, "server_type", "none");
        insert_config(conn, "enabled", "false");
    }
}

pub fn db_open() -> Connection {
    let db_path: &Path = &platform::get_data_file_path("database.sqlite");
    let conn: Connection = Connection::open(db_path).unwrap();
    conn
}

pub fn db_optionally_init() {
    let db_path: &Path = &platform::get_data_file_path("database.sqlite");
    let run_init: bool = !db_path.exists();
    let conn = db_open();
    if run_init {
        db_init(&conn)
    }
}
