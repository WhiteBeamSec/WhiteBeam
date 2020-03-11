#[macro_use]
/*
       int execlp(const char *path, const char *arg, ...
                       /* (char  *) NULL */);
*/
build_variadic_exec_hook! {
    hook execlp (program, args, envp)
    custom_routine {
        // Repopulate program
        let absolute_path = match crate::platforms::linux::search_path(&program) {
            Some(abspath) => abspath,
            None => {
                *crate::platforms::linux::errno_location() = libc::ENOENT;
                return -1 }
        };
        program = absolute_path.as_os_str().to_owned();
    }
}
