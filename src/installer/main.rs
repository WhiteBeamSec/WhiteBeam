use std::{env,
          ffi::OsStr,
          fs,
          path::Path,
          process::Command};
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
    platform::check_build_environment();
    let (mut compile_bin, mut compile_lib) = (true, true);
    if (args.len()-1) > 1 {
        let subcommand: &str = &(&args[2].to_lowercase());
        match subcommand {
            "binary" => {
                compile_lib = false;
            },
            "library" => {
                compile_bin = false;
            },
            _ => {
                eprintln!("WhiteBeam: Invalid subcommand. Valid subcommands are: binary library");
                return;
            }
        }
    }
    if compile_lib {
        println!("Building library");
        let _exit_status_lib = Command::new("cargo")
            .arg("+nightly").arg("build").arg("--package").arg("libwhitebeam").arg("--lib").arg("--release")
            .env("RUSTFLAGS", "-C link-arg=-s")
            .status()
            .expect("Failed to execute cargo command");
        match fs::metadata("./target/release/libwhitebeam.so") {
            Ok(meta) => println!("Completed. Size: {}", pretty_bytes(meta.len() as f64)),
            Err(_e) => println!("Failed to stat ./target/release/libwhitebeam.so")
        }
    }
    if compile_bin {
        let _exit_status_bin = Command::new("cargo")
            .arg("+stable").arg("build").arg("--package").arg("whitebeam").arg("--bin").arg("whitebeam").arg("--release")
            .env("RUSTFLAGS", "-C link-arg=-s")
            .status()
            .expect("Failed to execute cargo command");
        match fs::metadata("./target/release/whitebeam") {
            Ok(meta) => println!("Completed. Size: {}", pretty_bytes(meta.len() as f64)),
            Err(_e) => println!("Failed to stat ./target/release/whitebeam")
        }
    }
}

fn test(_args: Vec<String>) {
    // TODO: Verify we're in the right directory
    println!("Building test library");
    let _exit_status_lib = Command::new("cargo")
        .arg("+nightly").arg("build").arg("--package").arg("libwhitebeam").arg("--lib").arg("--release")
        // Arguments for testing
        .arg("--manifest-path").arg("./src/library/Cargo.toml").arg("--features").arg("whitelist_test")
        .env("RUSTFLAGS", "-C link-arg=-s")
        .status()
        .expect("Failed to execute cargo command");
    match fs::metadata("./target/release/libwhitebeam.so") {
        Ok(meta) => println!("Completed. Size: {}", pretty_bytes(meta.len() as f64)),
        Err(_e) => println!("Failed to stat ./target/release/libwhitebeam.so")
    }
    let libwhitebeam_file = Command::new(platform::search_path(OsStr::new("file")).unwrap())
            .arg("./target/release/libwhitebeam.so")
            .output()
            .expect("Failed to execute file command");
    println!("{}", String::from_utf8_lossy(&libwhitebeam_file.stdout).trim_end());
    println!("Exported symbols:");
    let libwhitebeam_objdump = Command::new(platform::search_path(OsStr::new("objdump")).unwrap())
            .arg("-T").arg("-j").arg(".text").arg("./target/release/libwhitebeam.so")
            .output()
            .expect("Failed to execute objdump command");
    let libwhitebeam_objdump_string = String::from_utf8_lossy(&libwhitebeam_objdump.stdout);
    let mut modules: Vec<&str> = Vec::new();
    for line in libwhitebeam_objdump_string.lines() {
        if line.contains(".text") && !line.contains("rust_eh_personality") {
            modules.push(line.split_ascii_whitespace().last().unwrap());
        }
    }
    for module in &modules {
        println!("* {}", module);
    }
    println!("Testing:");
    let _exit_status_tests = Command::new("cargo")
        .arg("+stable").arg("build").arg("--package").arg("libwhitebeam-tests").arg("--release")
        .env("RUSTFLAGS", "-C link-arg=-s")
        .status()
        .expect("Failed to execute cargo command");
    for module in &modules {
        // TODO: fexecve in Linux tests
        if module == &"fexecve" {
            eprintln!("Skipping fexecve");
            continue;
        }
        for test_type in &["positive", "negative"] {
            let exit_status_module = Command::new("./target/release/libwhitebeam-tests")
                .arg(module).arg(test_type)
                .env("LD_PRELOAD", "./target/release/libwhitebeam.so")
                .status()
                .expect("Failed to execute cargo command");
                // TODO: Use OS temp directory/directory relative to cwd instead of hardcoding /tmp/
            if test_type == &"positive" {
                // Positive test
                assert!(exit_status_module.success());
                let contents = fs::read_to_string("/tmp/test_result").expect("Could not read test result file");
                assert_eq!(contents, String::from("./target/release/libwhitebeam.so"));
                fs::remove_file("/tmp/test_result").expect("Failed to remove /tmp/test_result");
            } else {
                // Negative test
                // TODO: assert!(!exit_status_module.success());
                assert_eq!(Path::new("/tmp/test_result").exists(), false);
            }
            println!("{} passed ({} test).", module, test_type);
        }
    }
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
            .expect("Failed to execute cargo command");
    fs::remove_file("Cargo.lock").expect("Failed to remove Cargo.lock");
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
