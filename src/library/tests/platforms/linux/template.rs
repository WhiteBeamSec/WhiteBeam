#[macro_export]

macro_rules! exec_hook_template {
    (test $func_name:ident ($test_name:ident, mod_env: $mod_env:expr, mod_path: $mod_path:expr, $path:ident, $args:ident) custom_routine $body:block) => {
        fn $test_name(test_type: &str) {
            let ($path, flags, command, bash, sh);
            if !($mod_path) {
                bash = "/bin/bash";
                sh = "/bin/sh";
            } else {
                bash = "bash";
                sh = "sh";
            }
            if test_type == "positive" {
                $path = CString::new(bash).expect("WhiteBeam: CString::new failed");
            } else if test_type == "negative" {
                $path = CString::new(sh).expect("WhiteBeam: CString::new failed");
            } else {
                eprintln!("WhiteBeam: Invalid test type. Valid tests are: positive negative");
                return;
            }
            flags = CString::new("-c").expect("WhiteBeam: CString::new failed");
            if !($mod_env) {
                command = CString::new("echo -n $LD_PRELOAD > /tmp/test_result").expect("WhiteBeam: CString::new failed");
            } else {
                env::set_var("WB_TEST", "invalid");
                command = CString::new("echo -n $WB_TEST > /tmp/test_result").expect("WhiteBeam: CString::new failed");
            }
            let $args: Vec<*const c_char> = vec!($path.as_ptr(),
                                                 flags.as_ptr(),
                                                 command.as_ptr(),
                                                 std::ptr::null());
            $body
        }
    };
}

macro_rules! test_exec_hook {
    (test $func_name:ident ($test_name:ident, mod_env: true, mod_path: $mod_path:expr)) => {
        exec_hook_template! { test $func_name ($test_name, mod_env: true, mod_path: $mod_path, path, args)
                              custom_routine {
                                  let wb_test_env = CString::new("WB_TEST=./target/release/libwhitebeam.so").expect("WhiteBeam: CString::new failed");
                                  let env_vec: Vec<*const c_char> = vec!(wb_test_env.as_ptr(),
                                                                         std::ptr::null());
                                  unsafe { $func_name(path.as_ptr(), args.as_ptr(), env_vec.as_ptr()); }
                              }
                            }
    };
    (test $func_name:ident ($test_name:ident, mod_env: false, mod_path: $mod_path:expr)) => {
        exec_hook_template! { test $func_name ($test_name, mod_env: false, mod_path: $mod_path, path, args)
                              custom_routine {
                                  unsafe { $func_name(path.as_ptr(), args.as_ptr()); }
                              }
                            }
    };
}

macro_rules! test_variadic_exec_hook {
    (test $func_name:ident ($test_name:ident, mod_env: true, mod_path: $mod_path:expr)) => {
        exec_hook_template! { test $func_name ($test_name, mod_env: true, mod_path: $mod_path, path, args)
                              custom_routine {
                                  let wb_test_env = CString::new("WB_TEST=./target/release/libwhitebeam.so").expect("WhiteBeam: CString::new failed");
                                  let env_vec: Vec<*const c_char> = vec!(wb_test_env.as_ptr(),
                                                                         std::ptr::null());
                                  unsafe { $func_name(path.as_ptr(),
                                                      args[0],
                                                      args[1],
                                                      args[2],
                                                      args[3],
                                                      env_vec.as_ptr()); }
                              }
                            }
    };
    (test $func_name:ident ($test_name:ident, mod_env: false, mod_path: $mod_path:expr)) => {
        exec_hook_template! { test $func_name ($test_name, mod_env: false, mod_path: $mod_path, path, args)
                              custom_routine {
                                  unsafe { $func_name(path.as_ptr(),
                                                      args[0],
                                                      args[1],
                                                      args[2],
                                                      args[3]); }
                              }
                            }
    };
}
