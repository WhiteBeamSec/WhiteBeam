#[macro_use]
/*
       int execvpe(const char *path, char *const argv[],
                       char *const envp[]);
*/
build_exec_hook! {
    hook execvpe (program, envp) {
        // execvpe custom routines
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
