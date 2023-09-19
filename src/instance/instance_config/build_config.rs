use std::fmt::{Display, Formatter};
use crate::instance::instance_config::Engine;

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
}

impl Display for BuildConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Instance name: {},\nPort: {},\nEngine: {},\nVersion: {}", self.name, self.port, self.engine, self.game_version)
    }
}
