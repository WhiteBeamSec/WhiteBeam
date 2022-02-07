build_action! { ModifyString (_par_prog, _src_prog, hook, arg_id, args, act_args, do_return, return_value) {
    assert!(act_args.len() == 1);
    let arg_index = args.iter().position(|arg| arg.id == arg_id).expect("WhiteBeam: Lost track of environment");
    let act_arg_cstring: Box<std::ffi::CString> = Box::new(std::ffi::CString::new(act_args[0].clone()).expect("WhiteBeam: Unexpected null reference"));
    args[arg_index].real = Box::leak(act_arg_cstring).as_ptr() as usize;
}}