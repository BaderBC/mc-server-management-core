use std::path::Path;

use crate::{InstanceConfig};
use crate::lib::docker::Container;
use crate::utils::msma_dir;

const IMAGE: &str = "itzg/minecraft-server";
const MC_DEFAULT_PORT: u16 = 25565;
const MC_DATA_CONTAINER_DIR: &str = "/data";

pub fn create(config: InstanceConfig) {
    Container::new(IMAGE)
        .name(config.name)
        .port_mapping(config.port, MC_DEFAULT_PORT)
        .mount(msma_dir::get_and_fix_instance_folder(),Path::new(MC_DATA_CONTAINER_DIR))
        .create();
}
