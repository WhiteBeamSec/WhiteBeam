use clap::{Arg, App, AppSettings};
//use cli_table::{format::{CellFormat, Justify},
//                Cell, Row, Table};
use std::ffi::OsStr;
use std::env;
use std::io::{self, Read};
use std::process::Command;

pub mod platforms;
#[cfg(target_os = "windows")]
use platforms::windows as platform;
#[cfg(target_os = "linux")]
use platforms::linux as platform;
#[cfg(target_os = "macos")]
use platforms::macos as platform;
// Platform independent features
pub mod common;

// Support functions
fn valid_auth() -> bool {
    // TODO: Log
    let conn: rusqlite::Connection = common::db::db_open().expect("WhiteBeam: Could not open database");
    if common::db::get_protected(&conn) {
        if !common::db::get_valid_auth_env(&conn) {
            eprintln!("WhiteBeam: Authorization failed");
            return false;
        }
    }
    return true;
}

// Methods
fn run_add(program: &OsStr) {
    if !valid_auth() { return; }
    let conn: rusqlite::Connection = common::db::db_open().expect("WhiteBeam: Could not open database");
    // TODO: Use hook class to determine how to store value
    let hash: String = common::hash::common_hash_file(program);
    if hash == common::hash::hash_null() {
        eprintln!("WhiteBeam: No such file or directory");
        return;
    }
    let program_str = program.to_string_lossy();
    println!("WhiteBeam: Adding {} (SHA-512: {}) to whitelist", &program_str, hash);
    common::db::insert_whitelist(&conn, &program_str, &hash);
}

fn run_auth() {
    // TODO: Log
    let password = match rpassword::read_password_from_tty(Some("Password: ")) {
        Ok(p) => p,
        Err(_e) => {
            eprintln!("WhiteBeam: Could not read password");
            return;
        }
    };
    let conn: rusqlite::Connection = common::db::db_open().expect("WhiteBeam: Could not open database");
    if !common::db::get_valid_auth_string(&conn, &password) {
            eprintln!("WhiteBeam: Authorization failed");
            return;
    }
    println!("WhiteBeam: Opening administrative shell");
    let mut command = Command::new("/bin/sh");
    if let Ok(mut child) = command.env("WB_AUTH", &password)
                                  .spawn() {
        match child.wait() {
            Ok(_c) => (),
            Err(_e) => eprintln!("WhiteBeam: Session isn't running")
        };
        println!("WhiteBeam: Session closed");
    } else {
        eprintln!("WhiteBeam: Administrative shell failed to start");
    }
}

fn run_baseline() {
    // TODO: Refactor
    let conn: rusqlite::Connection = common::db::db_open().expect("WhiteBeam: Could not open database");
    //let whitelist = common::db::get_baseline(&conn).unwrap_or(Vec::new());
    /*
    let justify_right = CellFormat::builder().justify(Justify::Right).build();
    let bold = CellFormat::builder().bold(true).build();
    let mut table_vec: Vec<Row> = Vec::new();
    table_vec.push(Row::new(vec![
        Cell::new("Path", bold),
        Cell::new("Total Blocked", bold)
    ]));
    for result in whitelist {
        table_vec.push(Row::new(vec![
                Cell::new(&result.program, Default::default()),
                Cell::new(&result.total_blocked, justify_right)
            ]));
    }
    let table = match Table::new(table_vec, cli_table::format::BORDER_COLUMN_TITLE) {
        Ok(table_obj) => table_obj,
        Err(_e) => {
            eprintln!("WhiteBeam: Could not create table");
            return;
        }
    };
    let _res = table.print_stdout();
    */
}

fn run_disable() {
    if !valid_auth() { return; }
    println!("WhiteBeam: Disabling hook class");
    let conn: rusqlite::Connection = common::db::db_open().expect("WhiteBeam: Could not open database");
    // TODO: Disable hook class
}

