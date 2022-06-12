build_action! { PrintArguments (_par_prog, _src_prog, hook, _arg_id, args, _act_args, do_return, return_value) {
        // strace/l(a)trace-like functionality
        // TODO: Refactor to use Display
        let library: &str = &hook.library;
        let symbol: &str = &hook.symbol;
        let parent_arguments = args.iter().filter(|arg| arg.parent == 0);
        let mut print_string = format!("WhiteBeam: PrintArguments ({}) {}(", library, symbol);
        let mut idx = 0;
        for arg in parent_arguments {
            if idx > 0 {
                print_string.push_str(", ");
            }
            idx += 1;
            let print_val = match arg.datatype.as_ref() {
                "String" => unsafe { format!("\"{}\"", std::ffi::CStr::from_ptr(arg.real as *const libc::c_char).to_str().expect("WhiteBeam: Unexpected null reference")) },
                "StringArray" => unsafe { format!("{:?}", crate::common::convert::parse_arg_collection_lossy(arg.real as *const *const libc::c_char)) },
                "IntegerSigned" => format!("{}", arg.real as i32),
                "IntegerUnsigned" => format!("{}", arg.real as u32),
                "IntegerUnsignedPointer" => format!("{:p}", arg.real as *const usize),
                "LongSigned" => format!("{}", arg.real as i64),
                "LongUnsigned" => format!("{}", arg.real as u64),
                // TODO: Struct: build recursively
                _ => format!("(unknown)")
            };
            print_string.push_str(&print_val);
        }
        print_string.push_str(")");
        println!("{}", print_string);
}}