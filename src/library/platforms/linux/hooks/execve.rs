#[macro_use]
/*
       int execve(const char *path, char *const argv[],
                  char *const envp[]);
*/
build_exec_hook! {
    hook execve (program, envp) {
        // execve custom routines
        // Warn that legacy versions of man-db must disable seccomp
        // TODO: Hook proper function
        if program == "/usr/bin/man" {
            let needle = std::ffi::OsString::from("MAN_DISABLE_SECCOMP");
            let mut disable_defined = false;
            let man_env = crate::platforms::linux::parse_env_collection(envp);
            for env_var in man_env {
                if env_var.0 == needle {
                    disable_defined = true;
                    break;
                }
            }
            if !(disable_defined) {
                eprintln!("WhiteBeam: Legacy man-db versions require MAN_DISABLE_SECCOMP=1")
            }
        }
    }
}
