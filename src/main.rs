use clap::{Arg, App, AppSettings};
use std::env;
use std::process::Command;

pub mod application;

fn main() {
    // TODO: Only view certain options unless user is authenticated
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
                 .help("Add a whitelisted path or executable (+auth)")
                 .value_name("path"))
        .arg(Arg::with_name("remove")
                 .long("remove")
                 .takes_value(true)
                 .help("Remove a whitelisted path or executable (+auth)")
                 .value_name("path"))
        .arg(Arg::with_name("list")
                 .long("list")
                 .takes_value(false)
                 .help("View whitelist policy on this host"))
        .arg(Arg::with_name("admin")
                 .long("admin")
                 .takes_value(false)
                 .help("Open administrative shell (+auth)"))
        .arg(Arg::with_name("service")
                 .long("service")
                 .takes_value(false)
                 .hidden(true))
        .arg(Arg::with_name("start")
                 .long("start")
                 .takes_value(false)
                 .help("Start the WhiteBeam service"))
        .arg(Arg::with_name("stop")
                 .long("stop")
                 .takes_value(false)
                 .help("Stop the WhiteBeam service (+auth)"))
        .arg(Arg::with_name("baseline")
                 .long("baseline")
                 .takes_value(false)
                 .help("Print execution statistics for non-whitelisted binaries (+auth)"))
        .arg(Arg::with_name("status")
                 .long("status")
                 .takes_value(false)
                 .help("View status of the WhiteBeam client"))
        .get_matches();

    if matches.is_present("service") {
        run_service();
    } else if matches.is_present("start") {
        run_start();
    } else if matches.is_present("status") {
        run_status();
    }
}

fn run_service() {
    application::common::api::serve();
}

fn run_start() {
    println!("Starting WhiteBeam server");

    // Rust documentation says this about current_exe():
    // "The output of this function should not be used in anything that might have security implications."
    //
    // 1. No alternative is provided
    // 2. There are no clear implications in this context
    //
    // We will reward (based on the current security reward policy) the first researcher that can
    // demonstrate bypassing WhiteBeam's whitelisting policy using current_exe().

    let prog = env::current_exe().ok().unwrap();
    Command::new(prog)
            .arg("--service")
            .spawn().expect("Child process failed to start.");
}

fn run_status() {
    println!("OK");
}
