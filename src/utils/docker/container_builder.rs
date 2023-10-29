use std::collections::HashMap;
use std::env;
use std::fmt::Debug;
use std::path::{Path};
use std::process::Command;
use crate::utils::docker::container::Container;

pub struct ContainerBuilder {
    name: Option<String>,
    image: String,
    exposed_ports: HashMap<u16, u16>,
    mount_points: HashMap<String, String>,
    custom_options: Vec<(String, String)>,
}

impl ContainerBuilder {
    pub fn new<T: ToString>(image: T) -> Self {
        ContainerBuilder {
            name: None,
            image: image.to_string(),
            exposed_ports: Default::default(),
            mount_points: Default::default(),
            custom_options: Default::default(),
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
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

    pub fn env(self, value: &str) -> Self {
        self.custom_option("-e", value)
    }

    pub fn custom_option(mut self, option: &str, value: &str) -> Self {
        self.custom_options.push((option.to_string(), value.to_string()));
        self
    }

    pub fn create(self) -> anyhow::Result<Container> {
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
        for (option, value) in self.custom_options.into_iter() {
            command.args([option, value]);
        }

        command.arg(self.image);
        
        let command_output = command
            .output()?;

        let cmd_output_utf8 = command_output.stdout;
        let container_id = std::str::from_utf8(&cmd_output_utf8)
            .map_err(|_| anyhow::Error::msg("Failed to read container id"))?;

        Container::get(container_id.trim())
    }
}

fn path_to_string<T>(path: T) -> String
    where T: AsRef<Path> {
    let path = path.as_ref();

    if path.is_absolute() {
        path.to_path_buf().to_str().unwrap().to_string()
    } else {
        env::current_dir().unwrap().join(path).to_str().unwrap().to_string()
    }
}