fn run_enable() {
    if !valid_auth() { return; }
    println!("WhiteBeam: Enabling hook class");
    let conn: rusqlite::Connection = common::db::db_open().expect("WhiteBeam: Could not open database");
    // TODO: Enable hook class
}

fn run_list() {
    // TODO: Refactor
    let conn: rusqlite::Connection = common::db::db_open().expect("WhiteBeam: Could not open database");
    let whitelist = common::db::get_dyn_whitelist(&conn).unwrap_or(Vec::new());
    /*
    let bold = CellFormat::builder().bold(true).build();
    let mut table_vec: Vec<Row> = Vec::new();
    table_vec.push(Row::new(vec![
        Cell::new("Path", bold)
    ]));
    for result in whitelist {
        table_vec.push(Row::new(vec![
                Cell::new(&result.program, Default::default())
            ]));
    }
    let table = match Table::new(table_vec, cli_table::format::BORDER_COLUMN_TITLE) {
        Ok(table_obj) => table_obj,
        Err(_e) => {
            eprintln!("WhiteBeam: Could not create table");
            return;
        }
    };
    let _res = table.print_stdout();
    */
}

fn run_load(path: &OsStr) {
    //if !valid_auth() { return; }
    let conn: rusqlite::Connection = common::db::db_open().expect("WhiteBeam: Could not open database");
    let path_str = path.to_string_lossy();
    if (&path_str == "stdin") || (&path_str == "-") {
        println!("WhiteBeam: Loading SQL from standard input");
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).expect("WhiteBeam: Could not get handle to stdin");
        conn.execute_batch(&buffer);
        return;
    }
    // TODO: file/repository with std::fs::read_to_string and http.rs
    println!("WhiteBeam: Loading SQL from '{}'", &path_str);
}

fn run_remove(id: u32) {
    if !valid_auth() { return; }
    let conn: rusqlite::Connection = common::db::db_open().expect("WhiteBeam: Could not open database");
    common::db::delete_whitelist(&conn, id);
}

async fn run_service() {
    //common::db::db_optionally_init();
    common::api::serve().await;
}

fn run_setting(param: &str, value: &str) {
    if !valid_auth() { return; }
    let mut val: String = match value {
        "mask" => {
            let masked_value: String = match rpassword::read_password_from_tty(Some("Value: ")) {
                Ok(v) => v,
                Err(_e) => {
                    eprintln!("WhiteBeam: Could not read value");
                    return;
                }
            };
            masked_value
        },
        v => String::from(v)
    };
    if param == "RecoverySecret" {
        let hash: String = common::hash::common_hash_password(&val);
        val = hash;
    }
    let conn: rusqlite::Connection = common::db::db_open().expect("WhiteBeam: Could not open database");
    common::db::update_setting(&conn, param, &val);
}

fn run_start() {
    println!("WhiteBeam: Starting WhiteBeam service");
    platform::start_service();
}

fn run_status() {
    // TODO: Use ServicePort setting
    if let Ok(_response) = common::http::get("http://127.0.0.1:11998/status")
                                .with_timeout(1)
                                .send() {
        println!("WhiteBeam: OK");
    } else {
        eprintln!("WhiteBeam: Failed to communicate with WhiteBeam service");
    }
}

fn run_stop() {
    if !valid_auth() { return; }
    println!("WhiteBeam: Stopping WhiteBeam service");
    platform::stop_service();
}

