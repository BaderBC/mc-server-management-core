use std::fmt::Error;
use std::fs::File;
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};
use crate::utils::msmc_var_dir::init_and_get_instance_dir;


macro_rules! unwrap_or_return_default {
    {$option:expr} => {
        match $option {
            Some(v) => v,
            None => return Default::default()
        }
    }
}

pub trait ConfigTrait where for<'a> Self: Deserialize<'a> {
    const CONFIG_FILENAME: &'static str;
    fn read(container_name: &str) -> Vec<Self> {
        let file_name = <Self as ConfigTrait>::CONFIG_FILENAME.to_string();
        let config = read_file(container_name, &file_name);
        let config = unwrap_or_return_default!(config);

        serde_json::from_str::<Vec<Self>>(&config).unwrap()
    }
}

pub trait PlayerConfigTrait {
    #[allow(clippy::wrong_self_convention)]
    fn container_name(&self) -> String;

    fn save<Config>(&self) -> std::io::Result<()> where Config: ConfigTrait, Self: Serialize + Sized {
        let container_name = self.container_name();
        let json_config = serde_json::to_string(self).unwrap();

        write_file(&container_name, Config::CONFIG_FILENAME, json_config);
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WhitelistConfig {
    pub uuid: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpsConfig {
    pub uuid: String,
    pub name: String,
    pub level: u8,
    #[serde(alias = "bypassesPlayerLimit")]
    pub bypasses_player_limit: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BannedPlayersConfig {
    pub uuid: String,
    pub name: String,
    pub created: String,
    pub source: String,
    pub expires: String,
    pub reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BannedIpsConfig {
    pub ip: String,
    pub created: String,
    pub source: String,
    pub expires: String,
    pub reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Whitelist<'a> {
    pub config: Vec<WhitelistConfig>,
    is_modified: bool,
    container_name: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ops<'a> {
    pub config: Vec<OpsConfig>,
    is_modified: bool,
    container_name: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BannedPlayers<'a> {
    pub config: Vec<BannedPlayersConfig>,
    is_modified: bool,
    container_name: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BannedIps<'a> {
    pub config: Vec<BannedIpsConfig>,
    is_modified: bool,
    container_name: &'a str,
}

impl<'a> ConfigTrait for WhitelistConfig {
    const CONFIG_FILENAME: &'static str = "whitelist.json";
}

impl<'a> ConfigTrait for OpsConfig {
    const CONFIG_FILENAME: &'static str = "ops.json";
}

impl<'a> ConfigTrait for BannedPlayersConfig {
    const CONFIG_FILENAME: &'static str = "banned-players.json";
}

impl<'a> ConfigTrait for BannedIpsConfig {
    const CONFIG_FILENAME: &'static str = "banned-ips.json";
}

impl<'a> PlayerConfigTrait for Whitelist<'a> {
    fn container_name(&self) -> String { self.container_name.to_string() }
}

impl<'a> Whitelist<'a> {
    pub fn read(container_name: &'a str) -> Self {
        Self {
            config: WhitelistConfig::read(container_name),
            is_modified: false,
            container_name,
        }
    }

    pub fn remove_player(mut self, name: String) {
        // TODO: find a way to implement e.g. some type of inheritance, because of DRY
        let index = self
            .config
            .iter()
            .position(|e| e.name == name);
        let index = unwrap_or_return_default!(index);
        self.config.remove(index);
    }
}

impl<'a> PlayerConfigTrait for Ops<'a> {
    fn container_name(&self) -> String { self.container_name.to_string() }
}

impl<'a> Ops<'a> {
    pub fn read(container_name: &'a str) -> Self {
        Self {
            config: OpsConfig::read(container_name),
            is_modified: false,
            container_name,
        }
    }
    pub fn remove_player(mut self, name: String) {
        // TODO: find a way to implement e.g. some type of inheritance, because of DRY
        let index = self
            .config
            .iter()
            .position(|e| e.name == name);
        let index = unwrap_or_return_default!(index);
        self.config.remove(index);
    }
}

impl<'a> PlayerConfigTrait for BannedPlayers<'a> {
    fn container_name(&self) -> String { self.container_name.to_string() }
}

impl<'a> BannedPlayers<'a> {
    pub fn read(container_name: &'a str) -> Self {
        Self {
            config: BannedPlayersConfig::read(container_name),
            is_modified: false,
            container_name,
        }
    }
    pub fn remove_player(mut self, name: String) {
        // TODO: find a way to implement e.g. some type of inheritance, because of DRY
        let index = self
            .config
            .iter()
            .position(|e| e.name == name);
        let index = unwrap_or_return_default!(index);
        self.config.remove(index);
    }
}

impl<'a> PlayerConfigTrait for BannedIps<'a> {
    fn container_name(&self) -> String { self.container_name.to_string() }
}

impl<'a> BannedIps<'a> {
    pub fn read(container_name: &'a str) -> Self {
        Self {
            config: BannedIpsConfig::read(container_name),
            is_modified: false,
            container_name,
        }
    }
    pub fn remove_player(mut self, ip: String) {
        // TODO: find a way to implement e.g. some type of inheritance, because of DRY
        let index = self
            .config
            .iter()
            .position(|e| e.ip == ip);
        let index = unwrap_or_return_default!(index);
        self.config.remove(index);
    }
}

fn get_file(container_name: &str, file_name: &str) -> Option<File> {
    let mut file_path = init_and_get_instance_dir();
    file_path.push(container_name);
    file_path.push(file_name);

    File::open(file_path).ok()
}


fn read_file(container_name: &str, file_name: &str) -> Option<String> {
    let mut file_content_buf = String::new();

    let mut file = get_file(container_name, file_name)?;
    file.read_to_string(&mut file_content_buf).ok()?;

    Some(file_content_buf)
}

fn write_file(container_name: &str, file_name: &str, content: String) -> Result<(), String> {
    let mut file = get_file(container_name, file_name).ok_or("Failed to open file")?;
    file.write(content.as_ref()).map_err(|_| { "Failed to write file".to_string() })?;
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn macro_test() {
        let should_be_empty = _macro_test(None);
        let should_contain_one_element = _macro_test(Some(1));

        assert_eq!(should_be_empty.len(), 0);
        assert_eq!(should_contain_one_element.len(), 1);
    }

    fn _macro_test(option: Option<i32>) -> Vec<i32> {
        let option_value = unwrap_or_return_default!(option);

        match option {
            Some(_) => vec![option_value],
            None => unreachable!(),
        }
    }
}
