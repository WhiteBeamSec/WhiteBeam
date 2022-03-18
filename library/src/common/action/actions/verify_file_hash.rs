use std::io::prelude::*;

fn fail(library_basename: &str, symbol: &str, argument_path: &str) {
    // TODO: Library path in error inconsistent with rest of application
    if (symbol.contains("exec") || symbol.contains("posix_spawn")) && (library_basename == "libc.so.6") {
        // Terminate the child process
        eprintln!("WhiteBeam: {}: Permission denied", argument_path);
        unsafe { libc::exit(126) };
    } else {
        unimplemented!("WhiteBeam: The '{}' symbol (from {}) is not supported by the VerifyFileHash action", symbol, library_basename);
    }
}

build_action! { VerifyFileHash (par_prog, src_prog, hook, arg_id, args, _act_args, do_return, return_value) {
        // TODO: Depending on LogSeverity, log all use of this action
        // NB: For Execution hooks, system executables that aren't read world may be whitelisted as ANY
        let library: &str = &hook.library;
        let library_basename: &str = library.rsplit('/').next().unwrap_or(library);
        let symbol: &str = &hook.symbol;
        let any = String::from("ANY");
        let class = String::from("Hash/");
        let argument: crate::common::db::ArgumentRow = args.iter().find(|arg| arg.id == arg_id).expect("WhiteBeam: Lost track of environment").clone();
        let argument_path: String = match (library_basename, symbol) {
            ("libc.so.6", "fexecve") => {
                let canonical_path = platform::canonicalize_fd(argument.real as i32).expect("WhiteBeam: Lost track of environment");
                canonical_path.into_os_string().into_string().expect("WhiteBeam: Unexpected null reference")
            },
            _ => unsafe { String::from(std::ffi::CStr::from_ptr(argument.real as *const libc::c_char).to_str().expect("WhiteBeam: Unexpected null reference")) }
        };
        let all_allowed_hashes: Vec<(String, String)> = {
            let whitelist_cache_lock = crate::common::db::WL_CACHE.lock().expect("WhiteBeam: Failed to lock mutex");
            whitelist_cache_lock.iter().filter(|whitelist| (whitelist.class.starts_with(&class)) && ((whitelist.parent == par_prog) || (whitelist.parent == any)) && ((whitelist.path == argument_path) || (whitelist.path == any))).map(|whitelist| (whitelist.class.clone(), whitelist.value.clone())).collect()
        };
        // Permit ANY
        if all_allowed_hashes.iter().any(|hash_tuple| hash_tuple.1 == any) {
            return (hook, args, do_return, return_value);
        }
        // Permit whitelisted file hashes (consecutively). This allows hybrid hashing schemes for additional security (e.g. SHA3 and BLAKE3).
        let hash_count = all_allowed_hashes.len();
        let mut argument_file: std::fs::File = match std::fs::File::open(&argument_path) {
            Ok(f) => f,
            Err(_e) => {
                fail(library_basename, symbol, &argument_path);
                unreachable!("WhiteBeam: Lost track of environment");
            }
        };
        let passed_all: bool = all_allowed_hashes.iter().all(|hash_tuple| {
            argument_file.seek(std::io::SeekFrom::Start(0)).expect("WhiteBeam: VerifyFileHash failed to seek in target file");
            hash_tuple.1 == crate::common::hash::process_hash(&mut argument_file, &(hash_tuple.0), None)
        });
        if (hash_count > 0) && passed_all {
            return (hook, args, do_return, return_value);
        }
        if !(crate::common::db::get_prevention()) {
            crate::common::event::send_log_event(libc::LOG_NOTICE, format!("Detection: {} -> {} accessed file with invalid file hash {} (VerifyFileHash)", &par_prog, &src_prog, &argument_path));
            return (hook, args, do_return, return_value);
        }
        // Permit authorized execution
        if crate::common::db::get_valid_auth_env() {
            return (hook, args, do_return, return_value);
        }
        // Deny by default
        event::send_log_event(libc::LOG_WARNING, format!("Prevention: Blocked {} -> {} due to incorrect hash of {} (VerifyFileHash)", &par_prog, &src_prog, &argument_path));
        fail(library_basename, symbol, &argument_path);
}}
