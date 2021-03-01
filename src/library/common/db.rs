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
          path::Path,
          lazy::SyncLazy,
          sync::Mutex};
use rusqlite::{params, Connection};

// TODO: Hashmap/BTreemap to avoid race conditions, clean up of pthread_self() keys:
// Timestamp attribute, vec. len>0, check timestamp, pthread_equal, RefCell/Cell (?)
pub static HOOK_CACHE: SyncLazy<Mutex<Vec<HookRow>>> = SyncLazy::new(|| Mutex::new(vec![]));
pub static ARG_CACHE: SyncLazy<Mutex<Vec<ArgumentRow>>> = SyncLazy::new(|| Mutex::new(vec![]));
pub static WL_CACHE: SyncLazy<Mutex<Vec<WhitelistRow>>> = SyncLazy::new(|| Mutex::new(vec![]));
pub static RULE_CACHE: SyncLazy<Mutex<Vec<RuleRow>>> = SyncLazy::new(|| Mutex::new(vec![]));
pub static SET_CACHE: SyncLazy<Mutex<Vec<SettingRow>>> = SyncLazy::new(|| Mutex::new(vec![]));
// TODO: Cache rotation

#[derive(Clone)]
pub struct HookRow {
    pub language: String,
    pub library: String,
    pub symbol: String,
    pub id: i64
}

#[derive(Clone)]
pub struct ArgumentRow {
    pub hook: i64,
    pub parent: Option<i64>,
    pub id: i64,
    pub position: i64,
    pub real: usize,
    pub datatype: String,
    pub pointer: bool,
    pub signed: bool,
    pub variadic: bool,
    pub array: bool
}

#[derive(Clone)]
pub struct WhitelistRow {
    pub class: String,
    pub path: String,
    pub value: String
}

#[derive(Clone)]
pub struct RuleRow {
    pub arg: i64,
    pub action: String
}

#[derive(Clone)]
pub struct SettingRow {
    pub param: String,
    pub value: String
}

