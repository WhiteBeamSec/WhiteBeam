use std::{os::unix::ffi::OsStrExt,
    os::unix::ffi::OsStringExt};

extern "C" {
    pub fn mktemp(template: *mut libc::c_char) -> *mut libc::c_char;
}

build_action! { PopulateTemplate (_par_prog, _src_prog, hook, arg_position, args, act_args, do_return, return_value) {
    // TODO: ATTEMPTS_MIN, review for unnecessary allocations
    let arg_index = arg_position.expect("WhiteBeam: Lost track of environment") as usize;
    let template_argument: crate::common::db::ArgumentRow = args[arg_index].clone();
    let template_value = template_argument.real as *const libc::c_char;
    let template_osstring = unsafe { crate::common::convert::c_char_to_osstring(template_value) };
    let template_bytes = template_osstring.as_bytes();
    if !template_bytes.ends_with(b"XXXXXX") {
        do_return = true;
        return_value = -1;
        platform::set_errno(libc::EINVAL);
        return (hook, args, do_return, return_value);
    }
    if template_bytes.len() > 255 {
        do_return = true;
        return_value = -1;
        platform::set_errno(libc::ENAMETOOLONG);
        return (hook, args, do_return, return_value);
    }
    // Overwrites template_value
    unsafe { mktemp(template_value as *mut libc::c_char) };
    if (unsafe { *template_value } == 0) {
        do_return = true;
        return_value = -1;
        platform::set_errno(libc::EEXIST);
        return (hook, args, do_return, return_value);
    }
}}