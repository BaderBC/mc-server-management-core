use std::{fs, process};
use std::path::PathBuf;
use crate::utils::msmc_var_dir::INSTANCES_PATH;

pub fn init(name: &str) {
    let mut path = INSTANCES_PATH.to_path_buf();
    path.push(name);

    let dir_create = fs::create_dir_all(path);

    if let Err(err) = dir_create {
        eprintln!("Error: {}", err);
        eprintln!("Permission denied. Please contact your server administrator.");
        process::exit(1);
    }
}

pub fn init_and_get_container_dir(name: &str) -> PathBuf {
    init(name);

    let mut path = INSTANCES_PATH.to_path_buf();
    path.push(name);
    path
}
