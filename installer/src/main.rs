// TODO: Cross platform, replace install.sh, add sqlite config for osquery/osquery pkgs, disable spinner if terminal is not interactive

use std::{env,
          ffi::OsStr,
          fs,
          path::PathBuf,
          process::Command};
pub mod common;
pub mod platforms;
#[cfg(target_os = "windows")]
use platforms::windows as platform;
#[cfg(target_os = "linux")]
use platforms::linux as platform;
#[cfg(target_os = "macos")]
use platforms::macos as platform;

static SPINNER_CHOICE: spinners::Spinners = spinners::Spinners::Dots;

// Minimal build program

pub fn pretty_bytes(num: f64) -> String {
  let negative = if num.is_sign_positive() { "" } else { "-" };
  let num = num.abs();
  let units = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
  if num < 1_f64 {
      return format!("{}{} {}", negative, num, "B");
  }
  let delimiter = 1000_f64;
  let exponent = std::cmp::min((num.ln() / delimiter.ln()).floor() as i32, (units.len() - 1) as i32);
  let pretty_bytes = format!("{:.2}", num / delimiter.powi(exponent)).parse::<f64>().unwrap() * 1_f64;
  let unit = units[exponent as usize];
  format!("{}{} {}", negative, pretty_bytes, unit)
}

fn build(args: Vec<String>) {
    // TODO: Consistent naming: binary and application
    platform::check_build_environment();
    if args.len() <= 2 {
        // By default, build both the release library and binary
        build(vec![String::from("whitebeam-installer"), String::from("build"), String::from("binary")]);
        build(vec![String::from("whitebeam-installer"), String::from("build"), String::from("library")]);
        return;
    }
    // TODO: Replace with https://github.com/rust-lang/cargo/blob/master/src/doc/src/reference/unstable.md#profile-strip-option once stabilized
    let message: &str;
    let mut cargo_command = Command::new("cargo");
    cargo_command.env("RUSTFLAGS", "-C link-arg=-s -Awarnings");
    let lib_target_path: PathBuf = PathBuf::from(format!("{}/target/release/libwhitebeam.so", env!("PWD")));
    let bin_target_path: PathBuf = PathBuf::from(format!("{}/target/release/whitebeam", env!("PWD")));
    let subcommand: &str = &(&args[2].to_lowercase());
    let current_target_path = match subcommand {
        "binary" => {
            message = "WhiteBeam: Building binary";
            cargo_command.args(&["+stable", "build", "--package", "whitebeam", "--bin", "whitebeam", "--release", "-q"]);
            bin_target_path
        },
        "library" => {
            message = "WhiteBeam: Building library";
            cargo_command.args(&["+nightly-2022-07-23", "build", "--package", "libwhitebeam", "--lib", "--release", "-q"]);
            lib_target_path
        },
        "binary-test" => {
            message = "WhiteBeam: Building test binary";
            cargo_command.args(&["+stable", "build", "--package", "whitebeam", "--bin", "whitebeam", "--release",
                                 "--features", "whitelist_test", "-q"]);
            bin_target_path
        },
        "library-test" => {
            message = "WhiteBeam: Building test library";
            cargo_command.args(&["+nightly-2022-07-23", "build", "--package", "libwhitebeam", "--lib", "--release",
                                 "--features", "whitelist_test", "-q"]);
            lib_target_path
        },
        _ => {
            eprintln!("WhiteBeam: Invalid subcommand. Valid subcommands are: binary library binary-test library-test");
            return;
        }
    };
    let mut sp_build = spinners::Spinner::new(SPINNER_CHOICE.clone(), message.into());
    let status = cargo_command.status().expect("WhiteBeam: Failed to execute cargo command");
    sp_build.stop_with_newline();
    if !(status.success()) {
        eprintln!("WhiteBeam: Failed to build {}", subcommand);
        std::process::exit(1);
    }
    match fs::metadata(&current_target_path) {
        Ok(meta) => println!("WhiteBeam: Completed. Size: {}", pretty_bytes(meta.len() as f64)),
        Err(_e) => eprintln!("WhiteBeam: Failed to stat {}", (&current_target_path).display())
    }
}

