build_action! { ModifyInt (_par_prog, _src_prog, hook, arg_position, args, act_args, do_return, return_value) {
    assert!(act_args.len() == 1);
    let arg_index = arg_position.expect("WhiteBeam: Lost track of environment") as usize;
    let act_arg_int = act_args[0].parse::<i64>().expect("WhiteBeam: Unexpected null reference");
    args[arg_index].real = act_arg_int as usize;
}}