pub fn db_open() -> Result<Connection, String> {
    let db_path: &Path = &platform::get_data_file_path("database.sqlite");
    // TODO: Fix segmentation fault
    //let no_db: bool = !db_path.exists();
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

pub fn get_hook_view(conn: &Connection) -> Result<Vec<HookRow>, Box<dyn Error>> {
    // TODO: Log errors
    let mut result_vec: Vec<HookRow> = Vec::new();
    let mut stmt = conn.prepare("SELECT language, library, symbol, id FROM HookView")?;
    let result_iter = stmt.query_map(params![], |row| {
        Ok(HookRow {
            language: row.get(0)?,
            library: row.get(1)?,
            symbol: row.get(2)?,
            id: row.get(3)?
        })
    })?;
    for result in result_iter {
        result_vec.push(result?);
    }
    Ok(result_vec)
}

pub fn get_argument_view(conn: &Connection) -> Result<Vec<ArgumentRow>, Box<dyn Error>> {
    // TODO: Log errors
    let mut result_vec: Vec<ArgumentRow> = Vec::new();
    let mut stmt = conn.prepare("SELECT hook, parent, id, position, datatype, pointer, signed, variadic, array FROM ArgumentView")?;
    let result_iter = stmt.query_map(params![], |row| {
        Ok(ArgumentRow {
            hook: row.get(0)?,
            parent: match row.get(1) {
                Ok(id) => {Some(id)}
                Err(_) => {None}
            },
            id: row.get(2)?,
            position: row.get(3)?,
            real: 0 as usize,
            datatype: row.get(4)?,
            pointer: row.get(5)?,
            signed: row.get(6)?,
            variadic: row.get(7)?,
            array: row.get(8)?
        })
    })?;
    for result in result_iter {
        result_vec.push(result?);
    }
    Ok(result_vec)
}

pub fn get_whitelist_view(conn: &Connection) -> Result<Vec<WhitelistRow>, Box<dyn Error>> {
    // TODO: Log errors
    let mut result_vec: Vec<WhitelistRow> = Vec::new();
    let mut stmt = conn.prepare("SELECT class, path, value FROM WhitelistView")?;
    let result_iter = stmt.query_map(params![], |row| {
        Ok(WhitelistRow {
            class: row.get(0)?,
            path: row.get(1)?,
            value: row.get(2)?
        })
    })?;
    for result in result_iter {
        result_vec.push(result?);
    }
    Ok(result_vec)
}

pub fn get_rule_view(conn: &Connection) -> Result<Vec<RuleRow>, Box<dyn Error>> {
    // TODO: Log errors
    let mut result_vec: Vec<RuleRow> = Vec::new();
    let mut stmt = conn.prepare("SELECT arg, action FROM RuleView")?;
    let result_iter = stmt.query_map(params![], |row| {
        Ok(RuleRow {
            arg: row.get(0)?,
            action: row.get(1)?
        })
    })?;
    for result in result_iter {
        result_vec.push(result?);
    }
    Ok(result_vec)
}

pub fn get_setting_table(conn: &Connection) -> Result<Vec<SettingRow>, Box<dyn Error>> {
    // TODO: Log errors
    let mut result_vec: Vec<SettingRow> = Vec::new();
    let mut stmt = conn.prepare("SELECT param, value FROM Setting")?;
    let result_iter = stmt.query_map(params![], |row| {
        Ok(SettingRow {
            param: row.get(0)?,
            value: row.get(1)?
        })
    })?;
    for result in result_iter {
        result_vec.push(result?);
    }
    Ok(result_vec)
}

pub fn populate_cache() -> Result<(), Box<dyn Error>> {
    let conn = db_open()?;
    // Hook cache
    {
        let mut hook_cache_lock = HOOK_CACHE.lock()?;
        hook_cache_lock.clear();
        for row in get_hook_view(&conn)? {
            hook_cache_lock.push(row);
        }
    };
    // Argument cache
    {
        let mut arg_cache_lock = ARG_CACHE.lock()?;
        arg_cache_lock.clear();
        for row in get_argument_view(&conn)? {
            arg_cache_lock.push(row);
        }
    };
    // Whitelist cache
    {
        let mut wl_cache_lock = WL_CACHE.lock()?;
        wl_cache_lock.clear();
        for row in get_whitelist_view(&conn)? {
            wl_cache_lock.push(row);
        }
    };
    // Rule cache
    {
        let mut rule_cache_lock = RULE_CACHE.lock()?;
        rule_cache_lock.clear();
        for row in get_rule_view(&conn)? {
            rule_cache_lock.push(row);
        }
    };
    // Setting cache
    {
        let mut set_cache_lock = SET_CACHE.lock()?;
        set_cache_lock.clear();
        for row in get_setting_table(&conn)? {
            set_cache_lock.push(row);
        }
    };
    Ok(())
}

pub fn get_setting(param: String) -> String {
    // TODO: Log errors
    let set_cache_lock = SET_CACHE.lock().expect("WhiteBeam: Failed to lock mutex");
    let setting_option: Option<&SettingRow> = set_cache_lock.iter().find(|setting| setting.param == param);
    let setting_row_cloned: SettingRow = setting_option.expect("WhiteBeam: Lost track of environment").clone();
    (&setting_row_cloned.value).to_owned()
}

pub fn get_prevention() -> bool {
    get_setting(String::from("Prevention")) == String::from("true")
}

pub fn get_valid_auth_string(auth: String) -> bool {
    // TODO: Support more than ARGON2ID
    //let algorithm = get_setting(&conn, String::from("SecretAlgorithm"))?;
    let argon2 = argon2::Argon2::default();
    let console_secret = get_setting(String::from("ConsoleSecret"));
    let recovery_secret = get_setting(String::from("RecoverySecret"));
    let console_secret_pwhash: Option<argon2::PasswordHash> = match argon2::PasswordHash::new(&console_secret) {
        Ok(pwhash) => Some(pwhash),
        Err(_) => None
    };
    let recovery_secret_pwhash: Option<argon2::PasswordHash> = match argon2::PasswordHash::new(&recovery_secret) {
        Ok(pwhash) => Some(pwhash),
        Err(_) => None
    };
    let auth_bytes = auth.as_bytes();
    let console_secret_expiry: Option<u32> = match get_setting(String::from("ConsoleSecretExpiry")).parse() {
        Ok(v) => Some(v),
        Err(_e) => None
    };
    let time_now = time::get_timestamp();
    if console_secret_expiry.is_some()
       && (console_secret_expiry.unwrap() == 0 || console_secret_expiry.unwrap() >= time_now)
       && console_secret_pwhash.is_some()
       && argon2::PasswordVerifier::verify_password(&argon2, auth_bytes, &console_secret_pwhash.unwrap()).is_ok() {
        return true
    } else if recovery_secret_pwhash.is_some()
       && argon2::PasswordVerifier::verify_password(&argon2, auth_bytes, &recovery_secret_pwhash.unwrap()).is_ok() {
        return true
    }
    false
}

pub fn get_valid_auth_env() -> bool {
    match env::var("WB_AUTH") {
        Ok(val) => {
            get_valid_auth_string(val)
        }
        Err(_e) => {
            false
        }
    }
}
