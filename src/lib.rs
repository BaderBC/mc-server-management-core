mod instance;
mod utils;

use crate::instance::instance_config::BuildConfig;

fn main() {
    if cfg!(not(target_os = "linux")) {
        panic!("Currently only supported system is linux");
    }
    let instance_config = BuildConfig::default("test".to_string());
    println!("{}", instance_config);

    instance::create(instance_config);
    println!("test");
}
