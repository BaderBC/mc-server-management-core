use std::fmt::{Display, Formatter};

pub enum Engine {
    FORGE(String),
    VANILLA,
}

impl Display for Engine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Engine::FORGE(version) => write!(f, "FORGE version: {}", version),
            Engine::VANILLA => write!(f, "VANILLA"),
        }
    }
}

