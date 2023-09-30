use std::fmt::{Display, Formatter};
use std::path::Path;
use crate::instance::instance_config::Engine;
use crate::utils::docker::Container;
use crate::utils::msmc_var_dir;

const IMAGE: &str = "itzg/minecraft-server";
const MC_DEFAULT_PORT: u16 = 25565;
const MC_DATA_CONTAINER_DIR: &str = "/data";

pub struct BuildConfig {
    pub port: u16,
    pub name: String,
    pub engine: Engine,
    pub game_version: String,
    pub modpack_zip_url: Option<String>,
    pub seed: Option<String>,
}

impl BuildConfig {
    pub fn default(name: String) -> BuildConfig {
        BuildConfig {
            port: 25565,
            name,
            engine: Engine::VANILLA,
            game_version: "1.12".to_string(),
            modpack_zip_url: None,
            seed: None,
        }
    }

    pub fn create(self) {
        let seed = format!("SEED={}", self.seed.unwrap_or(String::new()));
        let modpack_url = format!("MODPACK={}", self.modpack_zip_url.unwrap_or(String::new()));

        let mut host_path = msmc_var_dir::init_and_get_instance_dir();
        host_path.push(&self.name);

        Container::new(IMAGE)
            .env("EULA=TRUE")
            .env(&format!("VERSION={}", self.game_version))
            .env(&seed)
            .env(&modpack_url)
            .name(self.name)
            .port_mapping(self.port, MC_DEFAULT_PORT)
            .mount(host_path, Path::new(MC_DATA_CONTAINER_DIR))
            .create();
    }
}

impl Display for BuildConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Instance name: {},\nPort: {},\nEngine: {},\nVersion: {}", self.name, self.port, self.engine, self.game_version)
    }
}
