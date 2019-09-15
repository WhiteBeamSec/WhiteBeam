// Load OS-specific modules
// TODO: DYLD_INSERT_LIBRARIES

use std::path::Path;

pub fn get_cache_file() -> &'static Path {
    Path::new("/Applications/WhiteBeam/data/cache.json")
}
