// Load OS-specific modules

use crate::common::{action,
                    convert,
                    db};
use libc::{c_char, c_int, c_void};
use std::{env,
          ffi::CStr,
          ffi::CString,
          ffi::OsStr,
          ffi::OsString,
          os::unix::ffi::OsStrExt,
          path::PathBuf,
          lazy::SyncLazy,
          sync::Mutex};

const LA_FLG_BINDTO: libc::c_uint = 0x01;
const LA_FLG_BINDFROM: libc::c_uint = 0x02;
const RTLD_NEXT: *const c_void = -1isize as *const c_void;

// TODO: Hashmap/BTreemap to avoid race conditions, clean up of pthread_self() keys:
// Timestamp attribute, vec. len>0, check timestamp, pthread_equal, RefCell/Cell (?)
static CUR_PROG: SyncLazy<Mutex<OsString>> = SyncLazy::new(|| Mutex::new(OsString::new()));
static FN_STACK: SyncLazy<Mutex<Vec<i64>>> = SyncLazy::new(|| Mutex::new(vec![]));
// TODO: Library cookie Hashmap/BTreemap

// LinkMap TODO: Review mut, assign libc datatypes?
#[repr(C)]
pub struct LinkMap {
    pub l_addr: usize,
    pub l_name: *const libc::c_char,
    pub l_ld: usize,
    pub l_next: *mut LinkMap,
    pub l_prev: *mut LinkMap
}

#[repr(C)]
pub struct Elf32_Sym {
	pub st_name: u32,
    pub st_value: u32,
    pub st_size: u32,
    pub st_info: u8,
	pub st_other: u8,
	pub st_shndx: u16
}

#[repr(C)]
pub struct Elf64_Sym {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: u64,
    pub st_size: u64
}

