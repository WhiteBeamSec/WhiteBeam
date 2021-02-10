// TODO: Non-zero exit codes for all errors
use clap::{Arg, App, AppSettings};
use cli_table::{format::{Justify, Separator}, print_stdout, CellStruct, Cell, Style, Table, TableStruct, Color};
use std::{env,
          error::Error,
          ffi::OsStr,
          fmt::{self, Debug, Display},
          io::{self, Read},
          process::Command};

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
fn valid_auth() -> Result<bool, Box<dyn Error>> {
    // TODO: Log
    let conn: rusqlite::Connection = common::db::db_open(false)?;
    if common::db::get_prevention(&conn)? {
        if !common::db::get_valid_auth_env(&conn).unwrap_or(false) {
            return Ok(false);
        }
    }
    return Ok(true);
}

// Methods
fn run_add(program: &OsStr) -> Result<(), Box<dyn Error>> {
    if !valid_auth()? { return Err("WhiteBeam: Authorization failed".into()); }
    let conn: rusqlite::Connection = common::db::db_open(false)?;
    // TODO: Use hook class to determine how to store value
    let hash: String = common::hash::common_hash_file(program);
    if hash == common::hash::hash_null() {
        return Err("WhiteBeam: No such file or directory".into());
    }
    let program_str = program.to_str().ok_or(String::from("Invalid UTF-8 provided"))?;
    println!("WhiteBeam: Adding {} (SHA-512: {}) to whitelist", &program_str, hash);
    common::db::insert_whitelist(&conn, &program_str, &hash)
}

fn run_auth() -> Result<(), Box<dyn Error>> {
    // TODO: Log
    let password: String = rpassword::read_password_from_tty(Some("Password: "))?;
    let conn: rusqlite::Connection = common::db::db_open(false)?;
    if !common::db::get_valid_auth_string(&conn, &password)? {
        return Err("WhiteBeam: Authorization failed".into());
    }
    println!("WhiteBeam: Opening administrative shell");
    let mut command = Command::new("/bin/sh");
    if let Ok(mut child) = command.env("WB_AUTH", &password)
                                  .spawn() {
        child.wait()?;
        println!("WhiteBeam: Session closed");
    } else {
        return Err("WhiteBeam: Administrative shell failed to start".into());
    }
    Ok(())
}

fn run_baseline() -> Result<(), Box<dyn Error>> {
    // TODO: Refactor
    let conn: rusqlite::Connection = common::db::db_open(false)?;
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
    Ok(())
}

fn run_disable() -> Result<(), Box<dyn Error>> {
    if !valid_auth()? { return Err("WhiteBeam: Authorization failed".into()); }
    println!("WhiteBeam: Disabling hook class");
    let conn: rusqlite::Connection = common::db::db_open(false)?;
    // TODO: Disable hook class
    Ok(())
}

fn run_enable() -> Result<(), Box<dyn Error>> {
    if !valid_auth()? { return Err("WhiteBeam: Authorization failed".into()); }
    println!("WhiteBeam: Enabling hook class");
    let conn: rusqlite::Connection = common::db::db_open(false)?;
    // TODO: Enable hook class
    Ok(())
}

fn run_list(param: &OsStr) -> Result<(), Box<dyn Error>> {
    // TODO: Zero argument case
    // TODO: Add hook class
    let conn: rusqlite::Connection = common::db::db_open(false)?;
    let param_str = param.to_str().ok_or(String::from("Invalid UTF-8 provided"))?;
    let table_struct: TableStruct = match param_str {
        "whitelist" => {
            let table: Vec<Vec<CellStruct>> = common::db::get_whitelist(&conn).unwrap_or(Vec::new()).iter()
                                                .map(|entry| vec![
                                                    entry.id.clone().cell(),
                                                    entry.class.clone().cell(),
                                                    entry.path.clone().cell(),
                                                    entry.value.clone().cell()
                                                ])
                                                .collect();
            table.table()
                    .title(vec![
                        "ID".cell().bold(true),
                        "Class".cell().bold(true),
                        "Path".cell().bold(true),
                        "Value".cell().bold(true)
                    ])
                    .separator(
                        Separator::builder()
                        .title(Some(Default::default()))
                        .row(None)
                        .column(Some(Default::default()))
                        .build(),
                    )
        },
        "hooks" => {
            let table: Vec<Vec<CellStruct>> = common::db::get_hooks_pretty(&conn).unwrap_or(Vec::new()).iter()
                                                .map(|entry| vec![
                                                    entry.id.clone().cell(),
                                                    {
                                                        let enabled = entry.enabled.clone();
                                                        if enabled {
                                                            enabled.cell().foreground_color(Some(Color::Green))
                                                        } else {
                                                            enabled.cell().foreground_color(Some(Color::Red))
                                                        }
                                                    },
                                                    entry.class.clone().cell().justify(Justify::Center),
                                                    entry.library.clone().cell(),
                                                    entry.symbol.clone().cell(),
                                                    entry.args.clone().cell()
                                                ])
                                                .collect();
            table.table()
                    .title(vec![
                        "ID".cell().bold(true),
                        "Enabled".cell().bold(true),
                        "Class".cell().bold(true).justify(Justify::Center),
                        "Library".cell().bold(true),
                        "Symbol".cell().bold(true),
                        "Arguments".cell().bold(true)
                    ])
                    .separator(
                        Separator::builder()
                        .title(Some(Default::default()))
                        .row(None)
                        .column(Some(Default::default()))
                        .build(),
                    )
        },
        "rules" => {
            let table: Vec<Vec<CellStruct>> = common::db::get_rules_pretty(&conn).unwrap_or(Vec::new()).iter()
                                                .map(|entry| vec![
                                                    entry.library.clone().cell(),
                                                    entry.symbol.clone().cell(),
                                                    entry.arg.clone().cell(),
                                                    entry.actions.clone().cell()
                                                ])
                                                .collect();
            table.table()
                    .title(vec![
                        "Library".cell().bold(true),
                        "Symbol".cell().bold(true),
                        "Argument".cell().bold(true),
                        "Actions".cell().bold(true)
                    ])
                    .separator(
                        Separator::builder()
                        .title(Some(Default::default()))
                        .row(None)
                        .column(Some(Default::default()))
                        .build(),
                    )
        },
        _ => {
            return Err("WhiteBeam: Invalid parameter for 'list' argument".into());
        }
    };
    Ok(print_stdout(table_struct)?)
}

