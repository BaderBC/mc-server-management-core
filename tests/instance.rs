use uuid::Uuid;
use mc_server_management_core as msmc;
use mc_server_management_core::instance::{Instance, Instances};
use msmc::instance::InstanceBuilder;

#[test]
fn main() -> anyhow::Result<()> {
    let instance_name = create()?;
    
    get_details(&instance_name)?;
    get_instances_details()?;

    delete(&instance_name)?;
    Ok(())
}

fn create() -> anyhow::Result<String> {
    let instance_name = Uuid::new_v4().to_string();
    InstanceBuilder::new(instance_name.clone())
        .create()?;

    Ok(instance_name)
}

fn get_details(name: &str) -> anyhow::Result<()> {
    let instance = Instance::get(name)?;
    let details = instance.get_details()?;
    assert_eq!(details.name, name);
    
    Ok(())
}

fn get_instances_details() -> anyhow::Result<()> {
    let instances = Instances::get()?;
    assert!(instances.get_instances_details()?.len() > 0);

    Ok(())
}

fn delete(name: &str) -> anyhow::Result<()> {
    Instance::get(name)?
        .delete()?;

    Ok(())
}