use libc::c_char;
use std::{ffi::CStr,
          ffi::CString,
          ffi::NulError,
          ffi::OsStr,
          ffi::OsString,
          os::unix::ffi::OsStrExt,
          os::unix::ffi::OsStringExt};

// TODO: impl/trait? Extend types? .into()?

pub unsafe fn c_char_to_osstring(char_ptr: *const c_char) -> OsString {
    match char_ptr.is_null() {
        true => OsString::new(),
        false => {
            let program_c_str: &CStr = CStr::from_ptr(char_ptr);
            OsStr::from_bytes(program_c_str.to_bytes()).to_owned()
        }
    }
}

pub fn osstr_to_cstring(osstr_input: &OsStr) -> Result<CString, NulError> {
    CString::new(osstr_input.as_bytes())
}

pub fn osstr_split_at_byte(osstr_input: &OsStr, byte: u8) -> (&OsStr, &OsStr) {
    for (i, b) in osstr_input.as_bytes().iter().enumerate() {
        if b == &byte {
            return (OsStr::from_bytes(&osstr_input.as_bytes()[..i]),
                OsStr::from_bytes(&osstr_input.as_bytes()[i + 1..]));
        }
    }
    (&*osstr_input, OsStr::from_bytes(&osstr_input.as_bytes()[osstr_input.len()..osstr_input.len()]))
}

pub fn parse_env_single(input: &[u8]) -> Option<(OsString, OsString)> {
    // TODO: Windows support
    // TODO: Test for environment without =
    if input.is_empty() {
        return None;
    }
    let pos = input[1..].iter().position(|&x| x == b'=').map(|p| p + 1);
    pos.map(|p| {
        (
            OsStringExt::from_vec(input[..p].to_vec()),
            OsStringExt::from_vec(input[p + 1..].to_vec()),
        )
    })
}

pub unsafe fn parse_arg_collection_lossy(args_c: *const *const c_char) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    if !(args_c.is_null()) {
        let mut args_c_iter = args_c;
        while !(*args_c_iter).is_null() {
                if let lossy_string = String::from(CStr::from_ptr(*args_c_iter).to_string_lossy()) {
                    args.push(lossy_string);
                }
                args_c_iter = args_c_iter.add(1);
        }
    }
    args
}

pub unsafe fn parse_env_collection(envp: *const *const c_char) -> Vec<(OsString, OsString)> {
    let mut env: Vec<(OsString, OsString)> = Vec::new();
    if !(envp.is_null()) {
        let mut envp_iter = envp;
        while !(*envp_iter).is_null() {
                if let Some(key_value) = parse_env_single(CStr::from_ptr(*envp_iter).to_bytes()) {
                    env.push(key_value);
                }
                envp_iter = envp_iter.add(1);
        }
    }
    env
}

pub fn u8_slice_as_os_str(s: &[u8]) -> &OsStr {
    unsafe { &*(s as *const [u8] as *const OsStr) }
}

#[cfg(not(target_os = "windows"))]
pub fn u8_slice_as_os_string(s: &[u8]) -> OsString {
    OsString::from_vec(s.to_vec())
}

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
