#[macro_use]
build_action! { AddFlags (_src_prog, hook, _arg_id, args, do_return, return_value) {
        let library: &str = &hook.library;
        let symbol: &str = &hook.symbol;
}}
