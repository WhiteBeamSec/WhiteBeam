// TODO: Cross platform, tests, replace install.sh, add sqlite config for osquery/osquery pkgs

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
    let mut cargo_command = Command::new("cargo");
    cargo_command.env("RUSTFLAGS", "-C link-arg=-s");
    let lib_target_path: PathBuf = PathBuf::from(format!("{}/target/release/libwhitebeam.so", env!("PWD")));
    let bin_target_path: PathBuf = PathBuf::from(format!("{}/target/release/whitebeam", env!("PWD")));
    let subcommand: &str = &(&args[2].to_lowercase());
    let current_target_path = match subcommand {
        "binary" => {
            println!("Building binary");
            cargo_command.args(&["build", "--package", "whitebeam", "--bin", "whitebeam", "--release"]);
            bin_target_path
        },
        "library" => {
            println!("Building library");
            cargo_command.args(&["+nightly", "build", "--package", "libwhitebeam", "--lib", "--release"]);
            lib_target_path
        },
        "binary-test" => {
            println!("Building test binary");
            cargo_command.args(&["build", "--package", "whitebeam", "--bin", "whitebeam", "--release",
                                 "--manifest-path", "./src/application/Cargo.toml", "--features", "whitelist_test"]);
            bin_target_path
        },
        "library-test" => {
            println!("Building test library");
            cargo_command.args(&["+nightly", "build", "--package", "libwhitebeam", "--lib", "--release",
                                 "--manifest-path", "./src/library/Cargo.toml", "--features", "whitelist_test"]);
            lib_target_path
        },
        _ => {
            eprintln!("WhiteBeam: Invalid subcommand. Valid subcommands are: binary library binary-test library-test");
            return;
        }
    };
    cargo_command.status().expect("WhiteBeam: Failed to execute cargo command");
    match fs::metadata(&current_target_path) {
        Ok(meta) => println!("WhiteBeam: Completed. Size: {}", pretty_bytes(meta.len() as f64)),
        Err(_e) => println!("WhiteBeam: Failed to stat {}", (&current_target_path).display())
    }
}

fn test(args: Vec<String>) {
    // TODO: More error handling
    build(vec![String::from("whitebeam-installer"), String::from("build"), String::from("library-test")]);
    build(vec![String::from("whitebeam-installer"), String::from("build"), String::from("binary-test")]);
    println!("Testing:");
    // Initialize test database
    common::db::db_optionally_init(&args[1].to_lowercase()).expect("WhiteBeam: Failed to initialize test database");
    // Load platform-specific Essential hooks through whitebeam command
    common::db::load_platform_data("essential").expect("WhiteBeam: Failed to load Essential hooks");
    // Load platform-specific test data through whitebeam command
    common::db::load_platform_data("test").expect("WhiteBeam: Failed to load test data");
    // Run tests
    let _exit_status_tests = Command::new("cargo")
        .arg("build").arg("--package").arg("libwhitebeam-tests").arg("--release")
        // TODO: Replace with https://github.com/rust-lang/cargo/blob/master/src/doc/src/reference/unstable.md#profile-strip-option once stabilized
        .env("RUSTFLAGS", "-C link-arg=-s")
        .status()
        .expect("WhiteBeam: Failed to execute cargo command");
    // TODO: Run tests (cont.): Iterate over tests that just got loaded with application listing them
    /*
    for module in &modules {
        // TODO: fexecve in Linux tests
        if module == &"fexecve" {
            eprintln!("WhiteBeam: Skipping fexecve");
            continue;
        }
        for test_type in &["positive", "negative"] {
            let exit_status_module = Command::new("./target/release/libwhitebeam-tests")
                .arg(module).arg(test_type)
                .env("LD_PRELOAD", "./target/release/libwhitebeam.so")
                .status()
                .expect("WhiteBeam: Failed to execute cargo command");
                // TODO: Use OS temp directory/directory relative to cwd instead of hardcoding /tmp/
            if test_type == &"positive" {
                // Positive test
                assert!(exit_status_module.success());
                let contents = fs::read_to_string("/tmp/test_result").expect("WhiteBeam: Could not read test result file");
                assert_eq!(contents, String::from("./target/release/libwhitebeam.so"));
                fs::remove_file("/tmp/test_result").expect("WhiteBeam: Failed to remove /tmp/test_result");
            } else {
                // Negative test
                // TODO: assert!(!exit_status_module.success());
                assert_eq!(Path::new("/tmp/test_result").exists(), false);
            }
            println!("{} passed ({} test).", module, test_type);
        }
    }
    */
    // TODO: Test binary (e.g. ./target/release/whitebeam || true)
}

fn install(_args: Vec<String>) {
    // TODO: Verify we're in the right directory
    platform::run_install();
}

fn clean(_args: Vec<String>) {
    println!("Cleaning up");
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
        } else if command == "clean" {
            clean(args)
        } else {
            eprintln!("WhiteBeam: Invalid command. Valid commands are: build test install clean")
        }
    } else {
        eprintln!("WhiteBeam: Invalid options. Syntax: cargo run command")
    }
}
