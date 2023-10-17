use std::fmt::{Display, Formatter};
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::instance::instance_config::Engine;
use crate::utils::docker::{ContainerBuilder, Container};
use crate::utils::msmc_var_dir;

const IMAGE: &str = "itzg/minecraft-server";
const MC_DEFAULT_PORT: u16 = 25565;
const MC_DATA_CONTAINER_DIR: &str = "/data";

pub struct InstanceBuilder {
    pub port: u16,
    pub name: String,
    pub engine: Engine,
    pub game_version: String,
    pub modpack_zip_url: Option<String>,
    pub seed: Option<String>,
}

impl InstanceBuilder {
    pub fn new(name: String) -> Self {
        Self {
            port: 25565,
            name,
            engine: Engine::VANILLA,
            game_version: "1.12".to_string(),
            modpack_zip_url: None,
            seed: None,
        }
    }

    pub fn create(self) -> anyhow::Result<Instance> {
        let seed = format!("SEED={}", self.seed.unwrap_or(String::new()));
        let modpack_url = format!("MODPACK={}", self.modpack_zip_url.unwrap_or(String::new()));

        let mut host_path = msmc_var_dir::init_and_get_instance_dir();
        host_path.push(&self.name);

        ContainerBuilder::new(IMAGE)
            .env("EULA=TRUE")
            .env(&format!("VERSION={}", self.game_version))
            .env(&seed)
            .env(&modpack_url)
            .name(&self.name)
            .port_mapping(self.port, MC_DEFAULT_PORT)
            .mount(host_path, Path::new(MC_DATA_CONTAINER_DIR))
            .create();

        Instance::get(&self.name)
    }
}

pub struct Instance {
    container: Container,
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct InstanceInfo {
    name: String,
    // TODO: more fields
}

impl Instance {
    pub fn get(name: &str) -> anyhow::Result<Self> {
        Ok(Self {
            container: Container::get(name)?,
            name: name.to_string(),
        })
    }

    pub fn get_info(self) -> InstanceInfo {
        // TODO: return way more information about instance
        InstanceInfo {
            name: self.name
        }
    }

    pub fn delete(self) {
        self.container.remove();
        // TODO: delete data_dir after deleting container
    }

    pub fn run(self) {
        self.container.start();
    }

    pub fn stop(self) {
        self.container.stop();
    }
}