// init_rtld_audit_interface
// Initializes WhiteBeam as an LD_AUDIT library
#[used]
#[allow(non_upper_case_globals)]
#[link_section = ".init_array"]
static init_rtld_audit_interface: unsafe extern "C" fn(libc::c_int, *const *const libc::c_char, *const *const libc::c_char) = {
    #[link_section = ".text.startup"]
    unsafe extern "C" fn init_rtld_audit_interface(argc: libc::c_int, argv: *const *const libc::c_char, envp: *const *const libc::c_char) {
        let mut update_ld_audit: bool = false;
        let mut update_ld_bind_not: bool = false;
        let mut wb_prog_present: bool = false;
        let rtld_audit_lib_path = get_rtld_audit_lib_path();
        // la_symbind*() doesn't get called when LD_BIND_NOW is set
        // More info: https://sourceware.org/bugzilla/show_bug.cgi?id=23734
        if env::var_os("LD_BIND_NOW").is_some() {
            // Technically we're looking for a non-empty string here, but instead we deny it altogether
            panic!("WhiteBeam: LD_BIND_NOW restricted");
        }
        let new_ld_audit_var: OsString = match env::var_os("LD_AUDIT") {
            Some(val) => {
                if convert::osstr_split_at_byte(&val, b':').0 == rtld_audit_lib_path {
                    OsString::new()
                } else {
                    update_ld_audit = true;
                    let mut new_ld_audit_osstring = OsString::from("LD_AUDIT=");
                    new_ld_audit_osstring.push(rtld_audit_lib_path.as_os_str());
                    new_ld_audit_osstring.push(OsStr::new(":"));
                    new_ld_audit_osstring.push(val);
                    new_ld_audit_osstring
                }
            }
            None => {
                update_ld_audit = true;
                let mut new_ld_audit_osstring = OsString::from("LD_AUDIT=");
                new_ld_audit_osstring.push(rtld_audit_lib_path.as_os_str());
                new_ld_audit_osstring
            }
        };
        let new_ld_bind_not_var: OsString = match env::var_os("LD_BIND_NOT") {
            Some(val) => {
                if val != OsString::from("1") {
                    update_ld_bind_not = true;
                    OsString::from("LD_BIND_NOT=1")
                } else {
                    OsString::new()
                }
            }
            None => {
                update_ld_bind_not = true;
                OsString::from("LD_BIND_NOT=1")
            }
        };
        // This variable is protected by WhiteBeam's Essential hooks/rules
        let program_path: OsString = match env::var_os("WB_PROG") {
            Some(val) => {
                wb_prog_present = true;
                let mut cur_prog_lock = CUR_PROG.lock().expect("WhiteBeam: Failed to lock mutex");
                cur_prog_lock.clear();
                cur_prog_lock.push(&val);
                val
            },
            None => {
                // TODO: We use procfs anyway for fexecve, so should we use it here?
                if (argc == 0) || (argv.is_null()) {
                    panic!("WhiteBeam: Lost track of environment");
                }
                match search_path(&convert::c_char_to_osstring(*argv)) {
                    Some(v) => {
                        v.canonicalize().expect("WhiteBeam: Could not canonicalize path").into_os_string()
                    },
                    None => {
                        panic!("WhiteBeam: Lost track of environment");
                    }
                }
            }
        };
        // Populate cache
        db::populate_cache().expect("WhiteBeam: Could not access database");
        if !(update_ld_audit) && !(update_ld_bind_not) {
            // Nothing to do, continue execution
            if wb_prog_present {
                env::remove_var("WB_PROG");
            }
            return;
        }
        // TODO: Log null reference, process errors
        let mut env_vec: Vec<*const libc::c_char> = Vec::new();
        let mut new_ld_audit_cstring: CString = CString::new("").expect("WhiteBeam: Unexpected null reference");
        let mut new_ld_bind_not_cstring: CString = CString::new("").expect("WhiteBeam: Unexpected null reference");
        if update_ld_audit {
            // TODO: Log null reference, process errors
            new_ld_audit_cstring = convert::osstr_to_cstring(&new_ld_audit_var).expect("WhiteBeam: Unexpected null reference");
            // TODO: Check whitelist for path
            env_vec.push(new_ld_audit_cstring.as_ptr());
        }
        if update_ld_bind_not {
            // TODO: Log null reference, process errors
            new_ld_bind_not_cstring = convert::osstr_to_cstring(&new_ld_bind_not_var).expect("WhiteBeam: Unexpected null reference");
            env_vec.push(new_ld_bind_not_cstring.as_ptr());
        }
        let mut program_path_env: OsString = OsString::from("WB_PROG=");
        program_path_env.push(&program_path);
        let program_path_env_cstring = convert::osstr_to_cstring(&program_path_env).expect("WhiteBeam: Unexpected null reference");
        env_vec.push(program_path_env_cstring.as_ptr());
        let program_path_cstring = convert::osstr_to_cstring(&program_path).expect("WhiteBeam: Unexpected null reference");
        if !(envp.is_null()) {
            let mut envp_iter = envp;
            while !(*envp_iter).is_null() {
                if let Some(key_value) = convert::parse_env_single(CStr::from_ptr(*envp_iter).to_bytes()) {
                    if  (!(update_ld_audit) && (key_value.0 == "LD_AUDIT"))
                     || (!(update_ld_bind_not) && (key_value.0 == "LD_BIND_NOT"))
                     || ((key_value.0 != "LD_AUDIT") && (key_value.0 != "LD_BIND_NOT") && (key_value.0 != "WB_PROG")) {
                        env_vec.push(*envp_iter);
                    }
                }
                envp_iter = envp_iter.offset(1);
            }
        }
        env_vec.push(std::ptr::null());
        let new_envp: *const *const libc::c_char = (&env_vec).as_ptr() as *const *const libc::c_char;
        // Drop any setuid privileges
        let uid = libc::getuid();
        let gid = libc::getgid();
        libc::setresuid(uid, uid, uid);
        libc::setresgid(gid, gid, gid);
        libc::execve(program_path_cstring.as_ptr(), argv, new_envp);
    }
    init_rtld_audit_interface
};

fn get_argc(args: Vec<db::ArgumentRow>) -> usize {
    let mut argc = 0;
    for arg in args {
        if arg.parent.is_none() {
            argc += 1;
        }
    }
    argc
}

