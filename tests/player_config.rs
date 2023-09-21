mod create_delete_instance;

use create_delete_instance::{create_instance, start_container, stop_container, delete_container};

use mc_server_management_core as msmc;
use msmc::instance::instance_config::players_config::{
    Whitelist,
    WhitelistConfig,
    PlayerConfigTrait,
};

#[test]
fn remove_non_existing_player() {
    let instance_name = create_instance();
    start_container(&instance_name);

    // TODO: wait till instance is fully initialize (we should use msmc lib for this)
    let whitelist = Whitelist::read(&instance_name);
    assert_eq!(whitelist.config.len(), 0);
    // Shouldn't panic when player isn't on the list
    whitelist.remove_player("random_stuff".to_string());

    stop_container(&instance_name);
    delete_container(&instance_name);
}

//#[test]
#[allow(dead_code)]
fn read_add_remove_whitelist_on_started_container() {
    let instance_name = create_instance();
    start_container(&instance_name);

    // TODO: wait till instance is fully initialize (we should use msmc lib for this)
    let whitelist = Whitelist::read(&instance_name);
    assert_eq!(whitelist.config.len(), 0);

    // TODO: implement this functionality:
    //whitelist.add_player("test".to_string());
    whitelist.save::<WhitelistConfig>().unwrap();
    let whitelist = Whitelist::read(&instance_name);
    assert_eq!(whitelist.config.len(), 1);
    assert_eq!(whitelist.config[0].name, "test".to_string());

    whitelist.remove_player("test".to_string());
    let whitelist = Whitelist::read(&instance_name);
    assert_eq!(whitelist.config.len(), 0);

    stop_container(&instance_name);
    delete_container(&instance_name);
    unimplemented!() //Makes sure that this function fails
}

// TODO: even though most of PlayerConfig uses same methods:
// TODO:               IMPLEMENT MORE TESTS