use std::path::{Path,
                PathBuf};

pub fn start_service() {
    unimplemented!("WhiteBeam: Service control is not currently supported on MacOS");
}

pub fn stop_service() {
    unimplemented!("WhiteBeam: Service control is not currently supported on MacOS");
}

pub fn get_data_file_path(data_file: &str) -> PathBuf {
    let data_path: String = String::from("/Applications/WhiteBeam/data/");
    let data_file_path = data_path + data_file;
    Path::new(&data_file_path).to_owned()
}

pub fn path_open_secure(file_path: &Path) {
    unimplemented!("WhiteBeam: Secure file opening is not currently supported on MacOS");
}
