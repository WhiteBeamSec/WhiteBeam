build_action! { ModifyInt (_src_prog, hook, _arg_id, args, act_args, do_return, return_value) {
    assert!(act_args.len() == 2);
    let arg_index = act_args[0].parse::<usize>().expect("WhiteBeam: Unexpected null reference");
    let act_arg_int = act_args[1].parse::<i64>().expect("WhiteBeam: Unexpected null reference");
    args[arg_index].real = act_arg_int as usize;
}}