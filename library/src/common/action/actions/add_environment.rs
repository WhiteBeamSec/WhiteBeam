build_action! { AddEnvironment (_par_prog, _src_prog, hook, _arg_id, args, _act_args, do_return, return_value) {
        if !((&hook.symbol).contains("exec") && (&hook.library).contains("libc.so")) {
            unimplemented!("WhiteBeam: AddEnvironment action is unsupported outside of Execution hooks");
        }
        let new_arg = crate::common::db::ArgumentRow {
            hook: hook.id,
            parent: 0,
            id: -1,
            position: args.len() as i64,
            real: unsafe { platform::environ() } as usize,
            datatype: String::from("StringArray"),
            pointer: true,
            signed: false,
            variadic: false,
            array: true
        };
        args.push(new_arg);
}}
