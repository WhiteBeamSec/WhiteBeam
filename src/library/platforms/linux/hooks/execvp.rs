#[macro_use]
/*
       int execvp(const char *file, char *const argv[]);
*/
build_exec_hook! {
    hook execvp (program) {
        // execvp custom routines
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
