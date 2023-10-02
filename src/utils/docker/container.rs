use std::process::Command;
use serde::de::Unexpected::Str;
use serde_json::Value;

pub struct Container {
    id: String,
}

impl Container {
    pub fn get(identifier: &str) -> Self {
        let container_info = Self::inspect_container(identifier);

        let container_id = container_info["Id"]
            .as_str()
            .expect("Failed to read docker container ID");

        Self {
            id: container_id.to_string()
        }
    }

    pub fn remove(&self) {
        const REMOVE_ERROR_MSG: &str = "Failed to remove container";

        let remove_status = Command::new("docker")
            .args(["rm", &self.id])
            .status()
            .expect(REMOVE_ERROR_MSG);

        assert!(remove_status.success(), "{}", REMOVE_ERROR_MSG);
    }

    pub fn is_running(&self) -> bool {
        let container_info = Self::inspect_container(&self.id);

        container_info["State"]["Running"]
            .as_bool()
            .expect("Failed to read docker container status")
    }

    pub fn start(&self) {
        const RUN_ERROR_MSG: &str = "Failed to start container";

        let mut run_status = Command::new("docker")
            .args(["start", &self.id])
            .status()
            .expect(RUN_ERROR_MSG);

        // We ensure that container stopped in case if container stopped imminently after starting
        assert!(
            run_status.success() && self.is_running(),
            "{}", RUN_ERROR_MSG
        );
    }
    
    pub fn stop(&self) {
        const STOP_ERROR_MSG: &str = "Failed to stop container";
        
        let mut stop_status = Command::new("docker")
            .args(["stop", &self.id])
            .status()
            .expect(STOP_ERROR_MSG);

        // We ensure that container stopped in case if container stopped imminently after starting
        assert!(
            stop_status.success() && !self.is_running(),
            "{}", STOP_ERROR_MSG
        );
    }

    fn inspect_container(identifier: &str) -> Value {
        // identifier can be either name or or ID
        const INSPECT_ERROR_MSG: &str = "Docker container inspect failed";

        let mut inspect_output = Command::new("docker")
            .args(["inspect", identifier])
            .output()
            .expect(INSPECT_ERROR_MSG);

        assert!(inspect_output.status.success(), "{}", INSPECT_ERROR_MSG);
        let inspect_data_str = std::str::from_utf8(&inspect_output.stdout)
            .expect("Docker output cannot be parsed to string");

        serde_json::from_str(inspect_data_str)
            .expect("Failed to convert docker inspect output to Object (serde_json::Value)")
    }
}
