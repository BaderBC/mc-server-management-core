use std::process::Command;
use anyhow::anyhow;
use serde::de::Unexpected::Str;
use serde_json::Value;

pub struct Container {
    id: String,
}

impl Container {
    pub fn get(identifier: &str) -> anyhow::Result<Self> {
        let container_info = Self::inspect_container(identifier)?;

        let container_id = container_info["Id"]
            .as_str()
            .ok_or(anyhow::Error::msg("No \"Id\" field on docker inspect result"))?;

        Ok(
            Self { id: container_id.to_string() }
        )
    }

    pub fn remove(&self) -> anyhow::Result<()> {
        const REMOVE_ERROR_MSG: &str = "Failed to remove container";

        let remove_status = Command::new("docker")
            .args(["rm", &self.id])
            .status()?;

        if !remove_status.success() {
            return Err(anyhow::Error::msg(REMOVE_ERROR_MSG));
        }
        Ok(())
    }

    pub fn is_running(&self) -> anyhow::Result<bool> {
        let container_info = Self::inspect_container(&self.id)?;

        Ok(
            container_info["State"]["Running"]
                .as_bool()
                .ok_or(anyhow::Error::msg("Failed to read docker container status"))?
        )
    }

    pub fn start(&self) -> anyhow::Result<()> {
        const RUN_ERROR_MSG: &str = "Failed to start container";

        let mut run_status = Command::new("docker")
            .args(["start", &self.id])
            .status()?;

        // We ensure that container stopped in case if container stopped imminently after starting
        if !run_status.success() {
            return Err(anyhow::Error::msg(RUN_ERROR_MSG));
        }
        Ok(())
    }

    pub fn stop(&self) -> anyhow::Result<()> {
        const STOP_ERROR_MSG: &str = "Failed to stop container";

        let mut stop_status = Command::new("docker")
            .args(["stop", &self.id])
            .status()?;

        // Ensure that container stopped in case if container stopped imminently after starting
        if !stop_status.success() {
            return Err(anyhow::Error::msg(STOP_ERROR_MSG));
        }
        Ok(())
    }

    fn inspect_container(identifier: &str) -> anyhow::Result<Value> {
        // identifier can be either name or or ID
        const INSPECT_ERROR_MSG: &str = "Docker container inspect failed";

        let mut inspect_output = Command::new("docker")
            .args(["inspect", identifier])
            .output()?;
        
        if !inspect_output.status.success() {
            return Err(anyhow::Error::msg(INSPECT_ERROR_MSG));
        }

        let inspect_data_str: Result<&str, anyhow::Error> = match std::str::from_utf8(&inspect_output.stdout) {
            Ok(v) => Ok(v),
            Err(_) => Err(anyhow::Error::msg("Docker output cannot be parsed to string"))
        };
        let inspect_data_str = inspect_data_str?;

        let json_res = match serde_json::from_str::<Value>(inspect_data_str) {
            Ok(v) => Ok(v),
            Err(_) => Err(anyhow::Error::msg("Failed to convert docker inspect output to Object (serde_json::Value)"))
        }?;
        
        Ok(
            json_res[0].to_owned()
        )
    }
}
