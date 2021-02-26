use std::io::Read;

pub struct HashObject {
    pub alias: &'static str,
    pub function: fn(&mut dyn Read) -> String
}

// Hash template
macro_rules! build_hash {
    ($alias:ident ($reader:ident) $body:block) => {
        use std::io::Read;
        #[allow(non_snake_case)]
        #[allow(unused_assignments)]
        #[allow(unused_mut)]
        pub fn $alias ($reader: &mut dyn Read) -> String {
            $body
        }
        #[linkme::distributed_slice(crate::common::hash::HASH_INDEX)]
        pub static HASH: crate::common::hash::HashObject = crate::common::hash::HashObject { alias: stringify!($alias), function: $alias };
    };
}

// Load hash modules
// TODO: Make sure this doesn't conflict with crate namespace
mod hashes {
    automod::dir!(pub "src/library/common/hash/hashes");
}

// Collect hashes in distributed slice
#[linkme::distributed_slice]
pub static HASH_INDEX: [HashObject] = [..];

pub fn process_hash(reader: &mut dyn Read, algorithm: &str) -> String {
    // TODO: Consider removing reference here
    match HASH_INDEX.iter().find(|a| format!("Hash/{}", a.alias.replace("_", "-")) == algorithm) {
        Some(hash) => {(hash.function)(reader)}
        None => panic!("WhiteBeam: Invalid hash algorithm: {}", algorithm)
    }
}
