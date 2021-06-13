// TODO: Non-zero exit codes for all errors
// TODO: Update SettingsModified
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
fn run_add(class: &OsStr, path: &OsStr, value: Option<&OsStr>) -> Result<(), Box<dyn Error>> {
    // TODO: Single argument shortcut whitelist creation
    // TODO: Warn when static is being whitelisted
    if !valid_auth()? { return Err("WhiteBeam: Authorization failed".into()); }
    let conn: rusqlite::Connection = common::db::db_open(false)?;
    let class_string = String::from(class.to_str().ok_or(String::from("Invalid UTF-8 provided"))?);
    let path_string = String::from(path.to_str().ok_or(String::from("Invalid UTF-8 provided"))?);
    let algorithm = format!("Hash/{}", common::db::get_setting(&conn, String::from("HashAlgorithm"))?);
    let mut added_whitelist_entries: Vec<(String, String, String)> = vec![];
    // Convenience shortcuts occur when value is none
    match value {
        Some(v) => {
            let v_str: &str = v.to_str().ok_or(String::from("Invalid UTF-8 provided"))?;
            added_whitelist_entries.push((class_string.clone(), path_string.clone(), String::from(v_str)));
            println!("WhiteBeam: Allowing new {} ({}) for {}", &class_string, v_str, &path_string);
        },
        None => {
            let class_str: &str = &class_string;
            match class_str {
                "Filesystem/Path/Executable" => {
                    added_whitelist_entries.push((class_string.clone(), String::from("ANY"), path_string.clone()));
                    let hash: String = common::hash::process_hash(&mut std::fs::File::open(&path_string)?, &algorithm, None);
                    if common::hash::hash_is_null(&hash) {
                        return Err("WhiteBeam: No such file or directory".into());
                    }
                    added_whitelist_entries.push((algorithm.clone(), path_string.clone(), hash.clone()));
                    let all_library_paths: Vec<String> = platform::recursive_library_scan(&path_string, None, None).unwrap_or(vec![]).iter()
                                                        // Always allowed in Essential, no need to whitelist these
                                                        .filter(|lib| !(lib.contains("libc.so.6")
                                                                      ||lib.contains("libdl.so.2")
                                                                      ||lib.contains("libpthread.so.0")
                                                                      ||lib.contains("libgcc_s.so.1")
                                                                      ||lib.contains("librt.so.1")
                                                                      ||lib.contains("libm.so.6")
                                                                      ||lib.contains("libwhitebeam")
                                                                      ||lib.contains("ld-linux")))
                                                        .map(|lib| String::from(lib))
                                                        .collect();
                    let all_library_names: Vec<String> = all_library_paths.iter()
                                                                          .filter_map(|lib| std::path::Path::new(lib).file_name())
                                                                          .filter_map(|filename| filename.to_str())
                                                                          .map(|filename_str| String::from(filename_str))
                                                                          .collect();
                    for lib_name in all_library_names.iter() {
                        added_whitelist_entries.push((String::from("Filesystem/Path/Library"), path_string.clone(), String::from(lib_name)));
                    }
                    for lib_path in all_library_paths.iter() {
                        added_whitelist_entries.push((String::from("Filesystem/Path/Library"), path_string.clone(), String::from(lib_path)));
                    }
                    println!("WhiteBeam: Adding {} ({}: {}) to whitelist", &path_string, &algorithm[5..], &hash);
                },
                _ => { return Err("WhiteBeam: Missing parameters for 'add' argument".into()); }
            }
        }
    };
    for entry in added_whitelist_entries.iter() {
        let _res = common::db::insert_whitelist(&conn, &(entry.0), &(entry.1), &(entry.2));
    }
    Ok(())
}

