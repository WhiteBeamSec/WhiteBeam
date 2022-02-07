build_action! { VerifyCanTerminate (_par_prog, src_prog, hook, arg_id, args, _act_args, do_return, return_value) {
        #[cfg(feature = "whitelist_test")]
        return (hook, args, do_return, return_value);
        let pid_index = args.iter().position(|arg| arg.id == arg_id).expect("WhiteBeam: Lost track of environment");
        let pid: i32 = args[pid_index].clone().real as i32;
        let service_pid_string: String = std::fs::read_to_string(platform::get_data_file_path_string("whitebeam.pid")).expect("WhiteBeam: Lost track of environment");
        let service_pid: i32 = service_pid_string.strip_suffix("\n").unwrap_or(&service_pid_string).parse().expect("WhiteBeam: Unexpected null reference");
        let service_pgid: i32 = unsafe { libc::getpgid(service_pid) };
        if (pid == service_pid) ||
           (pid == -service_pgid) ||
           ((pid == -1) && (platform::get_current_uid() == 0)) {
            // Permit termination if not running in prevention mode
            if !(crate::common::db::get_prevention()) {
                event::send_log_event(syslog::Severity::LOG_NOTICE as i64, format!("Detection: {} killed the WhiteBeam service (VerifyCanTerminate)", &src_prog));
                return (hook, args, do_return, return_value);
            }
            // Permit authorized termination
            if crate::common::db::get_valid_auth_env() {
                return (hook, args, do_return, return_value);
            }
            event::send_log_event(syslog::Severity::LOG_WARNING as i64, format!("Prevention: Blocked {} from killing WhiteBeam service (VerifyCanTerminate)", &src_prog));
            eprintln!("WhiteBeam: kill ({}): Operation not permitted", pid);
            do_return = true;
            return_value = -1;
        }
}}
