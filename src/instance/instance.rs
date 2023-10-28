use std::fmt::{Display, Formatter};
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::instance::instance_config::Engine;
use crate::utils::docker::{ContainerBuilder, Container};
use crate::utils::MsmcDirs;

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

        let mut instance_path = MsmcDirs::get_instance(&self.name);
        instance_path.ensure_exists();

        ContainerBuilder::new(IMAGE)
            .env("EULA=TRUE")
            .env(&format!("VERSION={}", self.game_version))
            .env(&seed)
            .env(&modpack_url)
            .name(&self.name)
            .port_mapping(self.port, MC_DEFAULT_PORT)
            .mount(instance_path.path, Path::new(MC_DATA_CONTAINER_DIR))
            .create()?;

        Instance::get(&self.name)
    }
}

#[derive(Clone)]
pub struct Instance {
    container: Container,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct InstanceDetails {
    pub name: String,
    pub is_online: bool,
    // TODO: more fields
}

impl Instance {
    pub fn get(name: &str) -> anyhow::Result<Self> {
        Instances::get()?.get_instance(name)
    }

    pub fn get_details(&self) -> anyhow::Result<InstanceDetails> {
        Ok(InstanceDetails {
            name: self.name.to_string(),
            is_online: self.container.is_running()?,
        })
    }

    pub fn delete(&self) -> anyhow::Result<()> {
        self.container.remove()?;
        let mut instance_dir = MsmcDirs::get_instance(&self.name);
        instance_dir.delete();
        
        Ok(())
    }

    pub fn start(&self) -> anyhow::Result<()> {
        self.container.start()?;
        Ok(())
    }

    pub fn stop(&self) -> anyhow::Result<()> {
        self.container.stop()?;
        Ok(())
    }
}

pub struct Instances {
    pub names: Vec<String>,
}

impl Instances {
    pub fn get() -> anyhow::Result<Self> {
        let mut names = vec![];

        let instances_dir = MsmcDirs::get_instances();
        let instances = instances_dir.path.read_dir()?;

        for instance in instances {
            if let Ok(instance) = instance {
                let name = instance.file_name().into_string();

                if let Ok(name) = name {
                    names.push(name);
                }
            }
        }
        let mut self_ = Self { names };
        self_.optimize_msmc_dir()?;

        Ok(self_)
    }

    fn optimize_msmc_dir(&mut self) -> anyhow::Result<()> {
        let mut optimized_names = vec![];
        for name in &self.names {
            if let Ok(_) = Container::get(name) {
                optimized_names.push(name.to_owned());
            } else {
                let mut instance_dir = MsmcDirs::get_instance(name);
                instance_dir.delete();
            };
        }

        self.names = optimized_names;
        Ok(())
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

    pub fn get_instances_details(self) -> anyhow::Result<Vec<InstanceDetails>> {
        let mut all_instances = vec![];

        // TODO: it look really inefficient - optimize it
        for name in self.names.iter() {
            println!("test----------------------: {}", name);
            all_instances.push(
                self.get_instance(name)?
                    .get_details()?
            )
        }

        Ok(all_instances)
    }
}