#[tokio::main]
async fn main() {
    // TODO: List enabled/disabled hook classes or individual hooks
    let matches = App::new("WhiteBeam")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(env!("CARGO_PKG_VERSION"))
        .about("https://github.com/WhiteBeamSec/WhiteBeam")
        .arg(Arg::with_name("add")
                 .long("add")
                 .takes_value(true)
                 .help("Add a whitelist rule (+auth when protected)")
                 .value_name("path"))
        .arg(Arg::with_name("auth")
                 .long("auth")
                 .takes_value(false)
                 .help("Authenticate for access to privileged commands"))
        .arg(Arg::with_name("baseline")
                 .long("baseline")
                 .takes_value(false)
                 .help("View statistics of failed operations"))
        .arg(Arg::with_name("disable")
                 .long("disable")
                 .takes_value(true)
                 .help("Disable a class of hooks (+auth when protected)"))
        .arg(Arg::with_name("enable")
                 .long("enable")
                 .takes_value(true)
                 .help("Enable a class of hooks (+auth when protected)"))
        .arg(Arg::with_name("list")
                 .long("list")
                 .takes_value(false)
                 .help("View whitelist policy on this host"))
        .arg(Arg::with_name("load")
                 .long("load")
                 .takes_value(true)
                 .help("Load SQL from standard input, a file, or repository (+auth when protected)"))
        .arg(Arg::with_name("remove")
                 .long("remove")
                 .takes_value(true)
                 .help("Remove a whitelist rule by id (+auth when protected)")
                 .value_name("path"))
        .arg(Arg::with_name("service")
                 .long("service")
                 .takes_value(false)
                 .hidden(true))
        .arg(Arg::with_name("setting")
                 .long("setting")
                 .takes_value(true)
                 .help("Modify WhiteBeam client settings (+auth when protected)"))
        .arg(Arg::with_name("start")
                 .long("start")
                 .takes_value(false)
                 .help("Start the WhiteBeam service"))
        .arg(Arg::with_name("status")
                 .long("status")
                 .takes_value(false)
                 .help("View status of the WhiteBeam client"))
        .arg(Arg::with_name("stop")
                 .long("stop")
                 .takes_value(false)
                 .help("Stop the WhiteBeam service (+auth when protected)"))
        .get_matches();

    if matches.is_present("add") {
        match matches.value_of_os("add") {
            Some(path) => run_add(path),
            None => {
                    eprintln!("WhiteBeam: Missing parameter for 'add' argument");
                    return;
            }
        };
    } else if matches.is_present("auth") {
        run_auth();
    } else if matches.is_present("baseline") {
        run_baseline();
    } else if matches.is_present("disable") {
        run_disable();
    } else if matches.is_present("enable") {
        run_enable();
    } else if matches.is_present("list") {
        run_list();
    } else if matches.is_present("load") {
        match matches.value_of_os("load") {
            Some(path) => run_load(path),
            None => run_load(OsStr::new("stdin"))
        };
    } else if matches.is_present("remove") {
        match matches.value_of("remove") {
            Some(val) => {
                match val.parse::<u32>() {
                    Ok(id) => run_remove(id),
                    Err(_) => {
                        eprintln!("WhiteBeam: Invalid parameter for 'remove' argument")
                    }
                }
            },
            None => {
                    eprintln!("WhiteBeam: Missing parameter for 'remove' argument");
                    return;
            }
        };
    } else if matches.is_present("service") {
        run_service().await;
    } else if matches.is_present("setting") {
        match matches.values_of_os("setting") {
            Some(vals) => {
                let mut vals_iter = vals.clone();
                if vals_iter.len() == 2 {
                    // TODO: Error handling
                    let param: &str = vals_iter.next().expect("WhiteBeam: Could not read param")
                                               .to_str().expect("WhiteBeam: Param was invalid UTF-8");
                    let value: &str = vals_iter.next().expect("WhiteBeam: Could not read value")
                                               .to_str().expect("WhiteBeam: Value was invalid UTF-8");
                    run_setting(param, value)
                } else {
                    eprintln!("WhiteBeam: Insufficient parameters for 'setting' argument");
                    return;
                }
            },
            None => {
                    eprintln!("WhiteBeam: Missing parameters for 'setting' argument");
                    return;
            }
        };
    } else if matches.is_present("start") {
        run_start();
    } else if matches.is_present("status") {
        run_status();
    } else if matches.is_present("stop") {
        run_stop();
    }
}
