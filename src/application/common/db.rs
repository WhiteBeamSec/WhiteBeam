// TODO: Modify heavily ahead of 0.2 release

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

pub struct WhitelistResult {
    pub param: String,
    pub value: String
}

pub struct BaselineResult {
    pub program: String,
    pub total_blocked: u32
}

pub fn get_setting(conn: &Connection, param: String) -> String {
    // TODO: Log errors
    conn.query_row("SELECT value FROM Setting WHERE param = ?", params![param], |r| r.get(0))
        .expect("WhiteBeam: Could not query setting")
}

pub fn get_dyn_whitelist(conn: &Connection) -> Result<Vec<WhitelistResult>, Box<dyn Error>> {
    let mut result_vec: Vec<WhitelistResult> = Vec::new();
    let mut stmt = conn.prepare("SELECT param, value FROM Whitelist")?;
    let result_iter = stmt.query_map(params![], |row| {
        Ok(WhitelistResult {
            param: row.get(0)?,
            value: row.get(1)?
        })
    })?;
    for result in result_iter {
        result_vec.push(result?);
    }
    Ok(result_vec)
}

pub fn get_baseline(conn: &Connection) -> Result<Vec<BaselineResult>, Box<dyn Error>> {
    let mut result_vec: Vec<BaselineResult> = Vec::new();
    let mut stmt = conn.prepare("SELECT program, count(program) AS total_blocked
                                          FROM Log
                                          WHERE success=false
                                          GROUP BY program
                                          ORDER BY total_blocked DESC")?;
    let result_iter = stmt.query_map(params![], |row| {
        Ok(BaselineResult {
            program: row.get(0)?,
            total_blocked: row.get(1)?
        })
    })?;
    for result in result_iter {
        result_vec.push(result?);
    }
    Ok(result_vec)
}

pub fn get_prevention(conn: &Connection) -> bool {
    get_setting(conn, String::from("Prevention")) == String::from("true")
}

pub fn get_valid_auth_string(conn: &Connection, auth: &str) -> bool {
    let auth_hash: String = hash::common_hash_password(auth);
    let console_secret_expiry: u32 = match get_setting(conn, String::from("console_secret_expiry")).parse() {
        Ok(v) => v,
        Err(_e) => return false
    };
    let time_now = time::get_timestamp();
    if console_secret_expiry == 0 ||
       console_secret_expiry >= time_now {
            return get_setting(conn, String::from("console_secret")) == String::from(auth_hash);
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
    // TODO: Log errors
    let count: i64 = conn.query_row("SELECT count(*) FROM NonceHistory WHERE nonce = ?", params![nonce], |r| r.get(0))
                         .expect("WhiteBeam: Could not query nonce history");
    count > 0
}

pub fn insert_setting(conn: &Connection, param: &str, value: &str) {
    let _res = conn.execute(
        "INSERT INTO Setting (param, value)
                  VALUES (?1, ?2)",
        params![param, value]
    );
}

pub fn insert_whitelist(conn: &Connection, param: &str, value: &str) {
    // TODO: Verify no duplicate value exists
    let _res = conn.execute(
        "INSERT INTO Whitelist (param, value)
                  VALUES (?1, ?2)",
        params![param, value]
    );
}

pub fn insert_exec(conn: &Connection, exec: LogExecObject) {
    let _res = conn.execute(
        "INSERT INTO Log (program, hash, uid, ts, success)
                  VALUES (?1, ?2, ?3, ?4, ?5)",
        params![exec.program, exec.hash, exec.uid, exec.ts, exec.success]
    );
}

pub fn update_setting(conn: &Connection, param: &str, value: &str) {
    let _res = conn.execute(
        "INSERT OR REPLACE INTO Setting (param, value)
                  VALUES (?1, ?2)",
        params![param, value]
    );
}

pub fn delete_whitelist(conn: &Connection, id: u32) {
    let _res = conn.execute("DELETE FROM Whitelist WHERE id = ?1",
                 params![id]);
}

pub fn db_open() -> Result<Connection, String> {
    let db_path: &Path = &platform::get_data_file_path("database.sqlite");
    let no_db: bool = !db_path.exists();
    // TODO: Log instead?
    //if no_db {
    //    return Err("No database file found".to_string());
    //}
    match Connection::open(db_path) {
        Ok(conn) => Ok(conn),
        Err(_e) => {
            return Err("Could not open database file".to_string());
        }
    }
}
