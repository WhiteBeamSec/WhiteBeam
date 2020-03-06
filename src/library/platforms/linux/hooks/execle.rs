#[macro_use]
/*
       int execle(const char *path, const char *arg, ...
                       /*, (char *) NULL, char * const envp[] */);
*/
build_variadic_exec_hook! {
    hook execle (program, args, envp) {
        // execle custom routines
        // Populate envp
        let envp_arg: isize = args.arg();
        envp = envp_arg as *const *const libc::c_char;
    }
}
