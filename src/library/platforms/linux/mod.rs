// Load OS-specific modules
#[macro_use]
mod hook;
mod system;

use libc::{c_char, c_int};
use std::ptr;
use std::ffi::CStr;
use std::{os::unix::ffi::OsStringExt};
use std::{ffi::OsString};
use crate::library::common::hash;
use crate::library::common::event;

fn parse_env(input: &[u8]) -> Option<(OsString, OsString)> {
	if input.is_empty() {
		return None;
	}
	let pos = input[1..].iter().position(|&x| x == b'=').map(|p| p + 1);
	pos.map(|p| {
		(
			OsStringExt::from_vec(input[..p].to_vec()),
			OsStringExt::from_vec(input[p + 1..].to_vec()),
		)
	})
}

unsafe fn transform_parameters(path: *const c_char, envp: *const *const c_char, fd: c_int) -> (String, Vec<(OsString, OsString)>, String, u32) {
	// TODO: GC

	// Program, hexdigest
	let (program, hexdigest) = if !(path.is_null()) {
		let program_c_str: &CStr = CStr::from_ptr(path);
		let program_str_slice: &str = program_c_str.to_str().unwrap();
		let prog: String = program_str_slice.to_owned(); // If necessary
		let hash_prog = hash::common_hash_file(&prog);
		(prog, hash_prog)
	} else {
		let prog: String = format!("fd://{}", fd);
		let hash_prog = hash::common_hash_file(&prog);
		(prog, hash_prog)
	};

	// Environment variables
	let mut env: Vec<(OsString, OsString)> = Vec::new();
	if !(envp.is_null()) {
		let mut envp_iter = envp;
		while !(*envp_iter).is_null() {
				if let Some(key_value) = parse_env(CStr::from_ptr(*envp_iter).to_bytes()) {
					env.push(key_value);
				}
				envp_iter = envp_iter.offset(1);
		}
	}

	// User ID
	let uid = system::get_current_uid();

	(program, env, hexdigest, uid)
}

fn is_whitelisted(program: &str, env: &Vec<(OsString, OsString)>) -> bool {
    // TODO: Reference /opt/whitebeam/cache.json, use SHA3-256 hash

    let mut unsafe_env = false;
    let mut allow_exec = false;
    let env_blacklist = [
            "LD_PRELOAD",
            "LD_AUDIT",
            "LD_LIBRARY_PATH"
    ];
    let whitelist = [
        // Tuple of (permitted program, allow unsafe environment variables)
        // Shells
        ("/bin/bash", false),
        ("/bin/sh", false),
        // Whitebeam
        ("/opt/whitebeam/whitebeam", false),
        ("/usr/local/bin/whitebeam", false)
    ];
    for env_var in env {
        if env_blacklist.contains(&env_var.0.to_str().unwrap()) {
            unsafe_env = true;
            break;
        }
    }
    for (allowed_program, allow_unsafe) in whitelist.iter() {
        if (&program == allowed_program) && (&unsafe_env == allow_unsafe) {
            allow_exec = true;
            break;
        }
    }
    allow_exec
}

/*
Primary hook
*/
/*
       int execve(const char *path, char *const argv[],
                  char *const envp[]);
*/
hook! {
    unsafe fn hooked_execve(path: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int => execve {
		let (program, env, hexdigest, uid) = transform_parameters(path, envp, -1);
        // Permit/deny execution
        if is_whitelisted(&program, &env) {
            event::send_exec_event(uid, &program, &hexdigest, true);
            real!(hooked_execve)(path, argv, envp)
        } else {
            event::send_exec_event(uid, &program, &hexdigest, false);
            *system::errno_location() = libc::EACCES;
            return -1
        }
    }
}

/*
Secondary hooks
*/
/*
       int execle(const char *path, const char *arg, ...
                       /*, (char *) NULL, char * const envp[] */);
*/
hook! {
    unsafe fn hooked_execle(path: *const c_char, arg: *const c_char, envp: *const *const c_char) -> c_int => execle {
		let (program, env, hexdigest, uid) = transform_parameters(path, envp, -1);
        // Permit/deny execution
        if is_whitelisted(&program, &env) {
            event::send_exec_event(uid, &program, &hexdigest, true);
            real!(hooked_execle)(path, arg, envp)
        } else {
            event::send_exec_event(uid, &program, &hexdigest, false);
            *system::errno_location() = libc::EACCES;
            return -1
        }
    }
}

