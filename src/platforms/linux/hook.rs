use ::libc::{c_char, c_void};

#[link(name = "dl")]
extern "C" {
    #[allow(dead_code)]
    pub fn dlsym(handle: *const c_void, symbol: *const c_char) -> *const c_void;
}

// GNU/Linux uses LD_PRELOAD to hook into arbitrary C functions
#[macro_use]
macro_rules! hook {
    (pub extern "C" unsafe fn $realname:ident($($paramname:ident : $paramtype:ty),*) -> $ret:ty => $customname:ident $block:block) => {
        #[allow(non_camel_case_types, dead_code)]
        pub struct $realname {
            _privatefield: (),
        }

        #[allow(non_camel_case_types, dead_code, non_upper_case_globals)]
        static $realname: $realname = $realname { _privatefield: () };

        impl $realname {
            fn get_inner(&self) -> unsafe extern "C" fn ($($paramname: $paramtype),*) -> $ret {
                use ::std::sync::{Once, ONCE_INIT};

                static mut REAL_PTR: *const u8 = 0 as *const u8;
                static mut ONCE: Once = ONCE_INIT;
                unsafe {
                    ONCE.call_once(|| {
                        let sym = platforms::linux::hook::dlsym(-1isize as *const c_void, concat!(stringify!($ealname), "\0").as_ptr() as *const c_char);
                        if sym.is_null() {
                            panic!("dlsym (ld_preload): Cannot find {}", stringify!($realname));
                        }
                        REAL_PTR = sym as *const u8;
                    });
                    ::std::mem::transmute(REAL_PTR)
                }
            }

            #[no_mangle]
            pub unsafe extern "C" fn $realname($($paramname: $paramtype),*) -> $ret {
                ::std::panic::catch_unwind(|| $customname($($paramname),*)).ok()
                    .unwrap_or_else(|| $realname.get_inner()($($paramname),*))
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn $customname($($paramname: $paramtype),*) -> $ret {
            $block
        }
    };
}
