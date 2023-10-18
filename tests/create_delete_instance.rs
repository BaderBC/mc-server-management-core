use std::fs;
use std::process::Command;
use uuid::Uuid;

use mc_server_management_core as msmc;
use mc_server_management_core::utils::msmc_container_dir::init_and_get_container_dir;
use msmc::instance::instance_config::Engine;
use msmc::instance::InstanceBuilder;

// TODO: replace this function with msmc lib function:
pub fn delete_container(name: &str) {
    let status = Command::new("docker")
        .args(["rm", name])
        .status()
        .unwrap();
    let container_path = init_and_get_container_dir(&name);
    let string_path = container_path.to_str().unwrap();
    // Ensure that we will later try to remove correct path
    assert!(string_path.contains("/var/lib/msmc/minecraft_instances"));

    fs::remove_dir_all(container_path).unwrap();

    assert!(status.success());
}

// TODO: replace this function with msmc lib function:
pub fn start_container(name: &str) {
    let status = Command::new("docker")
        .args(["start", name])
        .status()
        .unwrap();

    assert!(status.success());
}

// TODO: replace this function with msmc lib function:
pub fn stop_container(name: &str) {
    let status = Command::new("docker")
        .args(["stop", name])
        .status()
        .unwrap();

    assert!(status.success());
}

pub fn generate_uuid_name_build_config() -> (String, InstanceBuilder) {
    let name = Uuid::new_v4().to_string();

    let config = InstanceBuilder {
        port: 2136,
        name: name.clone(),
        engine: Engine::VANILLA,
        game_version: "1.12".to_string(),
        modpack_zip_url: None,
        seed: Some("2137".to_string()),
    };
    (name, config)
}

pub fn create_instance() -> anyhow::Result<String> {
    let (name, config) = generate_uuid_name_build_config();
    config.create()?;
    Ok(name)
}

#[test]
fn vanilla_1_12() -> anyhow::Result<()> {
    let (name, config) = generate_uuid_name_build_config();
    config.create()?;
    delete_container(&name);
    Ok(())
}

fn start_vanilla_1_12() -> anyhow::Result<()> {
    let (name, config) = generate_uuid_name_build_config();

    config.create()?;
    start_container(&name);
    stop_container(&name);
    delete_container(&name);
    Ok(())
}
