#[cfg(target_os = "windows")]
use crate::platforms::windows as platform;
#[cfg(target_os = "linux")]
use crate::platforms::linux as platform;
#[cfg(target_os = "macos")]
use crate::platforms::macos as platform;
use std::path::Path;
use rusqlite::{params, Connection};

pub struct WhitelistResult {
    pub program: String,
    pub allow_unsafe: bool,
    pub hash: String
}

pub fn get_config(conn: &Connection, config_param: String) -> String {
    conn.query_row("SELECT config_value FROM config WHERE config_param = ?", params![config_param], |r| r.get(0)).unwrap()
}

pub fn get_dyn_whitelist(conn: &Connection) -> Vec<WhitelistResult> {
    let mut result_vec: Vec<WhitelistResult> = Vec::new();
    let mut stmt = conn.prepare("SELECT program, allow_unsafe, hash FROM whitelist").unwrap();
    let result_iter = stmt.query_map(params![], |row| {
        Ok(WhitelistResult {
            program: row.get(0).unwrap(),
            allow_unsafe: row.get(1).unwrap(),
            hash: row.get(2).unwrap()
        })
    }).unwrap();
    for result in result_iter {
        result_vec.push(result.unwrap());
    }
    result_vec
}

pub fn get_enabled(conn: &Connection) -> bool {
    get_config(conn, String::from("enabled")) == String::from("true")
}

pub fn db_open() -> Connection {
    let db_path: &Path = &platform::get_data_file_path("database.sqlite");
    let no_db: bool = !db_path.exists();
    if no_db {
        panic!("WhiteBeam: No database found");
    }
    Connection::open(db_path).unwrap()
}
