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

trait ReadConfigVec where for<'a> Self: Deserialize<'a> {
    const CONFIG_FILENAME: &'static str;
    fn read(container_name: &str) -> Vec<Self> {
        let file_name = <Self as ReadConfigVec>::CONFIG_FILENAME.to_string();
        let config = read_file(container_name, &file_name);
        let config = unwrap_or_return_default!(config);

        serde_json::from_str::<Vec<Self>>(&config).unwrap()
    }
}

trait SaveAndModify {
    #[allow(clippy::wrong_self_convention)]
    fn is_modified(&self) -> bool;
    fn container_name(&self) -> &'static str;

    fn save<T>(self) -> std::io::Result<()> where T: ReadConfigVec, Self: Serialize + Sized {
        if !self.is_modified() { return Ok(()); }

        let container_name = self.container_name();
        let json_config = serde_json::to_string(&self).unwrap();

        write_file(&container_name, T::CONFIG_FILENAME, json_config);
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

impl<'a> ReadConfigVec for WhitelistConfig {
    const CONFIG_FILENAME: &'static str = "whitelist.json";
}

impl<'a> ReadConfigVec for OpsConfig {
    const CONFIG_FILENAME: &'static str = "ops.json";
}

impl<'a> ReadConfigVec for BannedPlayersConfig {
    const CONFIG_FILENAME: &'static str = "banned-players.json";
}

impl<'a> ReadConfigVec for BannedIpsConfig {
    const CONFIG_FILENAME: &'static str = "banned-ips.json";
}

impl<'a> Whitelist<'a> {
    pub fn read(container_name: &'a str) -> Self {
        Self {
            config: WhitelistConfig::read(container_name),
            is_modified: false,
            container_name,
        }
    }
}

impl<'a> Ops<'a> {
    pub fn read(container_name: &'a str) -> Self {
        Self {
            config: OpsConfig::read(container_name),
            is_modified: false,
            container_name,
        }
    }
}

impl<'a> BannedPlayers<'a> {
    pub fn read(container_name: &'a str) -> Self {
        Self {
            config: BannedPlayersConfig::read(container_name),
            is_modified: false,
            container_name,
        }
    }
}

impl<'a> BannedIps<'a> {
    pub fn read(container_name: &'a str) -> Self {
        Self {
            config: BannedIpsConfig::read(container_name),
            is_modified: false,
            container_name,
        }
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
    // TODO: add more tests
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
