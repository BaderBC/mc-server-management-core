use std::fmt::{Display, Formatter, write};
use std::net::Shutdown::Write;

pub enum Engine {
    FORGE,
    VANILLA,
}

pub struct InstanceConfig {
    pub port: u16,
    pub name: String,
    pub engine: Engine,
    pub engine_version: String,
}

impl InstanceConfig {
    pub fn default(name: String) -> InstanceConfig {
        InstanceConfig {
            port: 25565,
            name,
            engine: Engine::VANILLA,
            engine_version: "1.12".to_string(),
        }
    }
}

impl Display for Engine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Engine::FORGE => write!(f, "FORGE"),
            Engine::VANILLA => write!(f, "VANILLA"),
        }
    }
}

impl Display for InstanceConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Instance name: {},\nPort: {},\nEngine: {},\nVersion: {}", self.name, self.port, self.engine, self.engine_version)
    }
}