fn run_auth() -> Result<(), Box<dyn Error>> {
    // TODO: Log
    let password: String = rpassword::read_password_from_tty(Some("Password: "))?;
    let conn: rusqlite::Connection = common::db::db_open(false)?;
    if !common::db::get_valid_auth_string(&conn, &password)? {
        return Err("WhiteBeam: Authorization failed".into());
    }
    println!("WhiteBeam: Opening administrative shell");
    let mut shell = Command::new("/bin/bash");
    shell.arg("--noprofile")
         .arg("--norc")
         .env("WB_AUTH", &password)
         // Theme
         .env("PS1", "\\[\x1b[34m\\][WhiteBeam \\[\x1b[37m\\]\\W\\[\x1b[34m\\]]\\[\x1b[37m\\]\\$ \\[\x1b(B\x1b[m\\]\\[\x1b(B\x1b[m\\]");
    if let Ok(mut child) = shell.spawn() {
        child.wait()?;
        println!("WhiteBeam: Session closed");
    } else {
        return Err("WhiteBeam: Administrative shell failed to start".into());
    }
    Ok(())
}

fn run_baseline() -> Result<(), Box<dyn Error>> {
    // TODO: Filter terminal escape sequences
    let conn: rusqlite::Connection = common::db::db_open(false)?;
    let table_struct: TableStruct = {
        let table: Vec<Vec<CellStruct>> = common::db::get_baseline(&conn).unwrap_or(Vec::new()).iter()
                                                .map(|entry| vec![
                                                    entry.log.clone().cell(),
                                                    entry.total.clone().cell(),
                                                ])
                                                .collect();
        table.table()
                .title(vec![
                    "Log".cell().bold(true),
                    "Total".cell().bold(true)
                ])
                .separator(
                    Separator::builder()
                    .title(Some(Default::default()))
                    .row(None)
                    .column(Some(Default::default()))
                    .build(),
                )
    };
    Ok(print_stdout(table_struct)?)
}

fn run_disable(class: &OsStr) -> Result<(), Box<dyn Error>> {
    if !valid_auth()? { return Err("WhiteBeam: Authorization failed".into()); }
    let conn: rusqlite::Connection = common::db::db_open(false)?;
    let class_str = class.to_str().ok_or(String::from("Invalid UTF-8 provided"))?;
    println!("WhiteBeam: Disabling hooks in '{}' class", class_str);
    let _res = common::db::update_hook_class_enabled(&conn, class_str, false);
    Ok(())
}

fn run_enable(class: &OsStr) -> Result<(), Box<dyn Error>> {
    if !valid_auth()? { return Err("WhiteBeam: Authorization failed".into()); }
    let conn: rusqlite::Connection = common::db::db_open(false)?;
    let class_str = class.to_str().ok_or(String::from("Invalid UTF-8 provided"))?;
    println!("WhiteBeam: Enabling hooks in '{}' class", class_str);
    let _res = common::db::update_hook_class_enabled(&conn, class_str, true);
    Ok(())
}