fn run_load(path: &OsStr) -> Result<(), Box<dyn Error>> {
   match valid_auth() {
       Ok(is_valid) => {
           if !is_valid {
               return Err("WhiteBeam: Authorization failed".into());
           }
       }
       Err(desc) => {
           let desc_str: String = desc.to_string();
           // Allow database to be initialized for the first time
           // TODO: Audit denial of service attacks against --load (e.g. max opened files, exhausting memory)
           if (desc_str != "No database file found") &&
              (desc_str != "Query returned no rows") {
               return Err(desc);
           }
       }
    }
    let conn: rusqlite::Connection = common::db::db_open(true)?;
    let path_str = path.to_str().ok_or(String::from("Invalid UTF-8 provided"))?;
    if (path_str == "stdin") || (path_str == "-") {
        println!("WhiteBeam: Loading SQL from standard input");
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        conn.execute_batch(&buffer)?;
        return Ok(());
    }
    // Try reading a file
    if let Ok(buffer) = std::fs::read_to_string(&path) {
        println!("WhiteBeam: Loading SQL from local file '{}'", path_str);
        conn.execute_batch(&buffer)?;
        return Ok(());
    }
    // Try loading from repository
    // TODO: Rewrite path if str is "Base"
    let repository = match common::db::get_setting(&conn, String::from("Repository")) {
        Ok(repo) => repo,
        // TODO: Package Schema, Default, and Essential
        Err(_) => String::from("https://github.com/WhiteBeamSec/SQL/blob/master")
    };
    let mut url_common: String = format!("{}/sql/common/{}.sql", repository, path_str);
    let mut url_platform: String = format!("{}/sql/platforms/{}/{}.sql", repository, std::env::consts::OS, path_str);
    if repository.starts_with("https://github.com/") {
        url_common.push_str("?raw=true");
        url_platform.push_str("?raw=true");
    }
    // TODO: Identify ourselves with a user agent
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    let response_common = client.get(&url_common).send()?;
    if response_common.status().is_success() {
        println!("WhiteBeam: Loading '{}' from repository", path_str);
        let buffer = response_common.text()?;
        conn.execute_batch(&buffer)?;
        return Ok(());
    }
    let response_platform = client.get(&url_platform).send()?;
    if response_platform.status().is_success() {
        println!("WhiteBeam: Loading '{}' from repository", path_str);
        let buffer = response_platform.text()?;
        conn.execute_batch(&buffer)?;
        return Ok(())
    } else {
        return Err("WhiteBeam: Failed to load SQL from all available sources".into());
    }
}

fn run_remove(id: u32) -> Result<(), Box<dyn Error>> {
    if !valid_auth()? { return Err("WhiteBeam: Authorization failed".into()); }
    let conn: rusqlite::Connection = common::db::db_open(false)?;
    common::db::delete_whitelist(&conn, id)
}

#[tokio::main]
async fn run_service() {
    //common::db::db_optionally_init();
    common::api::serve().await;
}

fn run_setting(param: &OsStr, value: Option<&OsStr>) -> Result<(), Box<dyn Error>> {
    if !valid_auth()? { return Err("WhiteBeam: Authorization failed".into()); }
    let conn: rusqlite::Connection = common::db::db_open(false)?;
    let param_str = param.to_str().ok_or(String::from("Invalid UTF-8 provided"))?;
    if value.is_none() {
        println!("{}", common::db::get_setting(&conn, String::from(param_str))?);
        return Ok(());
    }
    let mut val: String = match value.unwrap().to_str().ok_or(String::from("Invalid UTF-8 provided"))? {
        "mask" => {
            rpassword::read_password_from_tty(Some("Value: "))?
        },
        v => String::from(v)
    };
    if param == "RecoverySecret" {
        let hash: String = common::hash::common_hash_password(&val);
        val = hash;
    }
    common::db::update_setting(&conn, param_str, &val)
}

