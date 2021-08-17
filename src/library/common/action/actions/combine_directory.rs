pub fn normalize_path(path: &std::path::Path) -> std::path::PathBuf {
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ std::path::Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        std::path::PathBuf::from(c.as_os_str())
    } else {
        std::path::PathBuf::new()
    };

    for component in components {
        match component {
            std::path::Component::Prefix(..) => unreachable!(),
            std::path::Component::RootDir => {
                ret.push(component.as_os_str());
            }
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                ret.pop();
            }
            std::path::Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}

build_action! { CombineDirectory (_src_prog, hook, arg_id, args, do_return, return_value) {
        let dirfd_index = args.iter().position(|arg| arg.id == arg_id).expect("WhiteBeam: Lost track of environment");
        let dirfd_argument: crate::common::db::ArgumentRow = args[dirfd_index].clone();
        let path_argument: crate::common::db::ArgumentRow = args[dirfd_index+1].clone();
        let dirfd_value = dirfd_argument.real as libc::c_int;
        let path_value = path_argument.real as *const libc::c_char;
        // TODO: Error handling
        let path_string = unsafe { String::from(std::ffi::CStr::from_ptr(path_value).to_str().expect("WhiteBeam: Unexpected null reference")) };
        if !(path_string.contains("/") || path_string.contains("..")) {
            return (hook, args, do_return, return_value);
        }
        let mut path_new: std::path::PathBuf = match dirfd_value {
            libc::AT_FDCWD => std::env::current_dir().expect("WhiteBeam: Lost track of environment"),
            _ => platform::canonicalize_fd(dirfd_value as i32).expect("WhiteBeam: Lost track of environment")
        };
        path_new.push(std::path::PathBuf::from(path_string));
        let path_new_normal: std::path::PathBuf = normalize_path(&path_new);
        // TODO: Error handling
        let filename_new: &std::ffi::OsStr = (&path_new_normal).file_name().unwrap_or(&std::ffi::OsStr::new("."));
        let filename_new_cstring: Box<std::ffi::CString> = Box::new(crate::common::convert::osstr_to_cstring(filename_new).expect("WhiteBeam: Unexpected null reference"));
        let path_new_parent: std::path::PathBuf = match (&path_new_normal).parent() {
            Some(f) => f.to_owned(),
            None => std::path::PathBuf::from("/")
        };
        let dirfd_new_cstring: std::ffi::CString = crate::common::convert::osstr_to_cstring((&path_new_parent).as_os_str()).expect("WhiteBeam: Unexpected null reference");
        // TODO: Don't we need a post action to close() this fd when the dirfd orig != dirfd new?
        let fd: libc::c_int = unsafe { libc::open(dirfd_new_cstring.as_ptr(), libc::O_PATH) };
        if fd >= 0 {
            args[dirfd_index].real = fd as usize;
            args[dirfd_index+1].real = Box::leak(filename_new_cstring).as_ptr() as usize;
            return (hook, args, do_return, return_value);
        }
        do_return = true;
        return_value = -1;
}}
