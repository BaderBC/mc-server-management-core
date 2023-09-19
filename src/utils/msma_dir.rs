use std::{fs, process};
use std::path::{PathBuf};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref INSTANCE_PATH: PathBuf = {
        let mut instances_path = PathBuf::new();
        instances_path.push(env!("MSMA_PATH"));
        instances_path.push("minecraft_instances");

        instances_path
    };
}

pub fn check_and_fix() {
    let dir_create = fs::create_dir_all(INSTANCE_PATH.to_path_buf());

    if let Err(err) = dir_create {
        eprintln!("Error: {}", err);
        eprintln!("Permission denied. Please contact your server administrator.");
        process::exit(1);
    }
}

pub fn get_and_fix_instance_folder() -> PathBuf {
    check_and_fix();
    INSTANCE_PATH.to_path_buf()
}
