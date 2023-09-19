use std::path::Path;

use crate::instance::instance_config::BuildConfig;
use crate::utils::docker::Container;
use crate::utils::msma_var_dir;

const IMAGE: &str = "itzg/minecraft-server";
const MC_DEFAULT_PORT: u16 = 25565;
const MC_DATA_CONTAINER_DIR: &str = "/data";

pub fn create(config: BuildConfig) {
    let seed = format!("SEED={}", config.seed.unwrap_or(String::new()));
    let modpack_url = format!("MODPACK={}", config.modpack_zip_url.unwrap_or(String::new()));
    
    let mut host_path = msma_var_dir::init_and_get_instance_dir();
    host_path.push(&config.name);
    
    Container::new(IMAGE)
        .env("EULA=TRUE")
        .env(&format!("VERSION={}", config.game_version))
        .env(&seed)
        .env(&modpack_url)
        .name(config.name)
        .port_mapping(config.port, MC_DEFAULT_PORT)
        .mount(host_path, Path::new(MC_DATA_CONTAINER_DIR))
        .create();
}
