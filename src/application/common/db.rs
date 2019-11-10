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

fn db_init(conn: &Connection) {
    let _res = conn.execute(
        "CREATE TABLE config (
            id INTEGER PRIMARY KEY,
            parameter TEXT NOT NULL,
            value TEXT NOT NULL
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
}

// TODO: get_config

pub fn insert_config(conn: &Connection, config_param: String, config_value: String) {
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

pub fn open() -> Connection {
    #[cfg(target_os = "linux")]
    let dbpath: &Path = Path::new("/opt/WhiteBeam/data/database.sqlite");
    let run_init: bool = !dbpath.exists();
    let conn: Connection = Connection::open(dbpath).unwrap();
    if run_init {
        db_init(&conn)
    }
    conn
}
