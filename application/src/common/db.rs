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

pub struct HookRow {
    pub id: i64,
    pub enabled: bool,
    pub class: String,
    pub library: String,
    pub symbol: String,
    pub args: Option<String>,
}

#[derive(Clone)]
pub struct WhitelistRow {
    pub class: String,
    pub id: i64,
    pub parent: String,
    pub path: String,
    pub value: String
}

#[derive(Clone)]
pub struct RuleRow {
    pub library: String,
    pub symbol: String,
    pub position: Option<i64>,
    pub action: String
}

/*
pub struct BaselineResult {
    pub log: String,
    pub total: u32
}
*/

pub fn get_setting(conn: &Connection, param: String) -> Result<String, Box<dyn Error>> {
    // TODO: Log errors
    Ok(conn.query_row("SELECT value FROM Setting WHERE param = ?", params![param], |r| r.get(0))?)
}

pub fn get_whitelist(conn: &Connection) -> Result<Vec<WhitelistRow>, Box<dyn Error>> {
    // TODO: Log errors
    let mut result_vec: Vec<WhitelistRow> = Vec::new();
    let mut stmt = conn.prepare("SELECT WhitelistClass.class, Whitelist.id, Whitelist.parent, Whitelist.path, Whitelist.value
                                 FROM Whitelist
                                 INNER JOIN WhitelistClass ON Whitelist.class = WhitelistClass.id")?;
    let result_iter = stmt.query_map(params![], |row| {
        Ok(WhitelistRow {
            class: row.get(0)?,
            id: row.get(1)?,
            parent: row.get(2)?,
            path: row.get(3)?,
            value: row.get(4)?
        })
    })?;
    for result in result_iter {
        result_vec.push(result?);
    }
    Ok(result_vec)
}

pub fn get_hooks_pretty(conn: &Connection) -> Result<Vec<HookRow>, Box<dyn Error>> {
    // TODO: Log errors
    let mut result_vec: Vec<HookRow> = Vec::new();
    let mut stmt = conn.prepare("SELECT Hook.id, Hook.enabled, HookClass.class, Hook.library || ' (' || HookLanguage.language || ')' AS library, Hook.symbol, GROUP_CONCAT('(' || Datatype.datatype || ') ' || Argument.name, ', ') AS args
                                 FROM Hook
                                 INNER JOIN HookClass ON Hook.class = HookClass.id
                                 INNER JOIN HookLanguage ON Hook.language = HookLanguage.id
                                 LEFT OUTER JOIN Argument ON Hook.id = Argument.hook
                                 LEFT OUTER JOIN Datatype ON Argument.datatype = Datatype.id
                                 WHERE Argument.parent = 0 OR Argument.parent IS NULL
                                 GROUP BY Hook.id
                                 ORDER BY Hook.id, Argument.position")?;
    let result_iter = stmt.query_map(params![], |row| {
        Ok(HookRow {
            id: row.get(0)?,
            enabled: row.get(1)?,
            class: row.get(2)?,
            library: row.get(3)?,
            symbol: row.get(4)?,
            args: row.get(5)?
        })
    })?;
    for result in result_iter {
        result_vec.push(result?);
    }
    Ok(result_vec)
}

pub fn get_rules_pretty(conn: &Connection) -> Result<Vec<RuleRow>, Box<dyn Error>> {
    // TODO: Log errors
    let mut result_vec: Vec<RuleRow> = Vec::new();
    let mut stmt = conn.prepare("SELECT Hook.library, Hook.symbol, Rule.position, Action.name
                                 FROM Rule
                                 INNER JOIN Action ON Rule.action = Action.id
                                 INNER JOIN Hook on Rule.hook = Hook.id
                                 ORDER BY Hook.id, Rule.id")?;
    let result_iter = stmt.query_map(params![], |row| {
        Ok(RuleRow {
            library: row.get(0)?,
            symbol: row.get(1)?,
            position: row.get(2)?,
            action: row.get(3)?
        })
    })?;
    for result in result_iter {
        result_vec.push(result?);
    }
    Ok(result_vec)
}

pub fn get_prevention(conn: &Connection) -> Result<bool, Box<dyn Error>> {
    Ok(get_setting(conn, String::from("Prevention"))? == String::from("true"))
}

pub fn get_service_port(conn: &Connection) -> Result<u16, Box<dyn Error>> {
    match get_setting(&conn, String::from("ServicePort")) {
        Ok(port) => Ok(port.parse().unwrap_or(11998)),
        // TODO: Log errors
        Err(_) => Ok(11998)
    }
}

pub fn get_valid_auth_string(conn: &Connection, auth: &str) -> Result<bool, Box<dyn Error>> {
    // TODO: Support more than ARGON2ID
    //let algorithm = get_setting(&conn, String::from("SecretAlgorithm"))?;
    let argon2 = argon2::Argon2::default();
    let console_secret = get_setting(conn, String::from("ConsoleSecret"))?;
    let recovery_secret = get_setting(conn, String::from("RecoverySecret"))?;
    let console_secret_pwhash: Option<argon2::PasswordHash> = match argon2::PasswordHash::new(&console_secret) {
        Ok(pwhash) => Some(pwhash),
        Err(_) => None
    };
    let recovery_secret_pwhash: Option<argon2::PasswordHash> = match argon2::PasswordHash::new(&recovery_secret) {
        Ok(pwhash) => Some(pwhash),
        Err(_) => None
    };
    let auth_string = auth.to_owned();
    let auth_bytes = auth_string.as_bytes();
    let console_secret_expiry: Option<u32> = match get_setting(conn, String::from("ConsoleSecretExpiry"))?.parse() {
        Ok(v) => Some(v),
        Err(_e) => None
    };
    let time_now = time::get_timestamp();
    if console_secret_expiry.is_some()
       && (console_secret_expiry.unwrap() == 0 || console_secret_expiry.unwrap() >= time_now)
       && console_secret_pwhash.is_some()
       && argon2::PasswordVerifier::verify_password(&argon2, auth_bytes, &console_secret_pwhash.unwrap()).is_ok() {
        return Ok(true)
    } else if recovery_secret_pwhash.is_some()
       && argon2::PasswordVerifier::verify_password(&argon2, auth_bytes, &recovery_secret_pwhash.unwrap()).is_ok() {
        return Ok(true)
    }
    Ok(false)
}

pub fn get_valid_auth_env(conn: &Connection) -> Result<bool, Box<dyn Error>> {
    get_valid_auth_string(conn, &env::var("WB_AUTH")?)
}

pub fn get_seen_nonce(conn: &Connection, nonce: &str) -> Result<bool, Box<dyn Error>> {
    // TODO: Log errors
    let count: i64 = conn.query_row("SELECT count(*) FROM NonceHistory WHERE nonce = ?", params![nonce], |r| r.get(0))?;
    Ok(count > 0)
}

// TODO: Dead code?
pub fn insert_setting(conn: &Connection, param: &str, value: &str) -> Result<usize, rusqlite::Error> {
    conn.execute("INSERT INTO Setting (param, value) VALUES (?1, ?2)", params![param, value])
}

pub fn insert_whitelist(conn: &Connection, class: &str, parent: &str, path: &str, value: &str) -> Result<usize, rusqlite::Error> {
    conn.execute("INSERT OR REPLACE INTO Whitelist (parent, path, value, class) VALUES (?1, ?2, ?3, (SELECT id from WhitelistClass WHERE class=?4))", params![parent, path, value, class])
}

pub fn update_setting(conn: &Connection, param: &str, value: &str) -> Result<usize, rusqlite::Error> {
    conn.execute("INSERT OR REPLACE INTO Setting (param, value) VALUES (?1, ?2)", params![param, value])
}

pub fn update_hook_class_enabled(conn: &Connection, class: &str, enabled: bool) -> Result<usize, rusqlite::Error> {
    conn.execute("UPDATE Hook SET enabled = ?2 WHERE class = (SELECT id from HookClass WHERE class=?1)", params![class, enabled])
}

pub fn delete_whitelist(conn: &Connection, id: u32) -> Result<usize, rusqlite::Error> {
    conn.execute("DELETE FROM Whitelist WHERE id = ?1", params![id])
}

pub fn db_open(force: bool) -> Result<Connection, String> {
    let db_path: &Path = &platform::get_data_file_path("database.sqlite");
    let no_db: bool = !db_path.exists();
    // TODO: Log instead?
    if no_db && !force {
        return Err("No database file found".to_string());
    }
    match Connection::open(db_path) {
        Ok(conn) => Ok(conn),
        Err(_e) => {
            return Err("Could not open database file".to_string());
        }
    }
}

pub fn db_update_realtime() -> Result<(), Box<dyn Error>> {
    let db_path: &Path = &platform::get_data_file_path("database.sqlite");
    let realtime_path: &Path = &platform::get_realtime_file_path("database.sqlite");
    let no_db: bool = !db_path.exists();
    if no_db {
        return Err("No database file found".into());
    }
    let db_path_realtime_temp: &Path = &platform::get_data_file_path("database.sqlite.tmp");
    std::fs::copy(db_path, db_path_realtime_temp)?;
    std::fs::rename(db_path_realtime_temp, realtime_path)?;
    Ok(())
}
