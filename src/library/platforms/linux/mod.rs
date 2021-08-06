// Load OS-specific modules

use crate::common::{convert,
                    db};
use libc::{c_char, c_int, c_void};
use std::{collections::BTreeMap,
          env,
          ffi::CStr,
          ffi::CString,
          ffi::OsStr,
          ffi::OsString,
          io::prelude::*,
          os::unix::ffi::OsStrExt,
          os::unix::ffi::OsStringExt,
          path::PathBuf,
          lazy::SyncLazy,
          sync::RwLock};

const LA_FLG_BINDTO: libc::c_uint = 0x01;
const LA_FLG_BINDFROM: libc::c_uint = 0x02;

static LIB_MAP: SyncLazy<RwLock<BTreeMap<usize, &str>>> = SyncLazy::new(|| RwLock::new(BTreeMap::new()));

// LinkMap TODO: Review mut, assign libc datatypes? Upstream into Rust libc
#[repr(C)]
pub struct LinkMap {
    pub l_addr: usize,
    pub l_name: *const libc::c_char,
    pub l_ld: usize,
    pub l_next: *mut LinkMap,
    pub l_prev: *mut LinkMap
}

// Debug: Cause a breakpoint exception by invoking the `int3` instruction.
//pub fn int3() { unsafe { asm!("int3"); } }

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
                let procfs_ld_audit = procfs_getenv("LD_AUDIT");
                if procfs_ld_audit.is_some() &&
                   convert::osstr_split_at_byte(&procfs_ld_audit.expect("WhiteBeam: Unexpected null reference"), b':').0 == rtld_audit_lib_path {
                    // The dynamic linker deleted LD_AUDIT (secure binary) but we're still loaded
                    std::env::set_var("LD_AUDIT", rtld_audit_lib_path);
                    OsString::new()
                } else {
                update_ld_audit = true;
                let mut new_ld_audit_osstring = OsString::from("LD_AUDIT=");
                new_ld_audit_osstring.push(rtld_audit_lib_path.as_os_str());
                new_ld_audit_osstring
            }
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
                let mut cur_prog_lock = crate::common::hook::CUR_PROG.lock().expect("WhiteBeam: Failed to lock mutex");
                cur_prog_lock.clear();
                cur_prog_lock.push(&val);
                val
            },
            None => {
                // TODO: Is proc mounted early enough? May need some combination of the canonicalized argv[0] and exe
                match std::env::current_exe() {
                    Ok(v) => {
                        v.into_os_string()
                    },
                    Err(_e) => {
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

// la_version
#[no_mangle]
unsafe extern "C" fn la_version(version: libc::c_uint) -> libc::c_uint {
	version
}

// la_objsearch
#[no_mangle]
unsafe extern "C" fn la_objsearch(name: *const libc::c_char, _cookie: libc::uintptr_t, _flag: libc::c_uint) -> *const libc::c_char {
    let src_prog: String = { crate::common::hook::CUR_PROG.lock().expect("WhiteBeam: Failed to lock mutex").clone().into_string().expect("WhiteBeam: Invalid executable name") };
    let any = String::from("ANY");
    let class = String::from("Filesystem/Path/Library");
    let all_allowed_library_paths: Vec<String> = {
        let whitelist_cache_lock = crate::common::db::WL_CACHE.lock().expect("WhiteBeam: Failed to lock mutex");
        whitelist_cache_lock.iter().filter(|whitelist| (whitelist.class == class) && ((whitelist.path == src_prog) || (whitelist.path == any))).map(|whitelist| whitelist.value.clone()).collect()
    };
    let all_allowed_library_names: Vec<String> = all_allowed_library_paths.iter()
                                                                          .filter_map(|lib| std::path::Path::new(lib).file_name())
                                                                          .filter_map(|filename| filename.to_str())
                                                                          .map(|filename_str| String::from(filename_str))
                                                                          .collect();
    // Permit ANY
    if all_allowed_library_paths.iter().any(|library| library == &any) {
        return name;
    }
    let target_library = String::from(CStr::from_ptr(name).to_str().expect("WhiteBeam: Unexpected null reference"));
    // Permit whitelisted libraries
    if all_allowed_library_names.iter().any(|library| library == &target_library) {
        return name;
    }
    if all_allowed_library_paths.iter().any(|library| library == &target_library) {
        return name;
    }
    if !(crate::common::db::get_prevention()) {
        // TODO: Check if file exists?
        crate::common::event::send_log_event(crate::common::event::LogClass::Info as i64, format!("Detection: {} executed {} (la_objsearch)", &src_prog, &target_library));
        return name;
    }
    // Permit authorized execution
    if crate::common::db::get_valid_auth_env() {
        return name;
    }
    // Deny by default
    crate::common::event::send_log_event(crate::common::event::LogClass::Warn as i64, format!("Prevention: Blocked {} from executing {} (la_objsearch)", &src_prog, &target_library));
	0 as *const libc::c_char
}

// la_objopen
#[no_mangle]
unsafe extern "C" fn la_objopen(map: *const LinkMap, _lmid: libc::c_long, cookie: libc::uintptr_t) -> libc::c_uint {
    //libc::printf("WhiteBeam objopen: %s\n\0".as_ptr() as *const libc::c_char, (*map).l_name);
    let library_string = CStr::from_ptr((*map).l_name).to_str().expect("WhiteBeam: Unexpected null reference");
    {
        match LIB_MAP.write() {
                Ok(mut m) => {
                        m.insert(cookie, library_string);
                }
                Err(_e) => { panic!("WhiteBeam: Failed to acquire write lock in la_objopen"); }
        }
    }
    LA_FLG_BINDTO | LA_FLG_BINDFROM
}

// la_objclose
// TODO: Remove key *cookie from LIB_MAP

// la_symbind32
#[no_mangle]
unsafe extern "C" fn la_symbind32(sym: *const libc::Elf32_Sym, _ndx: libc::c_uint,
                                      _refcook: *const libc::uintptr_t, _defcook: *const libc::uintptr_t,
                                      _flags: *const libc::c_uint, symname: *const libc::c_char) -> libc::uintptr_t {
    //libc::printf("WhiteBeam symbind32: %s\n\0".as_ptr() as *const libc::c_char, symname);
	(*(sym)).st_value as usize
}

// la_symbind64
#[no_mangle]
unsafe extern "C" fn la_symbind64(sym: *const libc::Elf64_Sym, _ndx: libc::c_uint,
                                      refcook: *const libc::uintptr_t, defcook: *const libc::uintptr_t,
                                      _flags: *const libc::c_uint, symname: *const libc::c_char) -> libc::uintptr_t {
    // Warning: The Rust standard library is not guaranteed to be available during this function
    //libc::printf("WhiteBeam symbind64: %s\n\0".as_ptr() as *const libc::c_char, symname);
    let symbol_str = CStr::from_ptr(symname).to_str().expect("WhiteBeam: Unexpected null reference");
    let empty: &str = "";
    let calling_library_str: &str = match LIB_MAP.read() {
        Ok(lib_map_lock) => { match lib_map_lock.get(&(refcook as libc::uintptr_t)) { Some(cook) => cook, None => empty } }
        Err(_e) => { panic!("WhiteBeam: Failed to acquire read lock in la_symbind64"); /* empty */ }
    };
    let library_str: &str = match LIB_MAP.read() {
        Ok(lib_map_lock) => { match lib_map_lock.get(&(defcook as libc::uintptr_t)) { Some(cook) => cook, None => empty } }
        Err(_e) => { panic!("WhiteBeam: Failed to acquire read lock in la_symbind64"); /* empty */ }
    };
    // FIXME: Hack around libpam issue
    if (calling_library_str == "/lib/x86_64-linux-gnu/libpam.so.0") && (symbol_str == "dlopen") {
        return (*(sym)).st_value as usize;
    }
    {
        let hook_cache_lock = db::HOOK_CACHE.lock().expect("WhiteBeam: Failed to lock mutex");
        let hook_cache_iter = hook_cache_lock.iter();
        for hook in hook_cache_iter {
            // TODO: Library match
            if (hook.symbol == symbol_str) && (hook.library == library_str) {
                //libc::printf("WhiteBeam hook: %s\n\0".as_ptr() as *const libc::c_char, symname);
                {
                    let addr = (*(sym)).st_value as usize;
                    crate::common::hook::FN_STACK.lock().unwrap().push((hook.id, addr));
                };
                return crate::common::hook::generic_hook as usize
            }
        }
    };
	(*(sym)).st_value as usize
}

pub unsafe fn dlsym_next(symbol: &str) -> *const u8 {
    let symbol_cstring: CString = CString::new(symbol).expect("WhiteBeam: Unexpected null reference");
    let ptr = libc::dlsym(libc::RTLD_NEXT, symbol_cstring.as_ptr() as *const c_char);
    if ptr.is_null() {
        panic!("WhiteBeam: Unable to find underlying function for {}", symbol);
    }
    ptr as *const u8
}

#[allow(non_snake_case)]
pub unsafe fn dlsym_next_relative(symbol: &str, real_addr: usize) -> *const u8 {
    // real_addr.base+dlsym_addr.st_addr
    // TODO: dlopen(NULL)?
    let RTLD_DL_SYMENT: libc::c_int = 1;
    let symbol_cstring: CString = CString::new(symbol).expect("WhiteBeam: Unexpected null reference");
    let mut dl_info_dlsym = libc::Dl_info {
        dli_fname: core::ptr::null(),
        dli_fbase: core::ptr::null_mut(),
        dli_sname: core::ptr::null(),
        dli_saddr: core::ptr::null_mut(),
    };
    let mut dl_info_real = libc::Dl_info {
        dli_fname: core::ptr::null(),
        dli_fbase: core::ptr::null_mut(),
        dli_sname: core::ptr::null(),
        dli_saddr: core::ptr::null_mut(),
    };
    let mut dl_info_verify = libc::Dl_info {
        dli_fname: core::ptr::null(),
        dli_fbase: core::ptr::null_mut(),
        dli_sname: core::ptr::null(),
        dli_saddr: core::ptr::null_mut(),
    };
    let mut extra_info_dlsym = std::mem::MaybeUninit::<*mut libc::Elf64_Sym>::uninit();
    let dlsym_addr = libc::dlsym(libc::RTLD_NEXT, symbol_cstring.as_ptr() as *const c_char);
    if dlsym_addr.is_null() {
        panic!("WhiteBeam: Unable to find underlying function for {}", symbol);
    }
    let real_addr_base: usize = match libc::dladdr(real_addr as *const c_void, &mut dl_info_real as *mut libc::Dl_info) {
        0 => panic!("WhiteBeam: dladdr failed"),
        _ => dl_info_real.dli_fbase as usize
    };
    let dlsym_addr_st_addr: usize = match libc::dladdr1(dlsym_addr as *const c_void, &mut dl_info_dlsym as *mut libc::Dl_info, extra_info_dlsym.as_mut_ptr() as *mut *mut libc::c_void, RTLD_DL_SYMENT) {
        0 => panic!("WhiteBeam: dladdr1 failed"),
        _ => {
            let extra_info_dlsym_init = extra_info_dlsym.assume_init();
            (*extra_info_dlsym_init).st_value as usize
        }
    };
    let calculated_addr = (real_addr_base+dlsym_addr_st_addr) as *const u8;
    match libc::dladdr(calculated_addr as *const c_void, &mut dl_info_verify as *mut libc::Dl_info) {
        0 => panic!("WhiteBeam: dladdr failed"),
        _ => {
            if !(dl_info_verify.dli_sname.is_null()) {
                let sname = String::from(CStr::from_ptr(dl_info_verify.dli_sname).to_str().expect("WhiteBeam: Unexpected null reference"));
                // TODO: Sometimes ftruncate64 resolves to ftruncate
                assert!((symbol == &sname) || (symbol == "ftruncate64" && &sname == "ftruncate"))
            } else {
                // Fallback on RTLD_NEXT
                // TODO: Determine why this gets called
                return dlsym_next(symbol);
            }
        }
    };
    calculated_addr
}

pub fn get_data_file_path_string(data_file: &str) -> String {
    #[cfg(feature = "whitelist_test")]
    let data_path: String = format!("{}/target/release/examples/", env!("PWD"));
    #[cfg(not(feature = "whitelist_test"))]
    let data_path: String = String::from("/opt/WhiteBeam/data/");
    let data_file_path = data_path + data_file;
    data_file_path
}

pub fn get_data_file_path(data_file: &str) -> PathBuf {
    PathBuf::from(get_data_file_path_string(data_file))
}

pub fn get_rtld_audit_lib_path() -> PathBuf {
    #[cfg(feature = "whitelist_test")]
    let rtld_audit_lib_path = PathBuf::from(format!("{}/target/release/libwhitebeam.so", env!("PWD")));
    #[cfg(not(feature = "whitelist_test"))]
    let rtld_audit_lib_path = PathBuf::from(String::from("libwhitebeam.so"));
    rtld_audit_lib_path
}

pub unsafe fn errno_location() -> *mut c_int {
    libc::__errno_location()
}

pub fn canonicalize_fd(fd: i32) -> Option<PathBuf> {
    // TODO: Better validation here
    if (0 <= fd) && (fd <= 1024) {
        // TODO: Remove dependency on procfs
        return std::fs::read_link(format!("/proc/self/fd/{}", fd)).ok();
    }
    None
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
                if (valid_path.st_mode & libc::S_IFMT) == libc::S_IFREG {
                    return Some(path);
                }
            }
            Err(_) => {}
        }
    }
    None
}

