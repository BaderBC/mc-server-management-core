use std::fmt::{Display, Formatter};
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::instance::instance_config::Engine;
use crate::utils::docker::{ContainerBuilder, Container};
use crate::utils::msmc_var_dir;
use crate::utils::msmc_var_dir::init_and_get_instances_dir;

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

        let mut instances_path = init_and_get_instances_dir();
        instances_path.push(&self.name);

        ContainerBuilder::new(IMAGE)
            .env("EULA=TRUE")
            .env(&format!("VERSION={}", self.game_version))
            .env(&seed)
            .env(&modpack_url)
            .name(&self.name)
            .port_mapping(self.port, MC_DEFAULT_PORT)
            .mount(&instances_path, Path::new(MC_DATA_CONTAINER_DIR))
            .create()?;

        fs::create_dir_all(instances_path)?;

        Instance::get(&self.name)
    }
}

pub struct Instance {
    container: Container,
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct InstanceDetails {
    name: String,
    // TODO: more fields
}

impl Instance {
    pub fn get(name: &str) -> anyhow::Result<Self> {
        Instances::get()?.get_instance(name)
    }

    pub fn get_details(self) -> InstanceDetails {
        // TODO: return way more information about instance
        InstanceDetails {
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

pub struct Instances {
    names: Vec<String>,
}

impl Instances {
    pub fn get() -> anyhow::Result<Self> {
        let mut names = vec![];

        let instances_path = init_and_get_instances_dir();
        let instances = instances_path.read_dir()?;

        for instance in instances {
            if let Ok(instance) = instance {
                let name = instance.file_name().into_string();

                if let Ok(name) = name {
                    names.push(name);
                }
            }
        }

        Ok(Self { names })
    }

    pub fn get_instance(&self, name: &str) -> anyhow::Result<Instance> {
        if !self.names.contains(&name.to_string()) {
            return Err(
                anyhow::Error::msg("Instance does not exist")
            );
        }

        Ok(Instance {
            container: Container::get(name)?,
            name: name.to_string(),
        })
    }

    pub fn get_all_instances(self) -> anyhow::Result<Vec<InstanceDetails>> {
        let mut all_instances = vec![];

        // TODO: it look really inefficient - optimize it
        for name in self.names.iter() {
            all_instances.push(
                self.get_instance(name)?
                    .get_details()
            )
        }

        Ok(all_instances)
    }
}
