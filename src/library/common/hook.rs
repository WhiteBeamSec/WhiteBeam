use crate::common::{action,
                    db};
use std::{ffi::OsString,
          lazy::SyncLazy,
          sync::Mutex};

pub static CUR_PROG: SyncLazy<Mutex<OsString>> = SyncLazy::new(|| Mutex::new(OsString::new()));
pub static FN_STACK: SyncLazy<Mutex<Vec<(i64, usize)>>> = SyncLazy::new(|| Mutex::new(vec![]));

#[allow(unused_mut)]
pub unsafe extern "C" fn generic_hook(mut arg1: usize, mut args: ...) -> isize {
    /*
    Generic hook: a variadic function capable of interposing other functions through runtime polymorphism

    Linux:
    Notes on limitations of WhiteBeam's generic Linux hook, planned to be resolved in future versions of WhiteBeam:
    - Can receive any function call and arguments, but hardcoded to call functions with up to 6 arguments
      (supports 1,587 out of 1,589 glibc functions)
    - 6 out of 1,589 glibc functions are unsupported due to no VaList equivalent
      (argp_failure, fcntl, ioctl, makecontext, strfmon, syscall, and ulimit)
    - No known security implications while Execution and Filesystem hooks are enforcing prevention mode

    Windows:
    - Can receive any function call and arguments

    macOS:
    Untested
    */
    // TODO: Test zero argument case
    // Program
    let src_prog: String = { CUR_PROG.lock().expect("WhiteBeam: Failed to lock mutex").clone().into_string().expect("WhiteBeam: Invalid executable name") };
    // Hook
    let stack_hook: (i64, usize) = { FN_STACK.lock().expect("WhiteBeam: Failed to lock mutex").pop().expect("WhiteBeam: Lost track of environment") };
    let stack_hook_id = stack_hook.0;
    let stack_hook_addr = stack_hook.1;
    let mut hook: db::HookRow = {
        let hook_cache_lock = db::HOOK_CACHE.lock().expect("WhiteBeam: Failed to lock mutex");
        let hook_option = hook_cache_lock.iter().find(|hook| hook.id == stack_hook_id);
        hook_option.expect("WhiteBeam: Lost track of environment").clone()
    };
    let hook_orig = hook.clone();
    // Arguments
    // TODO: Create Rust structures here with generic T and enum of Datatype rather than passing pointers and leaking memory
    // Converted back into respective C datatypes when Actions are completed
    // https://doc.rust-lang.org/book/ch10-01-syntax.html
    // https://stackoverflow.com/questions/40559931/vector-store-mixed-types-of-data-in-rust
    let mut arg_vec: Vec<db::ArgumentRow> = {
        let arg_cache_lock = db::ARG_CACHE.lock().expect("WhiteBeam: Failed to lock mutex");
        arg_cache_lock.iter().filter(|arg| arg.hook == stack_hook_id).map(|arg| arg.clone()).collect()
    };
    let mut argc: usize = arg_vec.iter().filter(|arg| arg.parent.is_none()).count();
    // FIXME: Refactor block, this won't work for edge cases
    // TODO: Only do this for parent arguments?
    if argc > 0 {
        let mut current_arg_idx = 0;
        arg_vec[current_arg_idx].real = arg1 as usize;
        current_arg_idx += 1;
        for i in current_arg_idx..argc {
            if arg_vec[current_arg_idx].variadic {
                // TODO: arg_vec.splice()
                let mut loops: usize = 0;
                let mut do_break: bool = false;
                let mut next_argv: usize = args.arg();
                while !(do_break) {
                    // Excess parameters are truncated in ConsumeVariadic action
                    if next_argv == 0 {
                        do_break = true;
                    }
                    if loops == 0 {
                        arg_vec[i].real = next_argv;
                    } else {
                        let mut cloned_arg = arg_vec[i].clone();
                        cloned_arg.real = next_argv;
                        current_arg_idx += 1;
                        arg_vec.insert(current_arg_idx, cloned_arg);
                    }
                    if do_break {
                        break;
                    }
                    next_argv = args.arg();
                    loops += 1;
                }
                current_arg_idx += 1;
            } else {
                arg_vec[current_arg_idx].real = args.arg();
                current_arg_idx += 1;
            }
        }
    }
    // Rules
    let mut rules: Vec<db::RuleRow> = {
        let rule_cache_lock = db::RULE_CACHE.lock().expect("WhiteBeam: Failed to lock mutex");
        let all_arg_ids: Vec<i64> = arg_vec.iter().map(|arg| arg.id).collect();
        rule_cache_lock.iter().filter(|rule| all_arg_ids.contains(&rule.arg)).map(|rule| rule.clone()).collect()
    };
    // Actions
    for rule in rules {
        // TODO: Eliminate redundancy
        // TODO: Is clone of src_prog needed?
        let (hook_new, arg_vec_new, do_return, return_value) = action::process_action(src_prog.clone(), rule, hook, arg_vec);
        hook = hook_new;
        arg_vec = arg_vec_new;
        if do_return {
            return return_value;
        }
    };
    // Dispatch
    // FIXME: Bug in some *64 functions like open64 => openat and fopen64 => fdopen
    // TODO: Generic symbol resolution
    let fn_addr = match hook_orig.symbol.as_ref() {
        "open64" => libc::openat as *const u8,
        _ => crate::platforms::linux::dlsym_next_relative(&hook.symbol, stack_hook_addr)
    };
    let hooked_fn_zargs: unsafe extern "C" fn() -> isize = std::mem::transmute(fn_addr);
    let hooked_fn_margs: unsafe extern "C" fn(arg1: usize, args: ...) -> isize = std::mem::transmute(fn_addr);
    let par_args: Vec<&db::ArgumentRow> = arg_vec.iter().filter(|arg| arg.parent.is_none()).collect(); // Parent arguments
    argc = par_args.len();
    let ret: isize = match argc {
        0 => hooked_fn_zargs(),
        1 => hooked_fn_margs(par_args[0].real),
        2 => hooked_fn_margs(par_args[0].real, par_args[1].real),
        3 => hooked_fn_margs(par_args[0].real, par_args[1].real, par_args[2].real),
        4 => hooked_fn_margs(par_args[0].real, par_args[1].real, par_args[2].real, par_args[3].real),
        5 => hooked_fn_margs(par_args[0].real, par_args[1].real, par_args[2].real, par_args[3].real, par_args[4].real),
        6 => hooked_fn_margs(par_args[0].real, par_args[1].real, par_args[2].real, par_args[3].real, par_args[4].real, par_args[5].real),
        // Unsupported
        _ => panic!("WhiteBeam: Unsupported operation"),
    };
    // TODO: Post actions
    let (do_return, return_value) = action::process_post_action(src_prog.clone(), hook_orig, hook, arg_vec);
    if do_return {
        return return_value;
    }
    ret
}