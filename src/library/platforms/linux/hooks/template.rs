#[macro_export]

macro_rules! call_real {
    ($func_name:ident ( $($v:ident : $t:ty),* ) -> $rt:ty) => {
        static mut REAL: *const u8 = 0 as *const u8;
        static mut ONCE: ::std::sync::Once = ::std::sync::Once::new();
        ONCE.call_once(|| {
            REAL = crate::platforms::linux::dlsym_next(concat!(stringify!($func_name), "\0"));
        });
        let rust_func: unsafe extern "C" fn( $($v : $t),* ) -> $rt = std::mem::transmute(REAL);
        rust_func( $($v),* )
    }
}

// Exec hook template
macro_rules! build_exec_hook {
    (hook $func_name:ident ($program: ident) custom_routine $body:block) => {
        #[no_mangle]
        #[allow(unused_mut)]
        pub unsafe extern "C" fn $func_name (mut path: *const libc::c_char, argv: *const *const libc::c_char) -> libc::c_int {
            let envp: *const *const libc::c_char = std::ptr::null();
            let mut $program = crate::platforms::linux::c_char_to_osstring(path);
            $body
            let program_c_str = match crate::platforms::linux::osstr_to_cstring(&$program) {
                Err(_why) => {
                    *crate::platforms::linux::errno_location() = libc::ENOENT;
                    return -1 },
                Ok(res) => res
            };
            path = program_c_str.as_ptr() as *const libc::c_char;
            let hexdigest = crate::common::hash::common_hash_file(&$program);
            let env = crate::platforms::linux::parse_env_collection(envp);
            let uid = crate::platforms::linux::get_current_uid();
            // Permit/deny execution
            if !crate::common::whitelist::is_whitelisted(&$program, &env, &hexdigest) {
                crate::common::event::send_exec_event(uid, &$program, &hexdigest, false);
                *crate::platforms::linux::errno_location() = libc::EACCES;
                return -1
            }
            crate::common::event::send_exec_event(uid, &$program, &hexdigest, true);
            call_real!{ $func_name (path: *const libc::c_char, argv: *const *const libc::c_char) -> libc::c_int }
        }
    };
    (hook $func_name:ident ($program: ident, $envp:ident) custom_routine $body:block) => {
        #[no_mangle]
        #[allow(unused_assignments)]
        #[allow(unused_mut)]
        pub unsafe extern "C" fn $func_name (mut path: *const libc::c_char, argv: *const *const libc::c_char, $envp: *const *const libc::c_char) -> libc::c_int {
            let mut $program = crate::platforms::linux::c_char_to_osstring(path);
            $body
            let program_c_str = match crate::platforms::linux::osstr_to_cstring(&$program) {
                Err(_why) => {
                    *crate::platforms::linux::errno_location() = libc::ENOENT;
                    return -1 },
                Ok(res) => res
            };
            path = program_c_str.as_ptr() as *const libc::c_char;
            let hexdigest = crate::common::hash::common_hash_file(&$program);
            let env = crate::platforms::linux::parse_env_collection($envp);
            let uid = crate::platforms::linux::get_current_uid();
            // Permit/deny execution
            if !crate::common::whitelist::is_whitelisted(&$program, &env, &hexdigest) {
                crate::common::event::send_exec_event(uid, &$program, &hexdigest, false);
                *crate::platforms::linux::errno_location() = libc::EACCES;
                return -1
            }
            crate::common::event::send_exec_event(uid, &$program, &hexdigest, true);
            call_real!{ $func_name (path: *const libc::c_char, argv: *const *const libc::c_char, $envp: *const *const libc::c_char) -> libc::c_int }
        }
    };
}

// Variadic exec hook template
macro_rules! build_variadic_exec_hook {
    (hook $func_name:ident ($program: ident, $args:ident, $envp:ident) custom_routine $body:block) => {
        #[no_mangle]
        #[allow(unused_assignments)]
        #[allow(unused_mut)]
        pub unsafe extern "C" fn $func_name (mut path: *const libc::c_char, mut $args: ...) -> libc::c_int {
            // Populate argv
            let mut arg_vec: Vec<*const libc::c_char> = Vec::new();
            let mut next_argv: isize = $args.arg();
            let mut ptr_to_next_argv = next_argv as *const libc::c_char;
            while !(ptr_to_next_argv).is_null() {
                arg_vec.push(ptr_to_next_argv);
                next_argv = $args.arg();
                ptr_to_next_argv = next_argv as *const libc::c_char;
            }
            arg_vec.push(std::ptr::null());
            let argv: *const *const libc::c_char = (&arg_vec).as_ptr() as *const *const libc::c_char;
            let mut $envp: *const *const libc::c_char = crate::platforms::linux::environ();
            let mut $program = crate::platforms::linux::c_char_to_osstring(path);
            $body
            let program_c_str = match crate::platforms::linux::osstr_to_cstring(&$program) {
                Err(_why) => {
                    *crate::platforms::linux::errno_location() = libc::ENOENT;
                    return -1 },
                Ok(res) => res
            };
            path = program_c_str.as_ptr() as *const libc::c_char;
            let hexdigest = crate::common::hash::common_hash_file(&$program);
            let env = crate::platforms::linux::parse_env_collection($envp);
            let uid = crate::platforms::linux::get_current_uid();
            // Permit/deny execution
            if !crate::common::whitelist::is_whitelisted(&$program, &env, &hexdigest) {
                crate::common::event::send_exec_event(uid, &$program, &hexdigest, false);
                *crate::platforms::linux::errno_location() = libc::EACCES;
                return -1
            }
            crate::common::event::send_exec_event(uid, &$program, &hexdigest, true);
            call_real!{ execve (path: *const libc::c_char, argv: *const *const libc::c_char, $envp: *const *const libc::c_char) -> libc::c_int }
        }
    };
}
