
use uuid::Uuid;

use mc_server_management_core as msmc;
use mc_server_management_core::instance::InstanceBuilder;
use msmc::instance::instance_config::players_config::{
    Whitelist,
    WhitelistConfig,
    PlayerConfigTrait,
};

#[test]
fn remove_non_existing_player() -> anyhow::Result<()> {
    let instance_name = Uuid::new_v4().to_string();
    let instance = InstanceBuilder::new(instance_name.clone())
        .create()?;

    instance.start()?;

    // TODO: wait till common is fully initialize (we should use msmc lib for this)
    let whitelist = Whitelist::read(&instance_name);
    assert_eq!(whitelist.config.len(), 0);
    // Shouldn't panic when player isn't on the list
    whitelist.remove_player("random_stuff".to_string());

    instance.stop()?;
    instance.delete()?;
    Ok(())
}

//#[test]
#[allow(dead_code)]
fn read_add_remove_whitelist_on_started_container() -> anyhow::Result<()> {
    let instance_name = Uuid::new_v4().to_string();
    let instance = InstanceBuilder::new(instance_name.clone())
        .create()?;
    
    instance.start()?;
    // TODO: wait till common is fully initialize (we should use msmc lib for this)
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

    instance.stop()?;
    unimplemented!() //Makes sure that this function fails
}

// TODO: even though most of PlayerConfig uses same methods:
// TODO:               IMPLEMENT MORE TESTS