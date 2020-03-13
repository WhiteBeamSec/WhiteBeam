/*
       int fexecve(int fd, char *const argv[], char *const envp[]);
*/
#[no_mangle]
pub unsafe extern "C" fn fexecve(fd: libc::c_int, argv: *const *const libc::c_char, envp: *const *const libc::c_char) -> libc::c_int {
	let program = std::ffi::OsStr::new("fd");
    let env = crate::platforms::linux::parse_env_collection(envp);
    let hexdigest = crate::common::hash::common_hash_fd(fd);
    let uid = crate::platforms::linux::get_current_uid();
    // Permit/deny execution
    if !crate::common::whitelist::is_whitelisted(program, &env, &hexdigest) {
		crate::common::event::send_exec_event(uid, program, &hexdigest, false);
		*crate::platforms::linux::errno_location() = libc::EACCES;
		return -1
	}
    crate::common::event::send_exec_event(uid, program, &hexdigest, true);
	call_real!{ fexecve (fd: libc::c_int, argv: *const *const libc::c_char, envp: *const *const libc::c_char) -> libc::c_int }
}
