use std::collections::HashMap;
use rs_docker::container::{ContainerCreate};

use crate::DOCKER_CONNECTION;
use crate::{InstanceConfig};

pub fn create(config: InstanceConfig) {
    let mut connection = DOCKER_CONNECTION.lock().unwrap();
    connection.create_container(
        String::from("mc-server-manager_") + &config.name,
        ContainerCreate {
            Image: "itzg/minecraft-server".to_string(),
            Labels: None,
            ExposedPorts: Some(port_into_exposed_ports(config.port)),
            HostConfig: None,
        },
    ).expect("Failed to create new mc instance");
}

fn port_into_exposed_ports(port: u16) -> HashMap<String, HashMap<i32, i32>> {
    let mut port_map: HashMap<i32, i32> = HashMap::new();
    port_map.insert(port as i32, 25565);
    // I do not understand why docker-rs requires String here
    HashMap::from([(String::new(), port_map)])
}

