// Load OS-specific modules

mod support;
use support::*;
use crate::common::{convert,
                    db};
use libc::{c_char, c_int};
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
          sync::RwLock};

pub static IS_PRELOAD: RwLock<Option<bool>> = RwLock::new(None);
static LIB_MAP: RwLock<BTreeMap<usize, &str>> = RwLock::new(BTreeMap::new());
pub static RT_SIGNAL: RwLock<i32> = RwLock::new(0);

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
        let rtld_audit_lib_path = get_rtld_audit_lib_path();
        // la_symbind*() doesn't get called when LD_BIND_NOW is set
        // More info: https://sourceware.org/bugzilla/show_bug.cgi?id=23734
        if env::var_os("LD_BIND_NOW").is_some() {
            // Technically we're looking for a non-empty string here, but instead we deny it altogether
            eprintln!("WhiteBeam: LD_BIND_NOW restricted");
            std::process::exit(1);
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
                    // Init
                    if libc::getpid() == 1 {
                        return
                    }
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
                val
            },
            None => {
                let execfn = libc::getauxval(libc::AT_EXECFN) as *const libc::c_char;
                // A null pointer here will return an empty OsString, resulting in a strict whitelist
                crate::common::convert::c_char_to_osstring(execfn)
            }
        };
        if !(is_preload_lib()) {
            let mut cur_prog_lock = crate::common::hook::CUR_PROG.lock().expect("WhiteBeam: Failed to lock mutex");
            cur_prog_lock.clear();
            cur_prog_lock.push(&program_path);
        }
        if !(update_ld_audit) && !(update_ld_bind_not) {
            // Nothing to do, continue execution
            return;
        }
        // TODO: Log null reference, process errors
        let program_path_cstring = convert::osstr_to_cstring(&program_path).expect("WhiteBeam: Unexpected null reference");
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
        if !(envp.is_null()) {
            let mut envp_iter = envp;
            while !(*envp_iter).is_null() {
                if let Some(key_value) = convert::parse_env_single(CStr::from_ptr(*envp_iter).to_bytes()) {
                    if  (!(update_ld_audit) && (key_value.0 == "LD_AUDIT"))
                     || (!(update_ld_bind_not) && (key_value.0 == "LD_BIND_NOT"))
                     || ((key_value.0 != "LD_AUDIT") && (key_value.0 != "LD_BIND_NOT")) {
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

fn realtime_cache_init() {
    let mut act: libc::sigaction = unsafe { std::mem::zeroed() };
    let notify_signal: libc::c_int = libc::SIGRTMIN();
    {
        let mut rt_signal_lock = RT_SIGNAL.write().expect("WhiteBeam: Failed to lock mutex");
        *rt_signal_lock = notify_signal;
    }
    act.sa_sigaction = db::populate_cache as usize;
    unsafe { libc::sigemptyset(&mut act.sa_mask) };
    act.sa_flags = libc::SA_SIGINFO | libc::SA_RESTART;
    if unsafe { libc::sigaction(notify_signal, &act, std::ptr::null_mut()) } == -1 {
        panic!("WhiteBeam: Lost track of environment");
    };

    let mut realtime_folder_path: String = get_realtime_file_path_string("");
    realtime_folder_path.push('\0');
    unsafe {
        let fd: libc::c_int = libc::open(realtime_folder_path.as_ptr() as *const libc::c_char, libc::O_RDONLY);
        if fd == -1 {
            panic!("WhiteBeam: Cannot open realtime database path");
        };
        if libc::fcntl(fd, F_SETSIG, notify_signal) == -1 {
            panic!("WhiteBeam: Lost track of environment");
        };
        if libc::fcntl(fd, libc::F_NOTIFY, DN_CREATE | DN_MULTISHOT) == -1 {
            // TODO: What condition causes this? Namespaces? Log main errno.
            let par_prog: String = { crate::common::hook::PAR_PROG.lock().expect("WhiteBeam: Failed to lock mutex").clone().into_string().expect("WhiteBeam: Invalid executable name") };
            let src_prog: String = { crate::common::hook::CUR_PROG.lock().expect("WhiteBeam: Failed to lock mutex").clone().into_string().expect("WhiteBeam: Invalid executable name") };
            crate::common::event::send_log_event(libc::LOG_ERR, format!("Real-time cache: Failed to setup dnotify signaling in {} -> {}", &par_prog, &src_prog));
            //panic!("WhiteBeam: Lost track of environment");
        };
    }
}

// la_version
#[no_mangle]
unsafe extern "C" fn la_version(version: libc::c_uint) -> libc::c_uint {
    // This variable is protected by WhiteBeam's Essential hooks/rules
    // NB: There are cases where WB_PARENT is undefined, such as pid 1
    if let Some(val) = env::var_os("WB_PARENT") {
        let mut par_prog_lock = crate::common::hook::PAR_PROG.lock().expect("WhiteBeam: Failed to lock mutex");
        par_prog_lock.clear();
        par_prog_lock.push(&val);
        env::remove_var("WB_PARENT");
    } else {
        if libc::getppid() == 1 {
            let mut par_prog_lock = crate::common::hook::PAR_PROG.lock().expect("WhiteBeam: Failed to lock mutex");
            par_prog_lock.clear();
            par_prog_lock.push("init");
        }
    }
    // This variable is protected by WhiteBeam's Essential hooks/rules
    if env::var_os("WB_PROG").is_some() {
        env::remove_var("WB_PROG");
    }
    // Populate cache
    db::populate_cache().expect("WhiteBeam: Could not access database");
    // Refresh cache real-time
    let src_prog: String = { crate::common::hook::CUR_PROG.lock().expect("WhiteBeam: Failed to lock mutex").clone().into_string().expect("WhiteBeam: Invalid executable name") };
    #[cfg(feature = "whitelist_test")]
    if !(src_prog.ends_with("whitebeam")) {
        realtime_cache_init();
    }
    #[cfg(not(feature = "whitelist_test"))]
    if (src_prog != "/opt/WhiteBeam/whitebeam") &&
       (src_prog != "/usr/local/bin/whitebeam") {
        realtime_cache_init();
    }
    version
}

fn mask_sigrt(masked: bool) {
    // Mask realtime signals
    let rt_signal_number: i32 = match RT_SIGNAL.read() {
        Ok(lock) => {
            if *lock == 0 {
                libc::SIGRTMIN()
            } else {
                *lock
            }
        },
        Err(_e) => {
            libc::SIGRTMIN()
        }
    };
    let mut sig_mask: libc::sigset_t = unsafe { std::mem::zeroed() };
    unsafe {
        libc::sigemptyset(&mut sig_mask);
        if libc::sigaddset(&mut sig_mask, rt_signal_number) == -1 {
            panic!("WhiteBeam: Lost track of environment");
        };
        if masked {
            if libc::pthread_sigmask(libc::SIG_BLOCK, &sig_mask, std::ptr::null_mut()) == -1 {
                panic!("WhiteBeam: Lost track of environment");
            };
        } else {
            if libc::pthread_sigmask(libc::SIG_UNBLOCK, &sig_mask, std::ptr::null_mut()) == -1 {
                panic!("WhiteBeam: Lost track of environment");
            };
        }
    }
}

// la_objsearch
#[no_mangle]
unsafe extern "C" fn la_objsearch(name: *const libc::c_char, _cookie: *const libc::uintptr_t, _flag: libc::c_uint) -> *const libc::c_char {
    mask_sigrt(true);
    let par_prog: String = { crate::common::hook::PAR_PROG.lock().expect("WhiteBeam: Failed to lock mutex").clone().into_string().expect("WhiteBeam: Invalid executable name") };
    let src_prog: String = { crate::common::hook::CUR_PROG.lock().expect("WhiteBeam: Failed to lock mutex").clone().into_string().expect("WhiteBeam: Invalid executable name") };
    let any = String::from("ANY");
    let class = String::from("Filesystem/Path/Library");
    let all_allowed_library_paths: Vec<String> = {
        let whitelist_cache_lock = crate::common::db::WL_CACHE.lock().expect("WhiteBeam: Failed to lock mutex");
        whitelist_cache_lock.iter().filter(|whitelist| (whitelist.class == class) && ((whitelist.parent == par_prog) || (whitelist.parent == any)) && ((whitelist.path == src_prog) || (whitelist.path == any))).map(|whitelist| whitelist.value.clone()).collect()
    };
    let all_allowed_library_names: Vec<String> = all_allowed_library_paths.iter()
                                                                          .filter_map(|lib| std::path::Path::new(lib).file_name())
                                                                          .filter_map(|filename| filename.to_str())
                                                                          .map(|filename_str| String::from(filename_str))
                                                                          .collect();
    // Permit ANY
    if all_allowed_library_paths.iter().any(|library| library == &any) {
        mask_sigrt(false);
        return name;
    }
    let target_library = String::from(CStr::from_ptr(name).to_str().expect("WhiteBeam: Unexpected null reference"));
    // Permit whitelisted libraries
    if all_allowed_library_names.iter().any(|library| library == &target_library) {
        // TODO: Check this only if la_objsearch is LA_SER_ORIG?
        mask_sigrt(false);
        return name;
    }
    if all_allowed_library_paths.iter().any(|library| library == &target_library) {
        mask_sigrt(false);
        return name;
    }
    if !(crate::common::db::get_prevention()) {
        // TODO: Check if file exists?
        crate::common::event::send_log_event(libc::LOG_NOTICE, format!("Detection: {} -> {} executed {} (la_objsearch)", &par_prog, &src_prog, &target_library));
        mask_sigrt(false);
        return name;
    }
    // Permit authorized execution
    if crate::common::db::get_valid_auth_env() {
        mask_sigrt(false);
        return name;
    }
    // Deny by default
    crate::common::event::send_log_event(libc::LOG_WARNING, format!("Prevention: Blocked {} -> {} from executing {} (la_objsearch)", &par_prog, &src_prog, &target_library));
    mask_sigrt(false);
    0 as *const libc::c_char
}

// la_objopen
#[no_mangle]
unsafe extern "C" fn la_objopen(map: *const LinkMap, _lmid: libc::c_long, cookie: *const libc::uintptr_t) -> libc::c_uint {
    //libc::printf("WhiteBeam objopen: %s\n\0".as_ptr() as *const libc::c_char, (*map).l_name);
    let library_string = CStr::from_ptr((*map).l_name).to_str().expect("WhiteBeam: Unexpected null reference");
    {
        match LIB_MAP.write() {
                Ok(mut m) => {
                        m.insert(*cookie, library_string);
                }
                Err(_e) => { panic!("WhiteBeam: Failed to acquire write lock in la_objopen"); }
        }
    }
    LA_FLG_BINDTO | LA_FLG_BINDFROM
}

// la_objclose
// TODO: Remove key *cookie from LIB_MAP

// la_symbind32

// la_symbind64
#[cfg(feature = "whitelist_test")]
#[no_mangle]
unsafe extern "C" fn la_symbind64(sym: *const libc::Elf64_Sym, _ndx: libc::c_uint,
                                      refcook: *const libc::uintptr_t, defcook: *const libc::uintptr_t,
                                      _flags: *const libc::c_uint, symname: *const libc::c_char) -> libc::uintptr_t {
    let symbol_str = CStr::from_ptr(symname).to_str().expect("WhiteBeam: Unexpected null reference");
    if symbol_str == "is_hooked" {
        return is_hooked as usize;
    }
    return (*(sym)).st_value as usize;
}

// la_pltenter
/* TODO: Review mutability of data types, implement the following architectures:
- alpha
- arc
- arm
- csky
- hppa
- ia64
- m68k
- microblaze
- mips
- nios2
- powerpc
- riscv
- s390
- sh
- sparc
*/

unsafe fn plt_redirect(orig_addr: u64, refcook: *const libc::uintptr_t, defcook: *const libc::uintptr_t, symname: *const libc::c_char) -> u64 {
    // TODO: Option instead of empty str?
    let empty: &str = "";
    let calling_library_str: &str = match LIB_MAP.read() {
        Ok(lib_map_lock) => { match lib_map_lock.get(&(*refcook)) { Some(cook) => cook, None => empty } }
        Err(_e) => { panic!("WhiteBeam: Failed to acquire read lock in la_pltenter"); /* empty */ }
    };
    let library_path_str: &str = match LIB_MAP.read() {
        Ok(lib_map_lock) => { match lib_map_lock.get(&(*defcook)) { Some(cook) => cook, None => empty } }
        Err(_e) => { panic!("WhiteBeam: Failed to acquire read lock in la_pltenter"); /* empty */ }
    };
    let calling_library_basename_str: &str = calling_library_str.rsplit('/').next().unwrap_or(calling_library_str);
    let symbol_str = CStr::from_ptr(symname).to_str().expect("WhiteBeam: Unexpected null reference");
    if (*refcook) == 0 {
        return orig_addr;
    }
    // FIXME: Stability exception for fopen64
    match symbol_str {
        "fopen64" => {
            if calling_library_basename_str == "libcrypto.so.1.1" {
                return orig_addr;
            }
        }
        _ => ()
    };
    {
        let hook_cache_lock = db::HOOK_CACHE.lock().expect("WhiteBeam: Failed to lock mutex");
        // TODO: Use .find() instead
        let hook_cache_iter = hook_cache_lock.iter();
        for hook in hook_cache_iter {
            if (hook.symbol == symbol_str) && (hook.library == library_path_str) {
                {
                    // Get some information ahead of time of what the redirected symbol/library will be
                    let addr = match db::get_redirect(hook.id) {
                        Some(redirected_function) => { resolve_symbol(&redirected_function.0, &redirected_function.1) },
                        None => orig_addr as *const u8
                    };
                    crate::common::hook::FN_STACK.lock().unwrap().push((hook.id, addr as usize));
                };
                return crate::common::hook::generic_hook as u64
            }
        }
    };
    orig_addr
}

#[cfg(any(target_arch = "i386", target_arch = "i586", target_arch = "i686"))]
#[no_mangle]
unsafe extern "C" fn la_i86_gnu_pltenter(sym: *const libc::Elf32_Sym, _ndx: libc::c_uint,
                                          refcook: *const libc::uintptr_t, defcook: *const libc::uintptr_t, _regs: *const La_i86_regs,
                                          _flags: *const libc::c_uint, symname: *const libc::c_char, framesizep: *const libc::c_long) -> libc::Elf32_Addr {
    plt_redirect((*(sym)).st_value as u64, refcook, defcook, symname) as libc::Elf32_Addr
}

#[cfg(target_arch = "x86")]
#[no_mangle]
unsafe extern "C" fn la_x32_gnu_pltenter(sym: *const libc::Elf32_Sym, _ndx: libc::c_uint,
                                          refcook: *const libc::uintptr_t, defcook: *const libc::uintptr_t, _regs: *const La_x32_regs,
                                          _flags: *const libc::c_uint, symname: *const libc::c_char, framesizep: *const libc::c_long) -> libc::Elf32_Addr {
    plt_redirect((*(sym)).st_value as u64, refcook, defcook, symname) as libc::Elf32_Addr
}

#[cfg(target_arch = "x86_64")]
#[no_mangle]
unsafe extern "C" fn la_x86_64_gnu_pltenter(sym: *const libc::Elf64_Sym, _ndx: libc::c_uint,
                                              refcook: *const libc::uintptr_t, defcook: *const libc::uintptr_t, _regs: *const La_x86_64_regs,
                                              _flags: *const libc::c_uint, symname: *const libc::c_char, framesizep: *const libc::c_long) -> libc::Elf64_Addr {
    plt_redirect((*(sym)).st_value as u64, refcook, defcook, symname) as libc::Elf64_Addr
}

#[cfg(target_arch = "aarch64")]
#[no_mangle]
unsafe extern "C" fn la_aarch64_gnu_pltenter(sym: *const ElfW_Sym, _ndx: libc::c_uint,
                                              refcook: *const libc::uintptr_t, defcook: *const libc::uintptr_t, _regs: *const La_aarch64_regs,
                                              _flags: *const libc::c_uint, symname: *const libc::c_char, framesizep: *const libc::c_long) -> ElfW_Addr {
    plt_redirect((*(sym)).st_value as u64, refcook, defcook, symname) as ElfW_Addr
}

#[cfg(feature = "whitelist_test")]
#[no_mangle]
unsafe extern "C" fn is_hooked(library: *const libc::c_char, symbol: *const libc::c_char) -> libc::c_int {
    let library_str = CStr::from_ptr(library).to_str().expect("WhiteBeam: Unexpected null reference");
    let symbol_str = CStr::from_ptr(symbol).to_str().expect("WhiteBeam: Unexpected null reference");
    {
        let hook_cache_lock = db::HOOK_CACHE.lock().expect("WhiteBeam: Failed to lock mutex");
        // TODO: Use .find() instead
        let hook_cache_iter = hook_cache_lock.iter();
        for hook in hook_cache_iter {
            if (hook.symbol == symbol_str) && (hook.library == library_str) {
                return 1;
            }
        }
    }
    0
}

pub unsafe fn resolve_symbol(library: &str, symbol: &str) -> *const u8 {
    // TODO: Traverse link map, only if library isn't loaded run dlmopen (also test RTLD_NOLOAD on <2.35), otherwise dlsym in RTLD_DEFAULT
    // These may no longer be issues with pltenter, more tests needed:
    // - dlmopen() issue with sshd on x86_64
    // - returning libc::execve if symbol == "execve"
    let library_cstring: CString = CString::new(library).expect("WhiteBeam: Unexpected null reference");
    let symbol_cstring: CString = CString::new(symbol).expect("WhiteBeam: Unexpected null reference");
    let handle: *mut libc::c_void = libc::dlmopen(libc::LM_ID_BASE, library_cstring.as_ptr() as *const c_char, libc::RTLD_LAZY);
    if handle.is_null() {
        panic!("WhiteBeam: Unable to open handle for {}", library);
    }
    let fptr: *mut libc::c_void = libc::dlsym(handle, symbol_cstring.as_ptr() as *const c_char);
    if fptr.is_null() {
        panic!("WhiteBeam: Unable to find underlying function for {}", symbol);
    }
    fptr as *const u8
}

pub fn get_data_file_path_string(data_file: &str) -> String {
    #[cfg(feature = "whitelist_test")]
    let data_path: String = format!("{}/target/release/examples/data/", env!("PWD"));
    #[cfg(not(feature = "whitelist_test"))]
    let data_path: String = String::from("/opt/WhiteBeam/data/");
    let data_file_path = data_path + data_file;
    data_file_path
}

pub fn get_data_file_path(data_file: &str) -> PathBuf {
    PathBuf::from(get_data_file_path_string(data_file))
}

pub fn get_realtime_file_path_string(realtime_file: &str) -> String {
    #[cfg(feature = "whitelist_test")]
    let realtime_path: String = format!("{}/target/release/examples/realtime/", env!("PWD"));
    #[cfg(not(feature = "whitelist_test"))]
    let realtime_path: String = String::from("/opt/WhiteBeam/realtime/");
    let realtime_file_path = realtime_path + realtime_file;
    realtime_file_path
}

pub fn get_realtime_file_path(realtime_file: &str) -> PathBuf {
    PathBuf::from(get_realtime_file_path_string(realtime_file))
}

pub fn get_rtld_audit_lib_path() -> PathBuf {
    // TODO: Could this be discovered with dlopen self?
    #[cfg(feature = "whitelist_test")]
    let rtld_audit_lib_path = PathBuf::from(format!("{}/target/release/libwhitebeam.so", env!("PWD")));
    #[cfg(not(feature = "whitelist_test"))]
    let rtld_audit_lib_path = PathBuf::from(String::from("libwhitebeam.so"));
    rtld_audit_lib_path
}

pub fn locate_preload_symbol(symbol: &str) -> *mut libc::c_void {
    let preload_whitebeam_path: std::ffi::CString = crate::common::convert::osstr_to_cstring((get_rtld_audit_lib_path()).as_os_str()).expect("WhiteBeam: Unexpected null reference");
    let preload_whitebeam = unsafe { libc::dlmopen(libc::LM_ID_BASE, preload_whitebeam_path.as_ptr() as *const libc::c_char, libc::RTLD_LAZY | libc::RTLD_NOLOAD) };
    unsafe { libc::dlsym(preload_whitebeam, format!("{}\0", symbol).as_ptr() as *const libc::c_char) as *mut libc::c_void }
}

#[no_mangle]
pub unsafe extern "C" fn errno_location() -> *mut c_int {
    libc::__errno_location()
}

#[no_mangle]
pub unsafe extern "C" fn main_putenv(env_value: *mut c_char) -> c_int {
    libc::putenv(env_value as *mut c_char)
}

pub fn set_errno(errno_value: c_int) {
    let errno_location_addr = locate_preload_symbol("errno_location");
    let errno_location_fn: fn() -> *mut libc::c_int = unsafe { std::mem::transmute(errno_location_addr) };
    unsafe { *(errno_location_fn()) = errno_value };
}

pub fn put_env(env_value: *mut c_char) -> c_int {
    let putenv_addr = locate_preload_symbol("main_putenv");
    let putenv_fn: fn(*mut c_char) -> c_int = unsafe { std::mem::transmute(putenv_addr) };
    unsafe { putenv_fn(env_value) }
}

pub fn reflect_linker_errno() {
    set_errno(unsafe { *(errno_location()) });
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

pub fn gettid() -> u64 {
    unsafe { libc::syscall(libc::SYS_gettid) as u64 }
}

pub fn get_default_library_paths() -> OsString {
    if PathBuf::from("/lib64/libc.so.6").exists() {
        OsString::from("/lib64:/usr/lib64")
    } else {
        OsString::from("/lib:/usr/lib")
    }
}

pub fn search_path(program: &OsStr, paths: &OsStr) -> Option<PathBuf> {
    let mut paths_vec: Vec<PathBuf> = env::split_paths(paths).collect();
    if program.as_bytes()[0] == b'/' {
        // Absolute path
        let path = PathBuf::from(program);
        paths_vec.push(path);
    } else if program.as_bytes().contains(&b'/') {
        // Relative path with forward slashes
        match env::current_dir() {
            Ok(cwd) => paths_vec.push(cwd),
            Err(_val) => () // TODO: Log errors
        }
    }
    for mut path in paths_vec {
        if path != program {
            path.push(program);
        }
        let mut stat_struct: libc::stat = unsafe { std::mem::zeroed() };
        let c_path = convert::osstr_to_cstring(path.as_os_str()).expect("WhiteBeam: Unexpected null reference");
        let path_stat = match unsafe { libc::stat(c_path.as_ptr(), &mut stat_struct) } {
          0 => Ok(stat_struct),
          _ => Err(()),
        };
        match path_stat {
            Ok(valid_path) => {
                if ((valid_path.st_mode & libc::S_IFMT) == libc::S_IFREG) ||
                   ((valid_path.st_mode & libc::S_IFMT) == libc::S_IFLNK) {
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

pub unsafe fn linker_environ() -> *const *const c_char {
    extern "C" {
        static environ: *const *const c_char;
    }
    environ
}

#[no_mangle]
pub unsafe extern "C" fn main_environ() -> *const *const c_char {
    extern "C" {
        static environ: *const *const c_char;
    }
    environ
}

pub fn get_environ() -> *const *const c_char {
    let environ_addr = locate_preload_symbol("main_environ");
    let environ_fn: fn() -> *const *const c_char = unsafe { std::mem::transmute(environ_addr) };
    unsafe { environ_fn() }
}

extern "C" fn dl_iterate_phdr_callback(info: *mut libc::dl_phdr_info, size: libc::size_t, data: *mut libc::c_void) -> libc::c_int {
    let mut phdr_name = unsafe { CStr::from_ptr((*info).dlpi_name) };
    let vdso = CString::new("linux-vdso.so.1").expect("WhiteBeam: Unexpected null reference");
    if phdr_name == vdso.as_ref() {
        let mut is_preload_lock = IS_PRELOAD.write().expect("WhiteBeam: Failed to lock mutex");
        *is_preload_lock = Some(true);
        return 1
    }
    0
}

pub fn is_preload_lib() -> bool {
    if let Ok(is_preload_lock) = IS_PRELOAD.read() {
        if let Some(is_preload) = *is_preload_lock {
            return is_preload;
        }
    }
    // Search for linux-vdso.so.1
    unsafe { libc::dl_iterate_phdr(Some(dl_iterate_phdr_callback), std::ptr::null_mut()) == 1 }
}