// Load OS-specific modules
use std::{env,
          ffi::OsStr,
          ffi::OsString,
          fs,
          os::unix::ffi::OsStrExt,
          path::Path,
          path::PathBuf,
          process::Command};

pub fn get_data_file_path(data_file: &str) -> PathBuf {
    let data_path: String = String::from("/opt/WhiteBeam/data/");
    let data_file_path = data_path + data_file;
    Path::new(&data_file_path).to_owned()
}

pub fn get_current_uid() -> u32 {
    unsafe { libc::getuid() }
}

pub fn get_env_path() -> OsString {
    match env::var_os("PATH") {
        Some(val) => {
            val
        }
        None => {
            // Use CS_PATH
            OsString::from("/bin:/usr/bin")
        }
    }
}

pub fn search_path(program: &OsStr) -> Option<PathBuf> {
    let env_path: OsString = get_env_path();
    let mut paths: Vec<PathBuf> = env::split_paths(&env_path).collect();
    if program.as_bytes().contains(&b'/') {
        match env::current_dir() {
            Ok(cwd) => paths.push(cwd),
            Err(_val) => () // TODO: Log errors
        }
    }
    for mut path in paths {
        path.push(program);
        if path.exists() && path.is_file() {
            return Some(path);
        }
    }
    None
}

pub fn check_build_environment() {
    let requirements: Vec<&OsStr> = vec!("cc",
                                         "rustup",
                                         "pkg-config").into_iter().map(|s| OsStr::new(s)).collect();
    for requirement in requirements {
        if search_path(requirement).is_none() {
            let missing_requirement = requirement.to_string_lossy();
            // Give general advice for how to satisfy the missing requirement
            if missing_requirement == "cc" {
                eprintln!("cc not found in PATH, consider running: apt update && apt install -y build-essential");
            } else if missing_requirement == "rustup" {
                eprintln!("rustup not found in PATH, consider running: wget -q --https-only --secure-protocol=TLSv1_2 https://sh.rustup.rs -O - | sh /dev/stdin -y && source $$HOME/.cargo/env");
            } else if missing_requirement == "pkg-config" {
                eprintln!("pkg-config not found in PATH, consider running: apt update && apt install -y pkg-config libssl-dev");
            } else {
                // Reserved for future dependencies
                eprintln!("{} not found in PATH", missing_requirement);
            }
            std::process::exit(1);
        }
    }
    let rustup_toolchains = Command::new(search_path(OsStr::new("rustup")).unwrap())
            .arg("toolchain")
            .arg("list")
            .output()
            .expect("Failed to execute rustup command");
    let rustup_toolchains_string = String::from_utf8_lossy(&rustup_toolchains.stdout);
    if !rustup_toolchains_string.contains("stable") {
        eprintln!("No stable Rust found in toolchain, consider running: rustup toolchain install stable");
        std::process::exit(1);
    } else if !rustup_toolchains_string.contains("nightly") {
        eprintln!("No nightly Rust found in toolchain, consider running: rustup toolchain install nightly");
        std::process::exit(1);
    }
}

pub fn run_install() {
    if get_current_uid() != 0 {
        let sudo_path = match search_path(OsStr::new("sudo")) {
            Some(path) => path,
            None => {
                eprintln!("Insufficient privileges for installation of WhiteBeam and no sudo present");
                return;
            }
        };
        let program = env::current_exe().expect("Failed to determine path to current executable");
        Command::new(sudo_path)
            .arg(program)
            .arg("install")
            .status().expect("Child process failed to start.");
        return;
    }
    println!("Installing");
    let service = r#"#!/bin/bash
# WhiteBeam service
# chkconfig: 345 20 80
# description: WhiteBeam service
# processname: whitebeam

SERVICE_PATH="/opt/WhiteBeam/"

SERVICE=whitebeam
SERVICEOPTS="--service"

NAME=whitebeam
DESC="WhiteBeam service"
PIDFILE=/opt/WhiteBeam/data/$NAME.pid
SCRIPTNAME=/etc/init.d/$NAME

case "$1" in
start)
	printf "%-50s" "Starting $NAME..."
	cd $SERVICE_PATH
	PID=`$SERVICE $SERVICEOPTS > /dev/null 2>&1 & echo $!`
	#echo "Saving PID" $PID " to " $PIDFILE
        if [ -z $PID ]; then
            printf "%s\n" "Fail"
        else
            echo $PID > $PIDFILE
            printf "%s\n" "Ok"
        fi
;;
status)
        printf "%-50s" "Checking $NAME..."
        if [ -f $PIDFILE ]; then
            PID=`cat $PIDFILE`
            if [ -z "`ps axf | grep ${PID} | grep -v grep`" ]; then
                printf "%s\n" "Process dead but pidfile exists"
            else
                echo "Running"
            fi
        else
            printf "%s\n" "Service not running"
        fi
;;
stop)
        printf "%-50s" "Stopping $NAME"
            PID=`cat $PIDFILE`
            cd $SERVICE_PATH
        if [ -f $PIDFILE ]; then
            kill -HUP $PID
            printf "%s\n" "Ok"
            rm -f $PIDFILE
        else
            printf "%s\n" "pidfile not found"
        fi
;;

restart)
  	$0 stop
  	$0 start
;;

*)
        echo "Usage: $0 {status|start|stop|restart}"
        exit 1
esac"#;
    fs::write("/etc/init.d/whitebeam", service).expect("Unable to add WhiteBeam service");
    // TODO: Use Rust instead of coreutils
    Command::new(search_path(OsStr::new("bash")).unwrap())
            .arg("-c")
            .arg("mkdir -p /opt/WhiteBeam/;
                  cp ./target/release/whitebeam /opt/WhiteBeam/whitebeam;
                  cp ./target/release/libwhitebeam.so /opt/WhiteBeam/libwhitebeam.so;
                  mkdir /opt/WhiteBeam/data/;
                  ln -s /opt/WhiteBeam/whitebeam /usr/local/bin/whitebeam;
                  chmod 775 /etc/init.d/whitebeam;
                  ln -s /etc/init.d/whitebeam /etc/rc3.d/S01whitebeam;
                  /etc/init.d/whitebeam start;
                  echo '/opt/WhiteBeam/libwhitebeam.so' | tee -a /etc/ld.so.preload")
            .status()
            .expect("Installation failed");
    println!("Installation complete");
}
