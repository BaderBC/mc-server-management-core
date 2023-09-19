use std::collections::HashMap;
use std::path::{Path};
use std::process::Command;
use crate::lib::path_to_string;

pub struct Container {
    name: Option<String>,
    image: String,
    exposed_ports: HashMap<u16, u16>,
    mount_points: HashMap<String, String>,
}

impl Container {
    pub fn new<T: ToString>(image: T) -> Container {
        Container {
            name: None,
            image: image.to_string(),
            exposed_ports: Default::default(),
            mount_points: Default::default(),
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn port_mapping(mut self, host_port: u16, container_port: u16) -> Self {
        self.exposed_ports.insert(host_port, container_port);
        self
    }

    pub fn mount<T1, T2>(mut self, host_path: T1, container_path: T2) -> Self
        where T1: AsRef<Path>, T2: AsRef<Path> {
        let host_path = path_to_string(host_path);
        let container_path = path_to_string(container_path);

        self.mount_points.insert(host_path, container_path);
        self
    }

    pub fn create(self) {
        let mut command = Command::new("docker");
        command.args(["container", "create"]);

        if let Some(name) = self.name {
            command.args(["--name", &name]);
        }
        for (host_port, container_port) in self.exposed_ports.into_iter() {
            command.args(["-p", &format!("{}:{}", host_port, container_port)]);
        }
        for (host_path, container_path) in self.mount_points.into_iter() {
            command.args(["-v", &format!("{}:{}", host_path, container_path)]);
        }
        command.arg(self.image);

        const FAILED_TO_CREATE: &str = "Failed to create docker container";

        let exit_status = command
            .status()
            .expect(FAILED_TO_CREATE);

        assert!(exit_status.success(), "{}", FAILED_TO_CREATE);
    }
}