pub fn procfs_getenv(env_var: &str) -> Option<OsString> {
    for env_osstring_tuple in procfs_environ() {
        if env_osstring_tuple.0 == OsString::from(env_var) {
            return Some(env_osstring_tuple.1);
        }
    }
    None
}

pub fn procfs_environ() -> Vec<(OsString, OsString)> {
    // TODO: Test for environment without =
    let mut environ = std::fs::File::open("/proc/self/environ").expect("WhiteBeam: Lost track of environment");
    let mut environ_contents: Vec<u8> = vec![];
    environ.read_to_end(&mut environ_contents).expect("WhiteBeam: Unexpected null reference");
    let env_osstring_vec: Vec<OsString> = environ_contents.split(|&byte| byte == b'\0').map(|slice| OsStringExt::from_vec(slice.to_vec())).collect();
    env_osstring_vec.iter().map(|env_osstring| convert::osstr_split_at_byte(env_osstring, b'=')).collect::<Vec<(&OsStr, &OsStr)>>()
                    .iter().map(|env_osstr_tuple| (env_osstr_tuple.0.to_owned(), env_osstr_tuple.1.to_owned())).collect()
}

pub unsafe fn environ() -> *const *const c_char {
    extern "C" {
        static environ: *const *const c_char;
    }
    environ
}
