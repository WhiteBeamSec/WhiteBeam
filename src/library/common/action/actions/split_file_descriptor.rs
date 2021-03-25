#[macro_use]
build_action! { SplitFileDescriptor (_src_prog, hook, _arg_id, args, do_return, return_value) {
        // Open dirfd and rewrite user path to be basename only
        let library: &str = &hook.library;
        let symbol: &str = &hook.symbol;
}}
