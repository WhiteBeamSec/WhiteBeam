build_action! { VerifyCanMakeNode (par_prog, src_prog, hook, arg_position, args, _act_args, do_return, return_value) {
        // Reference for major device numbers: https://github.com/torvalds/linux/blob/master/Documentation/admin-guide/devices.txt
        if !((&hook.symbol).contains("mknod") && (&hook.library).contains("libc.so")) {
            unimplemented!("WhiteBeam: VerifyCanMakeNode action is unsupported outside of Filesystem hooks");
        }
        let mode_index = arg_position.expect("WhiteBeam: Lost track of environment") as usize;
        let mode: libc::mode_t = args[mode_index].clone().real as libc::mode_t; // u32
        let node_type = mode & libc::S_IFMT;
        // Ignore nodes that are not block or character device files
        if (node_type != libc::S_IFBLK) && (node_type != libc::S_IFCHR) {
            return (hook, args, do_return, return_value);
        }
        let dev: libc::dev_t = args[mode_index+1].clone().real as libc::dev_t; // u64
        let major: u32 = unsafe { libc::major(dev) };
        // Check whitelist for major number
        let any = String::from("ANY");
        let class: String = String::from("Filesystem/Device");
        let all_allowed_devices: Vec<String> = {
            let whitelist_cache_lock = crate::common::db::WL_CACHE.lock().expect("WhiteBeam: Failed to lock mutex");
            whitelist_cache_lock.iter().filter(|whitelist| (whitelist.class == class) && ((whitelist.parent == par_prog) || (whitelist.parent == any)) && ((whitelist.path == src_prog) || (whitelist.path == any))).map(|whitelist| whitelist.value.clone()).collect()
        };
        // Permit ANY
        if all_allowed_devices.iter().any(|device| device == &any) {
            return (hook, args, do_return, return_value);
        }
        let major_string: String = major.to_string();
        // Permit whitelisted devices
        if all_allowed_devices.iter().any(|device| device == &major_string) {
            return (hook, args, do_return, return_value);
        }
        // Permit device if not running in prevention mode
        if !(crate::common::db::get_prevention()) {
            event::send_log_event(libc::LOG_NOTICE, format!("Detection: {} -> {} created device file with major number {} (VerifyCanMakeNode)", &par_prog, &src_prog, &major_string));
            return (hook, args, do_return, return_value);
        }
        // Permit authorized creation of device files
        if crate::common::db::get_valid_auth_env() {
            return (hook, args, do_return, return_value);
        }
        // Deny by default
        event::send_log_event(libc::LOG_WARNING, format!("Prevention: Blocked {} -> {} from creating device file with major number {} (VerifyCanMakeNode)", &par_prog, &src_prog, &major_string));
        do_return = true;
        return_value = -1;
        platform::set_errno(libc::EPERM);
}}
