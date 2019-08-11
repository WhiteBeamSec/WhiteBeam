// Service provided by a module under main.rs:
// https://raw.githubusercontent.com/carllerche/tower-web/master/examples/json.rs
extern crate clap;

use clap::{Arg, App, AppSettings};

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
        .arg(Arg::with_name("start")
                 .long("start")
                 .takes_value(false)
                 .help("Start the WhiteBeam service"))
        .arg(Arg::with_name("stop")
                 .long("stop")
                 .takes_value(false)
                 .help("Stop the Whitebeam service (+auth)"))
        .arg(Arg::with_name("baseline")
                 .long("baseline")
                 .takes_value(false)
                 .help("Print execution statistics for non-whitelisted binaries (+auth)"))
        .arg(Arg::with_name("status")
                 .long("status")
                 .takes_value(false)
                 .help("View status of the WhiteBeam client"))
        .get_matches();
}