/*
       int execvpe(const char *path, char *const argv[],
                       char *const envp[]);
*/
hook! {
    unsafe fn hooked_execvpe(path: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int => execvpe {
		let (program, env, hexdigest, uid) = transform_parameters(path, envp, -1);
		let which_abs_pathbuf = match which::which(&program) {
            Err(_why) => {
				*system::errno_location() = libc::ENOENT;
				return -1 },
            Ok(prog_path) => prog_path
        };
		let abs_prog_str = which_abs_pathbuf.to_str().unwrap();
        // Permit/deny execution
        if is_whitelisted(abs_prog_str, &env) {
            event::send_exec_event(uid, abs_prog_str, &hexdigest, true);
            real!(hooked_execvpe)(path, argv, envp)
        } else {
            event::send_exec_event(uid, abs_prog_str, &hexdigest, false);
            *system::errno_location() = libc::EACCES;
            return -1
        }
    }
}

/*
       int fexecve(int fd, char *const argv[], char *const envp[]);
*/
hook! {
    unsafe fn hooked_fexecve(fd: c_int, argv: *const *const c_char, envp: *const *const c_char) -> c_int => fexecve {
		let path: *const c_char = ptr::null();
		let (program, env, hexdigest, uid) = transform_parameters(path, envp, fd);
        // Permit/deny execution
        if is_whitelisted(&program, &env) {
            event::send_exec_event(uid, &program, &hexdigest, true);
            real!(hooked_fexecve)(fd, argv, envp)
        } else {
            event::send_exec_event(uid, &program, &hexdigest, false);
            *system::errno_location() = libc::EACCES;
            return -1
        }
    }
}

/*
       int execl(const char *path, const char *arg, ...
                       /* (char  *) NULL */);
*/
hook! {
    unsafe fn hooked_execl(path: *const c_char, arg: *const c_char) -> c_int => execl {
		let envp: *const *const c_char = ptr::null();
		let (program, env, hexdigest, uid) = transform_parameters(path, envp, -1);
        // Permit/deny execution
        if is_whitelisted(&program, &env) {
            event::send_exec_event(uid, &program, &hexdigest, true);
            real!(hooked_execl)(path, arg)
        } else {
            event::send_exec_event(uid, &program, &hexdigest, false);
            *system::errno_location() = libc::EACCES;
            return -1
        }
    }
}

/*
       int execlp(const char *path, const char *arg, ...
                       /* (char  *) NULL */);
*/
hook! {
    unsafe fn hooked_execlp(path: *const c_char, arg: *const c_char) -> c_int => execlp {
		let envp: *const *const c_char = ptr::null();
		let (program, env, hexdigest, uid) = transform_parameters(path, envp, -1);
		let which_abs_pathbuf = match which::which(&program) {
            Err(_why) => {
				*system::errno_location() = libc::ENOENT;
				return -1 },
            Ok(prog_path) => prog_path
        };
		let abs_prog_str = which_abs_pathbuf.to_str().unwrap();
        // Permit/deny execution
        if is_whitelisted(abs_prog_str, &env) {
            event::send_exec_event(uid, abs_prog_str, &hexdigest, true);
            real!(hooked_execlp)(path, arg)
        } else {
            event::send_exec_event(uid, abs_prog_str, &hexdigest, false);
            *system::errno_location() = libc::EACCES;
            return -1
        }
    }
}

/*
       int execv(const char *path, char *const argv[]);
*/
hook! {
    unsafe fn hooked_execv(path: *const c_char, argv: *const *const c_char) -> c_int => execv {
		let envp: *const *const c_char = ptr::null();
		let (program, env, hexdigest, uid) = transform_parameters(path, envp, -1);
        // Permit/deny execution
        if is_whitelisted(&program, &env) {
            event::send_exec_event(uid, &program, &hexdigest, true);
            real!(hooked_execv)(path, argv)
        } else {
            event::send_exec_event(uid, &program, &hexdigest, false);
            *system::errno_location() = libc::EACCES;
            return -1
        }
    }
}

/*
       int execvp(const char *file, char *const argv[]);
*/
hook! {
    unsafe fn hooked_execvp(path: *const c_char, argv: *const *const c_char) -> c_int => execvp {
		let envp: *const *const c_char = ptr::null();
		let (program, env, hexdigest, uid) = transform_parameters(path, envp, -1);
		let which_abs_pathbuf = match which::which(&program) {
            Err(_why) => {
				*system::errno_location() = libc::ENOENT;
				return -1 },
            Ok(prog_path) => prog_path
        };
		let abs_prog_str = which_abs_pathbuf.to_str().unwrap();
        // Permit/deny execution
        if is_whitelisted(abs_prog_str, &env) {
            event::send_exec_event(uid, abs_prog_str, &hexdigest, true);
            real!(hooked_execvp)(path, argv)
        } else {
            event::send_exec_event(uid, abs_prog_str, &hexdigest, false);
            *system::errno_location() = libc::EACCES;
            return -1
        }
    }
}