#[allow(unused_mut)]
unsafe extern "C" fn generic_hook (mut arg1: usize, mut args: ...) -> isize {
    // TODO: Test zero argument case
    /*
    Notes on limitations of WhiteBeam's generic Linux hook, planned to be resolved in future versions of WhiteBeam:
    - Can receive any function call and arguments, but hardcoded to call functions with up to 6 arguments
      (supports 1,587 out of 1,589 glibc functions)
    - 6 out of 1,589 glibc functions are unsupported due to no VaList equivalent
      (argp_failure, fcntl, ioctl, makecontext, strfmon, syscall, and ulimit)
    - No known security implications while Execution and Filesystem hooks are enforcing prevention mode
    */
    // Program
    let src_prog: String = { CUR_PROG.lock().expect("WhiteBeam: Failed to lock mutex").clone().into_string().expect("WhiteBeam: Invalid executable name") };
    // Hook
    let stack_hook: i64 = { FN_STACK.lock().expect("WhiteBeam: Failed to lock mutex").pop().expect("WhiteBeam: Lost track of environment") };
    let mut hook: db::HookRow  = {
        let hook_cache_lock = db::HOOK_CACHE.lock().expect("WhiteBeam: Failed to lock mutex");
        let hook_option = hook_cache_lock.iter().find(|hook| hook.id == stack_hook);
        hook_option.expect("WhiteBeam: Lost track of environment").clone()
    };
    // Arguments
    let mut arg_vec: Vec<db::ArgumentRow> = {
        let arg_cache_lock = db::ARG_CACHE.lock().expect("WhiteBeam: Failed to lock mutex");
        arg_cache_lock.iter().filter(|arg| arg.hook == stack_hook).map(|arg| arg.clone()).collect()
    };
    // TODO: Pass by reference/slice
    let mut argc: usize = get_argc(arg_vec.clone());
    if argc > 0 {
        arg_vec[0].real = arg1 as usize;
        let mut next_arg: usize = 0 as usize;
        for i in 1..argc {
            next_arg = args.arg();
            arg_vec[i].real = next_arg as usize;
        }
    }
    // Rules
    let mut rules: Vec<db::RuleRow> = {
        let rule_cache_lock = db::RULE_CACHE.lock().expect("WhiteBeam: Failed to lock mutex");
        let all_arg_ids: Vec<i64> = arg_vec.iter().map(|arg| arg.id).collect();
        rule_cache_lock.iter().filter(|rule| all_arg_ids.contains(&rule.arg)).map(|rule| rule.clone()).collect()
    };
    // Actions
    for rule in rules {
        // TODO: Eliminate redundancy
        // TODO: Is clone needed?
        let (hook_new, arg_vec_new, do_return, return_value) = action::process_action(src_prog.clone(), rule.clone(), hook.clone(), arg_vec.clone());
        hook = hook_new;
        arg_vec = arg_vec_new;
        if do_return {
            return return_value;
        }
    };
    // Dispatch
    static mut REAL: *const u8 = 0 as *const u8;
    static mut ONCE: ::std::sync::Once = ::std::sync::Once::new();
    ONCE.call_once(|| {
        REAL = crate::platforms::linux::dlsym_next(&hook.symbol);
    });
    let hooked_fn_zargs_real: unsafe extern "C" fn() -> isize = std::mem::transmute(REAL);
    let hooked_fn_margs_real: unsafe extern "C" fn(arg1: usize, args: ...) -> isize = std::mem::transmute(REAL);
    // TODO: Pass by reference/slice
    argc = get_argc(arg_vec.clone());
    match argc {
        0 => return hooked_fn_zargs_real(),
        1 => return hooked_fn_margs_real(arg_vec[0].real),
        2 => return hooked_fn_margs_real(arg_vec[0].real, arg_vec[1].real),
        3 => return hooked_fn_margs_real(arg_vec[0].real, arg_vec[1].real, arg_vec[2].real),
        4 => return hooked_fn_margs_real(arg_vec[0].real, arg_vec[1].real, arg_vec[2].real, arg_vec[3].real),
        5 => return hooked_fn_margs_real(arg_vec[0].real, arg_vec[1].real, arg_vec[2].real, arg_vec[3].real, arg_vec[4].real),
        6 => return hooked_fn_margs_real(arg_vec[0].real, arg_vec[1].real, arg_vec[2].real, arg_vec[3].real, arg_vec[4].real, arg_vec[5].real),
        // Unsupported
        _ => panic!("WhiteBeam: Unsupported operation"),
    }
}

// la_version
#[no_mangle]
unsafe extern "C" fn la_version(version: libc::c_uint) -> libc::c_uint {
	version
}

// la_objsearch
#[no_mangle]
unsafe extern "C" fn la_objsearch(name: *const libc::c_char, _cookie: libc::uintptr_t, _flag: libc::c_uint) -> *const libc::c_char {
    // TODO: Whitelisting
    //libc::printf("WhiteBeam objsearch: %s\n\0".as_ptr() as *const libc::c_char, name);
	name
}

