// Load OS-specific modules
use std::{env,
          ffi::OsStr,
          ffi::OsString,
          os::unix::ffi::OsStrExt,
          path::Path,
          path::PathBuf,
          process::Command};

pub fn get_data_file_path(data_file: &str, release: &str) -> PathBuf {
    let data_path: String = match release {
        "test" => format!("{}/target/release/examples/", env!("PWD")),
        _ => String::from("/opt/WhiteBeam/data/")
    };
    let data_file_path = data_path + data_file;
    PathBuf::from(data_file_path)
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
                eprintln!("WhiteBeam: cc not found in PATH, consider running: apt update && apt install -y build-essential");
            } else if missing_requirement == "rustup" {
                eprintln!("WhiteBeam: rustup not found in PATH, consider running: wget -q --https-only --secure-protocol=TLSv1_2 https://sh.rustup.rs -O - | sh /dev/stdin -y && source $$HOME/.cargo/env");
            } else if missing_requirement == "pkg-config" {
                eprintln!("WhiteBeam: pkg-config not found in PATH, consider running: apt update && apt install -y pkg-config");
            } else {
                // Reserved for future dependencies
                eprintln!("WhiteBeam: {} not found in PATH", missing_requirement);
            }
            std::process::exit(1);
        }
    }
    if !(Path::new("/usr/include/openssl/").exists()) {
        eprintln!("WhiteBeam: OpenSSL development libraries not detected on this system, consider running: apt update && apt install -y libssl-dev");
        std::process::exit(1);
    }
    // Toolchains can be more than just "stable" and "nightly" (Docker containers use the Rust version number)
    /*
    let rustup_toolchains = Command::new(search_path(OsStr::new("rustup")).unwrap())
            .arg("toolchain")
            .arg("list")
            .output()
            .expect("WhiteBeam: Failed to execute rustup command");
    let rustup_toolchains_string = String::from_utf8_lossy(&rustup_toolchains.stdout);
    if !rustup_toolchains_string.contains("stable") {
        eprintln!("WhiteBeam: No stable Rust found in toolchain, consider running: rustup toolchain install stable");
        std::process::exit(1);
    } else if !rustup_toolchains_string.contains("nightly") {
        eprintln!("WhiteBeam: No nightly Rust found in toolchain, consider running: rustup toolchain install nightly");
        std::process::exit(1);
    }
    */
}

pub fn run_install() {
    if get_current_uid() != 0 {
        let sudo_path = match search_path(OsStr::new("sudo")) {
            Some(path) => path,
            None => {
                eprintln!("WhiteBeam: Insufficient privileges for installation of WhiteBeam and no sudo present");
                return;
            }
        };
        let program = env::current_exe().expect("WhiteBeam: Failed to determine path to current executable");
        Command::new(sudo_path)
            .arg(program)
            .arg("install")
            .status().expect("WhiteBeam: Child process failed to start.");
        return;
    }
    println!("Installing");
    // TODO: Use Rust instead of coreutils
    Command::new(search_path(OsStr::new("bash")).unwrap())
            .arg("-c")
            .arg("mkdir -p /opt/WhiteBeam/data/;
                  cp ./src/installer/platforms/linux/resources/service.sh /etc/init.d/whitebeam;
                  cp ./target/release/libwhitebeam.so /opt/WhiteBeam/libwhitebeam.so;
                  cp ./target/release/whitebeam /opt/WhiteBeam/whitebeam;
                  ln -s /etc/init.d/whitebeam /etc/rc3.d/S01whitebeam;
                  ln -s /opt/WhiteBeam/libwhitebeam.so /lib/libwhitebeam.so;
                  ln -s /opt/WhiteBeam/whitebeam /usr/local/bin/whitebeam;
                  chmod 775 /etc/init.d/whitebeam;
                  chmod 4555 /opt/WhiteBeam/libwhitebeam.so;
                  whitebeam --load Schema;
                  whitebeam --load Default;
                  whitebeam --setting SystemArchitecture `arch`;
                  whitebeam --load Essential;
                  /etc/init.d/whitebeam start;
                  echo '/lib/libwhitebeam.so' | tee -a /etc/ld.so.preload")
            .status()
            .expect("WhiteBeam: Installation failed");
    println!("Installation complete");
}