fn run_list(param: &OsStr) -> Result<(), Box<dyn Error>> {
    // TODO: Zero argument case
    // TODO: Add hook class
    let conn: rusqlite::Connection = common::db::db_open(false)?;
    let param_str = param.to_str().ok_or(String::from("Invalid UTF-8 provided"))?;
    let table_struct: TableStruct = match param_str {
        "whitelist" => {
            // TODO: Highlight path == "ANY" && value == "ANY" in red
            // TODO: Highlight writable directories containing an executable or library path in red
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
            // TODO: Columns for actions, separate tables for different classes (easier to follow)
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
    let base_version: String = platform::parse_os_version()?;
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
    let repository = match common::db::get_setting(&conn, String::from("Repository")) {
        Ok(repo) => repo,
        // TODO: Package Schema, Default, and Essential
        Err(_) => String::from("https://github.com/WhiteBeamSec/SQL/blob/master")
    };
    let mut url_common: String = format!("{}/sql/common/{}.sql", repository, path_str);
    let mut url_platform: String = format!("{}/sql/platforms/{}/{}.sql", repository, std::env::consts::OS, path_str);
    let mut url_base: String = format!("{}/sql/platforms/{}/base/{}.sql", repository, std::env::consts::OS, base_version);
    if repository.starts_with("https://github.com/") {
        url_common.push_str("?raw=true");
        url_platform.push_str("?raw=true");
        url_base.push_str("?raw=true");
    }
    // TODO: Identify ourselves with a user agent
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    if path_str == "Base" {
        // Base whitelist
        let response_base = client.get(&url_base).send()?;
        if response_base.status().is_success() {
            println!("WhiteBeam: Loading '{}' ({}) from repository", path_str, base_version);
            let buffer = response_base.text()?;
            conn.execute_batch(&buffer)?;
            return Ok(());
        }
        return Err("WhiteBeam: Failed to load SQL from all available sources".into());
    }
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
    let _res = common::db::delete_whitelist(&conn, id);
    Ok(())
}

#[tokio::main]
async fn run_service() -> Result<(), Box<dyn Error>> {
    //common::db::db_optionally_init();
    let conn: rusqlite::Connection = common::db::db_open(false)?;
    let service_port: u16 = common::db::get_service_port(&conn)?;
    common::api::serve(service_port).await;
    Ok(())
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
            let value_orig: String = rpassword::read_password_from_tty(Some("Value: "))?;
            let value_confirm: String = rpassword::read_password_from_tty(Some("Confirm: "))?;
            if value_orig == value_confirm {
                value_orig
            } else {
                return Err("WhiteBeam: Values did not match".into());
            }
        },
        v => String::from(v)
    };
    if ((param == "RecoverySecret") || (param == "ConsoleSecret")) && (val != String::from("undefined")) {
        let algorithm = format!("Hash/{}", common::db::get_setting(&conn, String::from("SecretAlgorithm"))?);
        let mut val_bytes: &[u8] = unsafe { &(val.as_bytes_mut()) };
        let hash: String = common::hash::process_hash(&mut val_bytes, &algorithm, None);
        val = hash;
    }
    let _res = common::db::update_setting(&conn, param_str, &val);
    Ok(())
}

fn run_start() {
    println!("WhiteBeam: Starting WhiteBeam service");
    platform::start_service();
}

fn run_status() -> Result<(), Box<dyn Error>> {
    let conn: rusqlite::Connection = common::db::db_open(false)?;
    let service_port: u16 = common::db::get_service_port(&conn)?;
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(1))
        .build()?;
    if let Ok(_response) = client.get(&format!("http://127.0.0.1:{}/status", service_port)).send() {
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
                 .multiple(true)
                 .help("Add policy to whitelist (+auth with Prevention)")
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
                 .value_name("id"))
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
        match matches.values_of_os("add") {
            Some(vals) => {
                let mut vals_iter = vals.clone();
                // TODO: Refactor
                if vals_iter.len() == 3 {
                    // TODO: Error handling
                    let class: &OsStr = vals_iter.next().ok_or(String::from("Missing class for 'add' argument"))?;
                    let path: &OsStr = vals_iter.next().ok_or(String::from("Missing path for 'add' argument"))?;
                    let value: &OsStr = vals_iter.next().ok_or(String::from("Missing value for 'add' argument"))?;
                    run_add(class, path, Some(value))?
                } else if vals_iter.len() == 2 {
                    let class: &OsStr = vals_iter.next().ok_or(String::from("Missing class for 'add' argument"))?;
                    let path: &OsStr = vals_iter.next().ok_or(String::from("Missing path for 'add' argument"))?;
                    run_add(class, path, None)?
                } else {
                    return Err("WhiteBeam: Insufficient parameters for 'add' argument".into());
                }
            },
            None => {
                return Err("WhiteBeam: Missing parameters for 'add' argument".into());
            }
        };
    } else if matches.is_present("auth") {
        run_auth()?;
    } else if matches.is_present("baseline") {
        run_baseline()?;
    } else if matches.is_present("disable") {
        run_disable(matches.value_of_os("disable").ok_or(String::from("WhiteBeam: Missing parameter for 'disable' argument"))?)?;
    } else if matches.is_present("enable") {
        run_enable(matches.value_of_os("enable").ok_or(String::from("WhiteBeam: Missing parameter for 'enable' argument"))?)?;
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
