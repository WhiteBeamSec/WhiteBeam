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

pub struct WhitelistResult {
    pub program: String,
    pub allow_unsafe: bool,
    pub hash: String
}

pub fn get_config(conn: &Connection, config_param: String) -> String {
    conn.query_row("SELECT config_value FROM config WHERE config_param = ?", params![config_param], |r| r.get(0)).unwrap()
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
    let auth: String = match env::var_os("WB_AUTH") {
        Some(val) => {
            val.into_string().unwrap()
        }
        None => {
            return false;
        }
    };
    get_valid_auth_string(conn, &auth)
}

pub fn db_open() -> Result<Connection, String> {
    let db_path: &Path = &platform::get_data_file_path("database.sqlite");
    let no_db: bool = !db_path.exists();
    if no_db {
        return Err("No database file found".to_string());
    }
    match Connection::open(db_path) {
        Ok(conn) => Ok(conn),
        Err(_e) => {
            return Err("Could not open database file".to_string());
        }
    }
}
