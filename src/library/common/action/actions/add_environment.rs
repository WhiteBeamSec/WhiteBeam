#[macro_use]
build_action! { AddEnvironment (_src_prog, hook, _arg_id, args, do_return, return_value) {
        if !((&hook.symbol).contains("exec") && (&hook.library).contains("libc.so")) {
            unimplemented!("WhiteBeam: AddEnvironment action is unsupported outside of Execution hooks");
        }
        #[cfg(target_os = "linux")]
        let new_arg = crate::common::db::ArgumentRow {
            hook: hook.id,
            parent: None,
            id: -1,
            position: args.len() as i64,
            real: unsafe { crate::platforms::linux::environ() } as usize,
            datatype: String::from("StringArray"),
            pointer: true,
            signed: false,
            variadic: false,
            array: true
        };
        #[cfg(not(target_os = "linux"))]
        unimplemented!("WhiteBeam: AddEnvironment action on non-Linux platforms is not currently supported");
        args.push(new_arg);
}}
