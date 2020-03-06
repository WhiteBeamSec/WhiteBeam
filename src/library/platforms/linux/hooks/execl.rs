#[macro_use]
/*
       int execl(const char *path, const char *arg, ...
                       /* (char  *) NULL */);
*/
build_variadic_exec_hook! {
    hook execl (program, args, envp) {
        // execl custom routines
    }
}
