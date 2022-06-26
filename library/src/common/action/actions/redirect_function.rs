build_action! { RedirectFunction (_par_prog, _src_prog, hook, _arg_position, args, act_args, do_return, return_value) {
        assert!(act_args.len() == 2);
        hook.library = act_args[0].clone();
        hook.symbol = act_args[1].clone();
}}