build_action! { ExitProgram (_par_prog, _src_prog, hook, _arg_position, args, act_args, do_return, return_value) {
        assert!(act_args.len() == 1);
        let status = match act_args[0].parse::<i32>() {
            Ok(code) => code,
            Err(_) => 0
        };
        std::process::exit(status);
}}