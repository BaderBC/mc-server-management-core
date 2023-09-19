use std::fmt::{Display, Formatter};

pub enum Engine {
    FORGE(String),
    VANILLA,
}

pub struct InstanceConfig {
    pub port: u16,
    pub name: String,
    pub engine: Engine,
    pub game_version: String,
}

impl InstanceConfig {
    pub fn default(name: String) -> InstanceConfig {
        InstanceConfig {
            port: 25565,
            name,
            engine: Engine::VANILLA,
            game_version: "1.12".to_string(),
        }
    }
}

impl Display for Engine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Engine::FORGE(verison) => write!(f, "FORGE version: {}", verison),
            Engine::VANILLA => write!(f, "VANILLA"),
        }
    }
}

impl Display for InstanceConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Instance name: {},\nPort: {},\nEngine: {},\nVersion: {}", self.name, self.port, self.engine, self.game_version)
    }
}
