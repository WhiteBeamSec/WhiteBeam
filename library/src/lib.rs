// TODO: Eliminate dependency on nightly
// Once cell can be removed once OsString::new() is const (see library/src/common/hook.rs)
#![feature(once_cell)]
// Variadic functions: https://github.com/rust-lang/rust/issues/44930
#![feature(c_variadic)]
//#![feature(asm)]
pub mod platforms;
// Platform independent features
pub mod common;
