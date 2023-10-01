use std::{fs, process};
use std::path::{PathBuf};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref INSTANCES_PATH: PathBuf = {
        let mut instances_path = PathBuf::new();
        instances_path.push("/var/lib/msmc");
        instances_path.push("minecraft_instances");

        instances_path
    };
}

pub fn init() {
    let dir_create = fs::create_dir_all(INSTANCES_PATH.to_path_buf());

    if let Err(err) = dir_create {
        eprintln!("Error: {}", err);
        eprintln!("Permission denied. Please contact your server administrator.");
        process::exit(1);
    }
}

pub fn init_and_get_instance_dir() -> PathBuf {
    init();
    INSTANCES_PATH.to_path_buf()
}
