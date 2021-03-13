#[macro_use]
build_action! { OpenFileDescriptor (_src_prog, hook, arg_id, args, do_return, return_value) {
        let library: &str = &hook.library;
        let symbol: &str = &hook.symbol;
}}
