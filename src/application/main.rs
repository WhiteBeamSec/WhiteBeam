use clap::{Arg, App, AppSettings};
use cli_table::{format::{CellFormat, Justify},
                Cell, Row, Table};
use std::env;
use std::process::Command;

pub mod platforms;
// Platform independent features
pub mod common;

fn run_auth() {
    // TODO: Log
    let password = rpassword::read_password_from_tty(Some("Password: ")).unwrap();
    let conn: rusqlite::Connection = common::db::db_open();
    if !common::db::get_valid_auth_string(&conn, &password) {
            eprintln!("WhiteBeam: Authorization failed");
            return;
    }
    println!("WhiteBeam: Authorization successful");
    env::set_var("WB_AUTH", &password);
}

fn run_add(program: &str, allow_unsafe: bool) {
    // TODO: Log
    let conn: rusqlite::Connection = common::db::db_open();
    if common::db::get_enabled(&conn) {
        if !common::db::get_valid_auth_env(&conn) {
            eprintln!("WhiteBeam: Authorization failed");
            return;
        }
    }
    // TODO: Whitelist more than individual files
    let hash: String = common::hash::common_hash_file(program);
    if hash == common::hash::hash_null() {
        eprintln!("WhiteBeam: No such file or directory");
        return;
    }
    println!("WhiteBeam: Adding {} (SHA3-256: {}) to whitelist", program, hash);
    common::db::insert_whitelist(&conn, program, allow_unsafe, &hash);
}

fn run_remove(program: &str) {
    // TODO: Log
    let conn: rusqlite::Connection = common::db::db_open();
    if common::db::get_enabled(&conn) {
        if !common::db::get_valid_auth_env(&conn) {
            eprintln!("WhiteBeam: Authorization failed");
            return;
        }
    }
    common::db::delete_whitelist(&conn, program);
}

fn run_list() {
    let conn: rusqlite::Connection = common::db::db_open();
    let whitelist = common::db::get_dyn_whitelist(&conn);
    let justify_right = CellFormat::builder().justify(Justify::Right).build();
    let bold = CellFormat::builder().bold(true).build();
    let mut table_vec: Vec<Row> = Vec::new();
    table_vec.push(Row::new(vec![
        Cell::new("Path", bold),
        Cell::new("Unsafe Env", bold)
    ]));
    for result in whitelist {
        table_vec.push(Row::new(vec![
                Cell::new(&result.program, Default::default()),
                Cell::new(&result.allow_unsafe, justify_right)
            ]));
    }
    let table = Table::new(table_vec, cli_table::format::BORDER_COLUMN_TITLE);
    let _res = table.print_stdout();
}

fn run_service() {
    common::db::db_optionally_init();
    common::api::serve();
}

fn run_enable() {
    println!("WhiteBeam: Enabling WhiteBeam");
    let conn: rusqlite::Connection = common::db::db_open();
    common::db::update_config(&conn, "enabled", "true");
}

fn run_start() {
    println!("WhiteBeam: Starting WhiteBeam service");
    let program = env::current_exe().ok().unwrap();
    Command::new(program)
            .arg("--service")
            .spawn().expect("Child process failed to start.");
}

fn run_status() {
    if let Ok(_response) = common::http::get("http://127.0.0.1:11998/status")
                                .with_timeout(1)
                                .send() {
        println!("WhiteBeam: OK");
    } else {
        eprintln!("WhiteBeam: Failed to communicate with WhiteBeam service");
    }
}

fn main() {
    let matches = App::new("WhiteBeam")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(env!("CARGO_PKG_VERSION"))
        .about("Open source EDR ( https://github.com/noproto/WhiteBeam )")
        .arg(Arg::with_name("auth")
                 .long("auth")
                 .takes_value(false)
                 .help("Authenticate for access to privileged commands"))
        .arg(Arg::with_name("add")
                 .long("add")
                 .takes_value(true)
                 .help("Add a whitelisted path or executable (+auth when enabled)")
                 .value_name("path"))
        .arg(Arg::with_name("unsafe")
                 .long("unsafe")
                 .takes_value(false)
                 .help("Allow use of unsafe environment variables (with --add, +auth when enabled)"))
        .arg(Arg::with_name("remove")
                 .long("remove")
                 .takes_value(true)
                 .help("Remove a whitelisted path or executable (+auth when enabled)")
                 .value_name("path"))
        .arg(Arg::with_name("list")
                 .long("list")
                 .takes_value(false)
                 .help("View whitelist policy on this host"))
        /* TODO
        .arg(Arg::with_name("admin")
                 .long("admin")
                 .takes_value(false)
                 .help("Open administrative shell (+auth)"))
        */
        .arg(Arg::with_name("service")
                 .long("service")
                 .takes_value(false)
                 .hidden(true))
        .arg(Arg::with_name("enable")
                 .long("enable")
                 .takes_value(false)
                 .help("Enable application whitelisting"))
        /* TODO
        .arg(Arg::with_name("disable")
                 .long("disable")
                 .takes_value(false)
                 .help("Disable application whitelisting (+auth)"))
        */
        .arg(Arg::with_name("start")
                 .long("start")
                 .takes_value(false)
                 .help("Start the WhiteBeam service"))
        /* TODO
        .arg(Arg::with_name("stop")
                 .long("stop")
                 .takes_value(false)
                 .help("Stop the WhiteBeam service (+auth)"))
        */
        /* TODO
        .arg(Arg::with_name("baseline")
                 .long("baseline")
                 .takes_value(false)
                 .help("Print execution statistics for non-whitelisted binaries"))
        */
        .arg(Arg::with_name("status")
                 .long("status")
                 .takes_value(false)
                 .help("View status of the WhiteBeam client"))
        .get_matches();

    if matches.is_present("auth") {
        run_auth();
    } else if matches.is_present("add") {
        run_add(matches.value_of("add").unwrap(), matches.is_present("unsafe"));
    } else if matches.is_present("remove") {
        run_remove(matches.value_of("remove").unwrap());
    } else if matches.is_present("list") {
        run_list();
    } else if matches.is_present("service") {
        run_service();
    } else if matches.is_present("enable") {
        run_enable();
    } else if matches.is_present("start") {
        run_start();
    } else if matches.is_present("status") {
        run_status();
    }
}
