#[macro_export]

macro_rules! exec_hook_template {
    (test $func_name:ident ($test_name:ident, mod_env: $mod_env:expr, mod_path: $mod_path:expr, $path:ident, $args:ident) custom_routine $body:block) => {
        fn $test_name(test_type: &str) -> i32 {
            let ($path, flags);
            // TODO: Copy bash to /tmp instead of using dash
            let (bash, dash) = match $mod_path {
                true => ("bash", "dash"),
                false => ("/bin/bash", "/bin/dash")
            };
            if test_type == "positive" {
                $path = CString::new(bash).expect("WhiteBeam: CString::new failed");
            } else if test_type == "negative" {
                $path = CString::new(dash).expect("WhiteBeam: CString::new failed");
            } else {
                eprintln!("WhiteBeam: Invalid test type. Valid tests are: positive negative");
                return -1;
            }
            flags = CString::new("-c").expect("WhiteBeam: CString::new failed");
            let command = match $mod_env {
                true => {
                    env::set_var("WB_TEST", "invalid");
                    CString::new("echo -n $WB_TEST > /tmp/test_result").expect("WhiteBeam: CString::new failed")
                },
                false => CString::new("echo -n $LD_PRELOAD > /tmp/test_result").expect("WhiteBeam: CString::new failed")
            };
            let $args: Vec<*const libc::c_char> = vec!($path.as_ptr(),
                                                 flags.as_ptr(),
                                                 command.as_ptr(),
                                                 std::ptr::null());
            $body
        }
    };
}

macro_rules! test_exec_hook {
    (test fexecve ($test_name:ident, mod_env: true, mod_path: $mod_path:expr)) => {
        exec_hook_template! { test fexecve ($test_name, mod_env: true, mod_path: $mod_path, path, args)
                              custom_routine {
                                  let pid = unsafe { libc::fork() };
                                  match pid {
                                      -1 => {return -1},
                                      0 => {
                                          let wb_test_env = CString::new(format!("WB_TEST={}/target/release/libwhitebeam.so", env!("PWD"))).expect("WhiteBeam: CString::new failed");
                                          let env_vec: Vec<*const libc::c_char> = vec!(wb_test_env.as_ptr(),
                                                                                 std::ptr::null());
                                          let fd: libc::c_int = unsafe { libc::open(path.as_ptr(), libc::O_RDONLY) };
                                          if fd < 0 {
                                              return -1
                                          }
                                          unsafe { fexecve(fd, args.as_ptr(), env_vec.as_ptr()); }
                                          return -1
                                      },
                                      _ => {
                                          let status = 0 as *mut i32;
                                          unsafe {libc::waitpid(pid, status, 0);}
                                          return status as i32
                                      }
                                  }
                              }
                          }
    };
    (test $func_name:ident ($test_name:ident, mod_env: true, mod_path: $mod_path:expr)) => {
        exec_hook_template! { test $func_name ($test_name, mod_env: true, mod_path: $mod_path, path, args)
                              custom_routine {
                                  let pid = unsafe { libc::fork() };
                                  match pid {
                                      -1 => {return -1},
                                      0 => {
                                          let wb_test_env = CString::new(format!("WB_TEST={}/target/release/libwhitebeam.so", env!("PWD"))).expect("WhiteBeam: CString::new failed");
                                          let env_vec: Vec<*const libc::c_char> = vec!(wb_test_env.as_ptr(),
                                                                                 std::ptr::null());
                                          unsafe { $func_name(path.as_ptr(), args.as_ptr(), env_vec.as_ptr()); }
                                          return -1
                                      },
                                      _ => {
                                          let status = 0 as *mut i32;
                                          unsafe {libc::waitpid(pid, status, 0);}
                                          return status as i32
                                      }
                                  }
                              }
                            }
    };
    (test $func_name:ident ($test_name:ident, mod_env: false, mod_path: $mod_path:expr)) => {
        exec_hook_template! { test $func_name ($test_name, mod_env: false, mod_path: $mod_path, path, args)
                              custom_routine {
                                  let pid = unsafe { libc::fork() };
                                  match pid {
                                      -1 => {return -1},
                                      0 => {
                                          unsafe { $func_name(path.as_ptr(), args.as_ptr()); }
                                          return -1
                                      },
                                      _ => {
                                          let status = 0 as *mut i32;
                                          unsafe {libc::waitpid(pid, status, 0);}
                                          return status as i32
                                      }
                                  }
                              }
                            }
    };
}

macro_rules! test_variadic_exec_hook {
    (test $func_name:ident ($test_name:ident, mod_env: true, mod_path: $mod_path:expr)) => {
        exec_hook_template! { test $func_name ($test_name, mod_env: true, mod_path: $mod_path, path, args)
                              custom_routine {
                                  let pid = unsafe { libc::fork() };
                                  match pid {
                                      -1 => {return -1},
                                      0 => {
                                          let wb_test_env = CString::new(format!("WB_TEST={}/target/release/libwhitebeam.so", env!("PWD"))).expect("WhiteBeam: CString::new failed");
                                          let env_vec: Vec<*const libc::c_char> = vec!(wb_test_env.as_ptr(),
                                                                                 std::ptr::null());
                                          unsafe { $func_name(path.as_ptr(),
                                                              args[0],
                                                              args[1],
                                                              args[2],
                                                              args[3],
                                                              env_vec.as_ptr()); }
                                          return -1
                                      },
                                      _ => {
                                          let status = 0 as *mut i32;
                                          unsafe {libc::waitpid(pid, status, 0);}
                                          return status as i32
                                      }
                                  }
                              }
                            }
    };
    (test $func_name:ident ($test_name:ident, mod_env: false, mod_path: $mod_path:expr)) => {
        exec_hook_template! { test $func_name ($test_name, mod_env: false, mod_path: $mod_path, path, args)
                              custom_routine {
                                  let pid = unsafe { libc::fork() };
                                  match pid {
                                      -1 => {return -1},
                                      0 => {
                                          unsafe { $func_name(path.as_ptr(),
                                                              args[0],
                                                              args[1],
                                                              args[2],
                                                              args[3]); }
                                          return -1
                                      },
                                      _ => {
                                          let status = 0 as *mut i32;
                                          unsafe {libc::waitpid(pid, status, 0);}
                                          return status as i32
                                      }
                                  }
                              }
                            }
    };
}
