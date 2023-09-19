mod instance;
mod lib;
mod utils;

use std::path::{PathBuf};
use crate::instance::InstanceConfig;


fn main() {
    let instance_config = InstanceConfig::default("test".to_string());
    println!("{}", instance_config);

    instance::create(instance_config);
    println!("test");
}
