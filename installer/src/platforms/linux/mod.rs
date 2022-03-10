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
    let rustup_toolchains = Command::new(search_path(OsStr::new("rustup")).unwrap())
            .arg("toolchain")
            .arg("list")
            .output()
            .expect("WhiteBeam: Failed to execute rustup command");
    let rustup_toolchains_string = String::from_utf8_lossy(&rustup_toolchains.stdout);
    /*
    if !rustup_toolchains_string.contains("stable") {
        eprintln!("WhiteBeam: No stable Rust found in toolchain, consider running: rustup toolchain install stable");
        std::process::exit(1);
    } else */ if !(rustup_toolchains_string.contains("nightly-2022-03-01")) {
        eprintln!("WhiteBeam: No pinned nightly Rust found in toolchain, consider running: rustup toolchain install nightly-2022-03-01");
        std::process::exit(1);
    }
}

pub fn run_install() {
    // TODO: Eliminate service.sh
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
            .status().expect("WhiteBeam: Child process failed to start");
        return;
    }
    // TODO: Update by the time glibc 2.36 is released (~Aug 2022)
    if std::env::consts::ARCH == "aarch64" {
        let ld_reported_version = match Command::new("/usr/lib/aarch64-linux-gnu/ld-linux-aarch64.so.1")
            .arg("--version")
            .output() {
            Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
            Err(_) => String::from("")
        };
        if !(ld_reported_version.contains("version 2.35")) && !(Path::new("/usr/lib/aarch64-linux-gnu/ld-2.35.so").exists()) {
            eprintln!("WhiteBeam: glibc 2.35 required on aarch64 platforms");
            std::process::exit(1);
        }
    }
    let mut installation_cmd: String = String::from("mkdir -p /opt/WhiteBeam/data/;");
    if PathBuf::from("./service.sh").exists() {
        // Release
        installation_cmd.push_str(concat!("cp ./service.sh /etc/init.d/whitebeam;",
                                          "cp ./libwhitebeam.so /opt/WhiteBeam/libwhitebeam.so;",
                                          "cp ./whitebeam /opt/WhiteBeam/whitebeam;"));
    } else if PathBuf::from("./installer/src/platforms/linux/resources/service.sh").exists() {
        // Source
        installation_cmd.push_str(concat!("cp ./installer/src/platforms/linux/resources/service.sh /etc/init.d/whitebeam;",
                                          "cp ./target/release/libwhitebeam.so /opt/WhiteBeam/libwhitebeam.so;",
                                          "cp ./target/release/whitebeam /opt/WhiteBeam/whitebeam;"));
    } else {
        eprintln!("WhiteBeam: Cannot locate installation files");
        std::process::exit(1);
    }
    installation_cmd.push_str(concat!("ln -s /etc/init.d/whitebeam /etc/rc3.d/S01whitebeam;",
                                      "ln -s /opt/WhiteBeam/libwhitebeam.so /lib/libwhitebeam.so;",
                                      "ln -s /opt/WhiteBeam/whitebeam /usr/local/bin/whitebeam;",
                                      "chmod 775 /etc/init.d/whitebeam;",
                                      "chmod 4555 /opt/WhiteBeam/libwhitebeam.so;",
                                      "whitebeam --load Schema;",
                                      "whitebeam --load Default;",
                                      "whitebeam --setting SystemArchitecture `arch`;",
                                      "whitebeam --load Essential;",
                                      "whitebeam --load Base;",
                                      "/etc/init.d/whitebeam start;",
                                      "echo '/lib/libwhitebeam.so' | tee -a /etc/ld.so.preload;"));
    println!("WhiteBeam: Installing");
    Command::new(search_path(OsStr::new("bash")).unwrap())
            .arg("-c")
            .arg(installation_cmd)
            .status()
            .expect("WhiteBeam: Installation failed");
}

pub fn run_package() {
    // TODO: Eliminate service.sh
    if !(PathBuf::from("./installer/src/platforms/linux/resources/service.sh").exists()) {
        eprintln!("WhiteBeam: Must be run from source directory");
        std::process::exit(1);
    }
    if !(PathBuf::from("./target/debug/whitebeam-installer").exists() &&
         PathBuf::from("./target/release/libwhitebeam.so").exists() &&
         PathBuf::from("./target/release/whitebeam").exists()) {
        eprintln!("WhiteBeam: Missing files necessary for packaging, consider running: cargo run build");
        std::process::exit(1);
    }
    let package_name: String = format!("WhiteBeam_{}_{}", env!("CARGO_PKG_VERSION"), std::env::consts::ARCH);
    let package_cmd: String = format!(concat!("tar --transform='s%.*/%%' --transform 'flags=r;s|^|WhiteBeam/|' --numeric-owner --owner 0 --group 0 -cvzf ./target/release/{}.tar.gz ",
                                                   "./target/debug/whitebeam-installer ./target/release/libwhitebeam.so ./installer/src/platforms/linux/resources/service.sh ./target/release/whitebeam;",
                                              "sha256sum ",
                                                   "./target/release/{}.tar.gz > ./target/release/{}.SHA256;"), package_name, package_name, package_name);
    println!("WhiteBeam: Packaging");
    Command::new(search_path(OsStr::new("bash")).unwrap())
            .arg("-c")
            .arg(package_cmd)
            .status()
            .expect("WhiteBeam: Packaging failed");
}