fn test(args: Vec<String>) {
    // TODO: Use build.rs for test setup steps?
    // TODO: More error handling
    println!("WhiteBeam: Testing:");
    build(vec![String::from("whitebeam-installer"), String::from("build"), String::from("library-test")]);
    build(vec![String::from("whitebeam-installer"), String::from("build"), String::from("binary-test")]);
    // Initialize test database
    let mut sp_db_init = spinners::Spinner::new(SPINNER_CHOICE.clone(), "WhiteBeam: Initializing test database".into());
    common::db::db_optionally_init(&args[1].to_lowercase()).expect("WhiteBeam: Failed to initialize test database");
    // Load platform-specific Essential hooks through whitebeam command
    common::db::db_load("Essential").expect("WhiteBeam: Failed to load Essential hooks");
    // Load platform-specific test data through whitebeam command
    common::db::db_load("Test").expect("WhiteBeam: Failed to load test data");
    // Allow the libwhitebeam.so (test) library to load when Prevention is enabled
    let _exit_status_allow_libwhitebeam = Command::new(format!("{}/target/release/whitebeam", env!("PWD")))
        .args(&["--add", "Filesystem/Path/Library", "ANY", &format!("{}/target/release/libwhitebeam.so", env!("PWD"))])
        .stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null())
        .status()
        .expect("WhiteBeam: Failed to execute whitebeam command");
    // Allow the whitebeam (test) binary to run when Prevention is enabled
    let _exit_status_allow_whitebeam = Command::new(format!("{}/target/release/whitebeam", env!("PWD")))
        .args(&["--add", "Filesystem/Path/Executable", &format!("{}/target/release/whitebeam", env!("PWD"))])
        .stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null())
        .status()
        .expect("WhiteBeam: Failed to execute whitebeam command");
    // Set a test recovery secret
    let _exit_status_secret = Command::new(format!("{}/target/release/whitebeam", env!("PWD")))
        .args(&["--setting", "RecoverySecret", "test"])
        .stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null())
        .status()
        .expect("WhiteBeam: Failed to execute whitebeam command");
    sp_db_init.stop_with_newline();
    // Build integration tests
    let mut sp_integration = spinners::Spinner::new(SPINNER_CHOICE.clone(), "WhiteBeam: Building integration tests".into());
    let _exit_status_tests = Command::new("cargo")
        .arg("+nightly-2022-07-23").arg("test").arg("--package").arg("libwhitebeam").arg("--release").arg("--features").arg("whitelist_test").arg("--test").arg("test_integration").arg("-q").arg("--no-run")
        // TODO: Replace with https://github.com/rust-lang/cargo/blob/master/src/doc/src/reference/unstable.md#profile-strip-option once stabilized
        .env("RUSTFLAGS", "-C link-arg=-s -Z plt=yes")
        .status()
        .expect("WhiteBeam: Failed to execute cargo command");
    sp_integration.stop_with_newline();
    // Run integration tests
    let _exit_status_tests = Command::new("cargo")
        .arg("+nightly-2022-07-23").arg("test").arg("--package").arg("libwhitebeam").arg("--release").arg("--features").arg("whitelist_test").arg("--test").arg("test_integration").arg("-q")
        // TODO: Replace with https://github.com/rust-lang/cargo/blob/master/src/doc/src/reference/unstable.md#profile-strip-option once stabilized
        .env("RUSTFLAGS", "-C link-arg=-s -Z plt=yes")
        .status()
        .expect("WhiteBeam: Failed to execute cargo command");
    // Reset recovery secret
    let _exit_status_reset = Command::new(format!("{}/target/release/whitebeam", env!("PWD")))
        .args(&["--setting", "RecoverySecret", "undefined"])
        .status()
        .expect("WhiteBeam: Failed to execute whitebeam command");
    // TODO: Remove libwhitebeam.so and whitebeam from whitelist
    // TODO: Test actions
    // TODO: Make sure SQL schema/defaults exist
    // TODO: Test binary (e.g. ./target/release/whitebeam || true)
    // TODO: Benches
}

fn install(_args: Vec<String>) {
    // TODO: Verify we're in the right directory
    platform::run_install();
}

fn package(_args: Vec<String>) {
    // TODO: Verify we're in the right directory
    platform::run_package();
    let package_name: String = format!("WhiteBeam_{}_{}", env!("CARGO_PKG_VERSION"), std::env::consts::ARCH);
    #[cfg(target_os = "windows")]
    let package_path = format!(".\\target\\release\\{}.zip", package_name);
    #[cfg(not(target_os = "windows"))]
    let package_path = format!("./target/release/{}.tar.gz", package_name);
    match fs::metadata(&package_path) {
        Ok(meta) => println!("WhiteBeam: Completed ({}). Size: {}", package_path, pretty_bytes(meta.len() as f64)),
        Err(_e) => eprintln!("WhiteBeam: Failed to stat {}", package_path)
    }
}

fn clean(_args: Vec<String>) {
    println!("WhiteBeam: Cleaning up");
    let _clean_result = Command::new(platform::search_path(OsStr::new("cargo")).unwrap())
            .arg("clean")
            .output()
            .expect("WhiteBeam: Failed to execute cargo command");
    fs::remove_file("Cargo.lock").expect("WhiteBeam: Failed to remove Cargo.lock");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if (args.len()-1) > 0 {
        let command = &args[1].to_lowercase();
        if command == "build" {
            build(args)
        } else if command == "test" {
            test(args)
        } else if command == "install" {
            install(args)
        } else if command == "package" {
            package(args)
        } else if command == "clean" {
            clean(args)
        } else {
            eprintln!("WhiteBeam: Invalid command. Valid commands are: build test install clean")
        }
    } else {
        eprintln!("WhiteBeam: Invalid options. Syntax: cargo run command")
    }
}