fn run_start() {
    println!("WhiteBeam: Starting WhiteBeam service");
    platform::start_service();
}

fn run_status() -> Result<(), Box<dyn Error>> {
    // TODO: Use ServicePort setting
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(1))
        .build()?;
    if let Ok(_response) = client.get("http://127.0.0.1:11998/status").send() {
        println!("WhiteBeam: OK");
    } else {
        eprintln!("WhiteBeam: Failed to communicate with WhiteBeam service");
    }
    Ok(())
}

fn run_stop() -> Result<(), Box<dyn Error>> {
    if !valid_auth()? { return Err("WhiteBeam: Authorization failed".into()); }
    println!("WhiteBeam: Stopping WhiteBeam service");
    platform::stop_service();
    Ok(())
}

pub struct MainError(Box<dyn Error>);

impl<E: Into<Box<dyn Error>>> From<E> for MainError {
    fn from(e: E) -> Self {
        MainError(e.into())
    }
}

impl Debug for MainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)?;
        let mut source = self.0.source();
        while let Some(error) = source {
            write!(f, "\nCaused by: {}", error)?;
            source = error.source();
        }
        Ok(())
    }
}

fn main() -> Result<(), MainError> {
    // TODO: List enabled/disabled hook classes or individual hooks
    let matches = App::new("WhiteBeam")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(env!("CARGO_PKG_VERSION"))
        .about("https://github.com/WhiteBeamSec/WhiteBeam")
        .arg(Arg::with_name("add")
                 .long("add")
                 .takes_value(true)
                 .help("Add a whitelist rule (+auth with Prevention)")
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
                 .help("Disable a class of hooks (+auth with Prevention)"))
        .arg(Arg::with_name("enable")
                 .long("enable")
                 .takes_value(true)
                 .help("Enable a class of hooks (+auth with Prevention)"))
        .arg(Arg::with_name("list")
                 .long("list")
                 .takes_value(true)
                 .help("List hooks, rules, or whitelist policy on this host"))
        .arg(Arg::with_name("load")
                 .long("load")
                 .takes_value(true)
                 .help("Load SQL from standard input, a file, or repository (+auth with Prevention)"))
        .arg(Arg::with_name("remove")
                 .long("remove")
                 .takes_value(true)
                 .help("Remove a whitelist rule by id (+auth with Prevention)")
                 .value_name("path"))
        .arg(Arg::with_name("service")
                 .long("service")
                 .takes_value(false)
                 .hidden(true))
        .arg(Arg::with_name("setting")
                 .long("setting")
                 .takes_value(true)
                 .multiple(true)
                 .help("Modify or view WhiteBeam client settings (+auth with Prevention)"))
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
                 .help("Stop the WhiteBeam service (+auth with Prevention)"))
        .get_matches();

    if matches.is_present("add") {
        run_add(matches.value_of_os("add").ok_or(String::from("WhiteBeam: Missing parameter for 'add' argument"))?)?;
    } else if matches.is_present("auth") {
        run_auth()?;
    } else if matches.is_present("baseline") {
        run_baseline()?;
    } else if matches.is_present("disable") {
        run_disable()?;
    } else if matches.is_present("enable") {
        run_enable()?;
    } else if matches.is_present("list") {
        run_list(matches.value_of_os("list").ok_or(String::from("WhiteBeam: Missing parameter for 'list' argument"))?)?;
    } else if matches.is_present("load") {
        run_load(matches.value_of_os("load").unwrap_or(OsStr::new("stdin")))?;
    } else if matches.is_present("remove") {
        run_remove(matches.value_of("remove").ok_or(String::from("WhiteBeam: Missing parameter for 'remove' argument"))?.parse::<u32>()?)?;
    } else if matches.is_present("service") {
        run_service();
    } else if matches.is_present("setting") {
        match matches.values_of_os("setting") {
            Some(vals) => {
                let mut vals_iter = vals.clone();
                // TODO: Refactor
                if vals_iter.len() == 2 {
                    // TODO: Error handling
                    let param: &OsStr = vals_iter.next().ok_or(String::from("Missing parameter for 'setting' argument"))?;
                    let value: &OsStr = vals_iter.next().ok_or(String::from("Missing value for 'setting' argument"))?;
                    run_setting(param, Some(value))?
                } else if vals_iter.len() == 1 {
                    let param: &OsStr = vals_iter.next().ok_or(String::from("Missing parameter for 'setting' argument"))?;
                    run_setting(param, None)?
                } else {
                    return Err("WhiteBeam: Insufficient parameters for 'setting' argument".into());
                }
            },
            None => {
                return Err("WhiteBeam: Missing parameters for 'setting' argument".into());
            }
        };
    } else if matches.is_present("start") {
        run_start();
    } else if matches.is_present("status") {
        run_status()?;
    } else if matches.is_present("stop") {
        run_stop()?;
    }
    Ok(())
}
