// TODO: Eliminate dependency on nightly
// Once cell: https://github.com/rust-lang/rust/issues/74465
// Once cell can be easily removed, we're just keeping it here in case it gets stabilized
// before variadic functions: https://stackoverflow.com/a/27826181
#![feature(once_cell)]
// Variadic functions: https://github.com/rust-lang/rust/issues/44930
#![feature(c_variadic)]
//#![feature(asm)]
pub mod platforms;
// Platform independent features
pub mod common;
