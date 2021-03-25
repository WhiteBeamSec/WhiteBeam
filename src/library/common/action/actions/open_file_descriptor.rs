#[macro_use]
build_action! { OpenFileDescriptor (_src_prog, hook, arg_id, args, do_return, return_value) {
        let file_index = args.iter().position(|arg| arg.id == arg_id).expect("WhiteBeam: Lost track of environment");
        let file_argument: crate::common::db::ArgumentRow = args[file_index].clone();
        let file_value = file_argument.real as *const libc::c_char;
        // TODO: fd modes vary per-function. For now, just O_PATH for exec*.
        let fd: libc::c_int = unsafe { libc::open(file_value, libc::O_PATH) };
        if fd >= 0 {
            args[file_index].datatype = String::from("IntegerSigned");
            args[file_index].real = fd as usize;
            return (hook, args, do_return, return_value);
        }
        do_return = true;
        return_value = -1;
}}