// la_objopen
#[no_mangle]
unsafe extern "C" fn la_objopen(map: *const LinkMap, _lmid: libc::c_long, _cookie: libc::uintptr_t) -> libc::c_uint {
    //libc::printf("WhiteBeam objopen: %s\n\0".as_ptr() as *const libc::c_char, (*map).l_name);
    // TODO: Capture library for generic hook (map.l_name:cookie)
    LA_FLG_BINDTO | LA_FLG_BINDFROM
}

// la_symbind32
#[no_mangle]
unsafe extern "C" fn la_symbind32(sym: *const Elf32_Sym, _ndx: libc::c_uint,
                                      _refcook: *const libc::uintptr_t, _defcook: *const libc::uintptr_t,
                                      _flags: *const libc::c_uint, symname: *const libc::c_char) -> libc::uintptr_t {
    //libc::printf("WhiteBeam symbind32: %s\n\0".as_ptr() as *const libc::c_char, symname);
	(*(sym)).st_value as usize
}

// la_symbind64
#[no_mangle]
unsafe extern "C" fn la_symbind64(sym: *const Elf64_Sym, _ndx: libc::c_uint,
                                      _refcook: *const libc::uintptr_t, _defcook: *const libc::uintptr_t,
                                      _flags: *const libc::c_uint, symname: *const libc::c_char) -> libc::uintptr_t {
    // Warning: The Rust standard library is not guaranteed to be available during this function
    //libc::printf("WhiteBeam symbind64: %s\n\0".as_ptr() as *const libc::c_char, symname);
    let symbol_string = String::from(CStr::from_ptr(symname).to_str().expect("WhiteBeam: Unexpected null reference"));
    {
        let hook_cache_lock = db::HOOK_CACHE.lock().expect("WhiteBeam: Failed to lock mutex");
        let hook_cache_iter = hook_cache_lock.iter();
        for hook in hook_cache_iter {
            // TODO: Library match
            if hook.symbol == symbol_string {
                //libc::printf("WhiteBeam hook: %s\n\0".as_ptr() as *const libc::c_char, symname);
                {
                    FN_STACK.lock().unwrap().push(hook.id);
                };
                return generic_hook as usize
            }
        }
    };
	(*(sym)).st_value as usize
}

#[link(name = "dl")]
extern "C" {
    fn dlsym(handle: *const c_void, symbol: *const c_char) -> *const c_void;
}

pub unsafe fn dlsym_next(symbol: &str) -> *const u8 {
    let symbol_cstring: CString = CString::new(symbol).expect("WhiteBeam: Unexpected null reference");
    let ptr = dlsym(RTLD_NEXT, symbol_cstring.as_ptr() as *const c_char);
    if ptr.is_null() {
        panic!("WhiteBeam: Unable to find underlying function for {}", symbol);
    }
    ptr as *const u8
}

pub fn get_data_file_path(data_file: &str) -> PathBuf {
    #[cfg(feature = "whitelist_test")]
    let data_path: String = format!("{}/target/release/examples/", env!("PWD"));
    #[cfg(not(feature = "whitelist_test"))]
    let data_path: String = String::from("/opt/WhiteBeam/data/");
    let data_file_path = data_path + data_file;
    PathBuf::from(data_file_path)
}

pub fn get_rtld_audit_lib_path() -> PathBuf {
    #[cfg(feature = "whitelist_test")]
    let rtld_audit_lib_path = PathBuf::from(format!("{}/target/release/libwhitebeam.so", env!("PWD")));
    #[cfg(not(feature = "whitelist_test"))]
    let rtld_audit_lib_path = PathBuf::from(format!("/lib/libwhitebeam_{}.so", env::consts::ARCH));
    rtld_audit_lib_path
}

pub unsafe fn errno_location() -> *mut c_int {
    libc::__errno_location()
}


pub fn get_current_gid() -> u32 {
    unsafe { libc::getgid() }
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
        let mut stat_struct: libc::stat = unsafe { std::mem::zeroed() };
        let c_path = convert::osstr_to_cstring(path.as_os_str()).expect("WhiteBeam: Unexpected null reference");
        let path_stat = match unsafe { libc::stat(c_path.as_ptr(), &mut stat_struct) } {
          0 => Ok(stat_struct),
          _ => Err(unsafe { errno_location() }),
        };
        match path_stat {
            Ok(valid_path) => {
                if (valid_path.st_mode & libc::S_IFMT)==libc::S_IFREG {
                    return Some(path);
                }
            }
            Err(_) => {}
        }
    }
    None
}

pub unsafe fn environ() -> *const *const c_char {
    extern "C" {
        static environ: *const *const c_char;
    }
    environ
}
