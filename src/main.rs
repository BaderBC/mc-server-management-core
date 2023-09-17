mod docker_connection;
mod instance;

pub use docker_connection::DOCKER_CONNECTION;
use crate::instance::InstanceConfig;


fn main() {
    let instance_config = InstanceConfig::default("test".to_string());
    println!("{}", instance_config);
    
    //instance::create(instance_config);
}
