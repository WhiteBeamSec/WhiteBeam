use libc::c_char;
use std::{ffi::CStr,
          ffi::CString,
          ffi::NulError,
          ffi::OsStr,
          ffi::OsString,
          os::unix::ffi::OsStrExt,
          os::unix::ffi::OsStringExt};

// TODO: impl/trait? Extend types? 0.2

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
