#[macro_use]
/*
       int execv(const char *path, char *const argv[]);
*/
build_exec_hook! {
    hook execv (program)
    custom_routine {}
}
