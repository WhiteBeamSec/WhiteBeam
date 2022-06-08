use goblin::elf;
use std::{error::Error,
          fs::File,
          io::BufRead};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path,
                PathBuf};
use std::process::{Command, Stdio};

pub fn start_service() {
    match Command::new("/etc/init.d/whitebeam")
            .arg("start")
            .stdout(Stdio::null())
            .spawn() {
                Ok(_p) => return,
                Err(_e) => {
                    eprintln!("WhiteBeam: Child process failed to start");
                    return;
                }
    };
}

pub fn stop_service() {
    match Command::new("/etc/init.d/whitebeam")
            .arg("stop")
            .stdout(Stdio::null())
            .spawn() {
                Ok(_p) => return,
                Err(_e) => {
                    eprintln!("WhiteBeam: Child process failed to start");
                    return;
                }
    };
}

pub fn get_data_file_path(data_file: &str) -> PathBuf {
    #[cfg(feature = "whitelist_test")]
    let data_path: String = format!("{}/target/aarch64-unknown-linux-gnu/debug/examples/data/", env!("PWD"));
    #[cfg(not(feature = "whitelist_test"))]
    let data_path: String = String::from("/opt/WhiteBeam/data/");
    let data_file_path = data_path + data_file;
    PathBuf::from(data_file_path)
}

pub fn get_realtime_file_path(realtime_file: &str) -> PathBuf {
    #[cfg(feature = "whitelist_test")]
    let realtime_path: String = format!("{}/target/aarch64-unknown-linux-gnu/debug/examples/realtime/", env!("PWD"));
    #[cfg(not(feature = "whitelist_test"))]
    let realtime_path: String = String::from("/opt/WhiteBeam/realtime/");
    let realtime_file_path = realtime_path + realtime_file;
    PathBuf::from(realtime_file_path)
}

pub fn get_syslog_path() -> PathBuf {
    PathBuf::from("/var/log/syslog")
}

pub fn path_open_secure(file_path: &Path) -> Result<File, std::io::Error> {
    Ok(std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o700)
        .open(file_path)?)
}


pub fn parse_ld_so_conf(path: &str) -> std::io::Result<Vec<String>> {
    let mut paths = Vec::new();

    let f = File::open(path)?;
    let f = std::io::BufReader::new(&f);
    for line in f.lines() {
        let line = line?;
        let line = line.trim();
        if line.starts_with("#") {
            continue;
        }
        if line == "" {
            continue;
        }

        if line.contains(" ") {
            if line.starts_with("include ") {
                for entry in glob::glob(line.split(" ").last().unwrap()).expect("Failed to read glob pattern") {
                    paths.extend(parse_ld_so_conf(&entry.unwrap().to_string_lossy().into_owned())?);
                }

            }
        } else {
            paths.push(line.to_owned());
        }
    }
    Ok(paths)
}

pub fn library_scan(elf_path_str: &str, search_paths: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    // TODO: DT_RPATH/DT_RUNPATH
    let elf_path = std::path::Path::new(&elf_path_str);
    let elf_buffer = std::fs::read(elf_path)?;
    let elf_parsed = elf::Elf::parse(&elf_buffer)?;
    let elf_is_64 = elf_parsed.is_64;
    let mut collected_library_paths: Vec<String> = vec![];
    let collected_libraries: Vec<String> = elf_parsed.libraries.iter().map(|s| s.to_string()).collect();
    for lib in collected_libraries.iter() {
        for search_path in search_paths.iter() {
            let search_path_string = format!("{}/{}", &search_path, &lib);
            let search_path_expanded = std::path::Path::new(&search_path_string);
            if !(search_path_expanded.exists()) {
                continue
            }
            let elf_next_buffer = std::fs::read(search_path_expanded)?;
            let elf_next_parsed = elf::Elf::parse(&elf_next_buffer)?;
            if elf_next_parsed.is_64 != elf_is_64 {
                continue
            }
            collected_library_paths.push(search_path_string);
            break
        }
    }
    Ok(collected_library_paths)
}

pub fn recursive_library_scan(elf_path_str: &str, collected_library_paths_opt: Option<Vec<String>>, search_paths_opt: Option<Vec<String>>) -> Result<Vec<String>, Box<dyn Error>> {
    // Recursively collect DT_NEEDED libraries
    let search_paths: Vec<String> = match search_paths_opt {
        Some(paths) => paths,
        None => {
            // TODO: Missing default library_paths for 64 bit?
            parse_ld_so_conf("/etc/ld.so.conf").unwrap_or(vec![String::from("/lib"), String::from("/usr/lib")])
        }
    };
    let mut collected_library_paths: Vec<String> = match collected_library_paths_opt {
        Some(lib_paths) => lib_paths,
        None => {
            vec![]
        }
    };
    let elf_library_paths: Vec<String> = library_scan(elf_path_str, search_paths.clone())?;
    for lib_path in elf_library_paths.iter() {
        if collected_library_paths.contains(lib_path) {
            continue;
        }
        collected_library_paths.push(String::from(lib_path));
        for lib_dep in recursive_library_scan(lib_path, Some(collected_library_paths.clone()), Some(search_paths.clone()))?.iter() {
            if collected_library_paths.contains(lib_dep) {
                continue;
            }
            collected_library_paths.push(String::from(lib_dep));
        }
    }
    Ok(collected_library_paths)
}

pub fn parse_os_version() -> Result<String, Box<dyn Error>> {
    let file = std::fs::read_to_string("/etc/os-release")?;
    let mut distro = String::from("");
    let mut version = String::from("");
    for line in file.lines() {
        if line.starts_with("ID=") {
            distro = os_version_value(line)?;
        } else if line.starts_with("VERSION_ID=") {
            version = os_version_value(line)?;
        }
    }
    Ok(format!("{}_{}", distro, version))
}

pub fn pretty_os_version() -> Result<String, Box<dyn Error>> {
    let file = std::fs::read_to_string("/etc/os-release")?;
    let mut pretty_distro = String::from("Unknown");
    for line in file.lines() {
        if line.starts_with("PRETTY_NAME=") {
            pretty_distro = os_version_value(line)?;
        }
    }
    Ok(pretty_distro)
}

fn os_version_value(line: &str) -> Result<String, Box<dyn Error>> {
    let idx = line.find('=').unwrap();
    let mut value = &line[idx+1..];
    if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
        value = &value[1..value.len()-1];
    }
    Ok(value.to_string())
}

pub fn is_superuser() -> bool {
    #[cfg(feature = "whitelist_test")]
    let res: bool = true;
    #[cfg(not(feature = "whitelist_test"))]
    let res: bool = unsafe { libc::getuid() == 0 };
    res
}
