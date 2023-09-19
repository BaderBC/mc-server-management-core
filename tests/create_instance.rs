use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use uuid::Uuid;
use mc_server_management_api::instance::create;
use mc_server_management_api::instance::instance_config::{BuildConfig, Engine};

// TODO: replace this function with msma lib function:
fn delete_container(name: String) {
    let status = Command::new("docker")
        .args(["rm", &name])
        .status()
        .unwrap();
    
    assert!(status.success());
}

#[test]
fn vanilla_1_12() {
    let name = Uuid::new_v4().to_string();
    
    let config = BuildConfig {
        port: 2136,
        name: name.clone(),
        engine: Engine::VANILLA,
        game_version: "1.12".to_string(),
        modpack_zip_url: None,
        seed: Some("135215325".to_string()),
    };
    create(config);
    delete_container(name);
}
