use std::collections::HashMap;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;

pub type DockerInspect = Vec<Info>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Created")]
    pub created: String,
    #[serde(rename = "Path")]
    pub path: String,
    #[serde(rename = "Args")]
    pub args: Vec<Value>,
    #[serde(rename = "State")]
    pub state: State,
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "ResolvConfPath")]
    pub resolv_conf_path: String,
    #[serde(rename = "HostnamePath")]
    pub hostname_path: String,
    #[serde(rename = "HostsPath")]
    pub hosts_path: String,
    #[serde(rename = "LogPath")]
    pub log_path: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "RestartCount")]
    pub restart_count: i64,
    #[serde(rename = "Driver")]
    pub driver: String,
    #[serde(rename = "Platform")]
    pub platform: String,
    #[serde(rename = "MountLabel")]
    pub mount_label: String,
    #[serde(rename = "ProcessLabel")]
    pub process_label: String,
    #[serde(rename = "AppArmorProfile")]
    pub app_armor_profile: String,
    #[serde(rename = "ExecIDs")]
    pub exec_ids: Value,
    #[serde(rename = "HostConfig")]
    pub host_config: HostConfig,
    #[serde(rename = "GraphDriver")]
    pub graph_driver: GraphDriver,
    #[serde(rename = "Mounts")]
    pub mounts: Vec<Mount>,
    #[serde(rename = "Config")]
    pub config: Config,
    #[serde(rename = "NetworkSettings")]
    pub network_settings: NetworkSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Running")]
    pub running: bool,
    #[serde(rename = "Paused")]
    pub paused: bool,
    #[serde(rename = "Restarting")]
    pub restarting: bool,
    #[serde(rename = "OOMKilled")]
    pub oomkilled: bool,
    #[serde(rename = "Dead")]
    pub dead: bool,
    #[serde(rename = "Pid")]
    pub pid: i64,
    #[serde(rename = "ExitCode")]
    pub exit_code: i64,
    #[serde(rename = "Error")]
    pub error: String,
    #[serde(rename = "StartedAt")]
    pub started_at: String,
    #[serde(rename = "FinishedAt")]
    pub finished_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostConfig {
    #[serde(rename = "Binds")]
    pub binds: Value,
    #[serde(rename = "ContainerIDFile")]
    pub container_idfile: String,
    #[serde(rename = "LogConfig")]
    pub log_config: LogConfig,
    #[serde(rename = "NetworkMode")]
    pub network_mode: String,
    #[serde(rename = "PortBindings")]
    pub port_bindings: PortBindings,
    #[serde(rename = "RestartPolicy")]
    pub restart_policy: RestartPolicy,
    #[serde(rename = "AutoRemove")]
    pub auto_remove: bool,
    #[serde(rename = "VolumeDriver")]
    pub volume_driver: String,
    #[serde(rename = "VolumesFrom")]
    pub volumes_from: Value,
    #[serde(rename = "ConsoleSize")]
    pub console_size: Vec<i64>,
    #[serde(rename = "CapAdd")]
    pub cap_add: Value,
    #[serde(rename = "CapDrop")]
    pub cap_drop: Value,
    #[serde(rename = "CgroupnsMode")]
    pub cgroupns_mode: String,
    #[serde(rename = "Dns")]
    pub dns: Vec<Value>,
    #[serde(rename = "DnsOptions")]
    pub dns_options: Vec<Value>,
    #[serde(rename = "DnsSearch")]
    pub dns_search: Vec<Value>,
    #[serde(rename = "ExtraHosts")]
    pub extra_hosts: Value,
    #[serde(rename = "GroupAdd")]
    pub group_add: Value,
    #[serde(rename = "IpcMode")]
    pub ipc_mode: String,
    #[serde(rename = "Cgroup")]
    pub cgroup: String,
    #[serde(rename = "Links")]
    pub links: Value,
    #[serde(rename = "OomScoreAdj")]
    pub oom_score_adj: i64,
    #[serde(rename = "PidMode")]
    pub pid_mode: String,
    #[serde(rename = "Privileged")]
    pub privileged: bool,
    #[serde(rename = "PublishAllPorts")]
    pub publish_all_ports: bool,
    #[serde(rename = "ReadonlyRootfs")]
    pub readonly_rootfs: bool,
    #[serde(rename = "SecurityOpt")]
    pub security_opt: Value,
    #[serde(rename = "UTSMode")]
    pub utsmode: String,
    #[serde(rename = "UsernsMode")]
    pub userns_mode: String,
    #[serde(rename = "ShmSize")]
    pub shm_size: i64,
    #[serde(rename = "Runtime")]
    pub runtime: String,
    #[serde(rename = "Isolation")]
    pub isolation: String,
    #[serde(rename = "CpuShares")]
    pub cpu_shares: i64,
    #[serde(rename = "Memory")]
    pub memory: i64,
    #[serde(rename = "NanoCpus")]
    pub nano_cpus: i64,
    #[serde(rename = "CgroupParent")]
    pub cgroup_parent: String,
    #[serde(rename = "BlkioWeight")]
    pub blkio_weight: i64,
    #[serde(rename = "BlkioWeightDevice")]
    pub blkio_weight_device: Vec<Value>,
    #[serde(rename = "BlkioDeviceReadBps")]
    pub blkio_device_read_bps: Vec<Value>,
    #[serde(rename = "BlkioDeviceWriteBps")]
    pub blkio_device_write_bps: Vec<Value>,
    #[serde(rename = "BlkioDeviceReadIOps")]
    pub blkio_device_read_iops: Vec<Value>,
    #[serde(rename = "BlkioDeviceWriteIOps")]
    pub blkio_device_write_iops: Vec<Value>,
    #[serde(rename = "CpuPeriod")]
    pub cpu_period: i64,
    #[serde(rename = "CpuQuota")]
    pub cpu_quota: i64,
    #[serde(rename = "CpuRealtimePeriod")]
    pub cpu_realtime_period: i64,
    #[serde(rename = "CpuRealtimeRuntime")]
    pub cpu_realtime_runtime: i64,
    #[serde(rename = "CpusetCpus")]
    pub cpuset_cpus: String,
    #[serde(rename = "CpusetMems")]
    pub cpuset_mems: String,
    #[serde(rename = "Devices")]
    pub devices: Vec<Value>,
    #[serde(rename = "DeviceCgroupRules")]
    pub device_cgroup_rules: Value,
    #[serde(rename = "DeviceRequests")]
    pub device_requests: Value,
    #[serde(rename = "MemoryReservation")]
    pub memory_reservation: i64,
    #[serde(rename = "MemorySwap")]
    pub memory_swap: i64,
    #[serde(rename = "MemorySwappiness")]
    pub memory_swappiness: Value,
    #[serde(rename = "OomKillDisable")]
    pub oom_kill_disable: bool,
    #[serde(rename = "PidsLimit")]
    pub pids_limit: Value,
    #[serde(rename = "Ulimits")]
    pub ulimits: Value,
    #[serde(rename = "CpuCount")]
    pub cpu_count: i64,
    #[serde(rename = "CpuPercent")]
    pub cpu_percent: i64,
    #[serde(rename = "IOMaximumIOps")]
    pub iomaximum_iops: i64,
    #[serde(rename = "IOMaximumBandwidth")]
    pub iomaximum_bandwidth: i64,
    #[serde(rename = "MaskedPaths")]
    pub masked_paths: Vec<String>,
    #[serde(rename = "ReadonlyPaths")]
    pub readonly_paths: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogConfig {
    #[serde(rename = "Type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortBindings {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestartPolicy {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "MaximumRetryCount")]
    pub maximum_retry_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphDriver {
    #[serde(rename = "Data")]
    pub data: Data,
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "LowerDir")]
    pub lower_dir: String,
    #[serde(rename = "MergedDir")]
    pub merged_dir: String,
    #[serde(rename = "UpperDir")]
    pub upper_dir: String,
    #[serde(rename = "WorkDir")]
    pub work_dir: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mount {
    #[serde(rename = "Type")]
    pub type_field: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Source")]
    pub source: String,
    #[serde(rename = "Destination")]
    pub destination: String,
    #[serde(rename = "Driver")]
    pub driver: String,
    #[serde(rename = "Mode")]
    pub mode: String,
    #[serde(rename = "RW")]
    pub rw: bool,
    #[serde(rename = "Propagation")]
    pub propagation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(rename = "Hostname")]
    pub hostname: String,
    #[serde(rename = "Domainname")]
    pub domainname: String,
    #[serde(rename = "User")]
    pub user: String,
    #[serde(rename = "AttachStdin")]
    pub attach_stdin: bool,
    #[serde(rename = "AttachStdout")]
    pub attach_stdout: bool,
    #[serde(rename = "AttachStderr")]
    pub attach_stderr: bool,
    #[serde(rename = "ExposedPorts")]
    pub exposed_ports: HashMap<String, ExposedPort>,
    #[serde(rename = "Tty")]
    pub tty: bool,
    #[serde(rename = "OpenStdin")]
    pub open_stdin: bool,
    #[serde(rename = "StdinOnce")]
    pub stdin_once: bool,
    #[serde(rename = "Env")]
    pub env: Vec<String>,
    #[serde(rename = "Cmd")]
    pub cmd: Value,
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "Volumes")]
    pub volumes: Volumes,
    #[serde(rename = "WorkingDir")]
    pub working_dir: String,
    #[serde(rename = "Entrypoint")]
    pub entrypoint: Vec<String>,
    #[serde(rename = "OnBuild")]
    pub on_build: Value,
    #[serde(rename = "Labels")]
    pub labels: Labels,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ExposedPort {
    // TODO: extend this one
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Volumes {
    #[serde(rename = "/var/lib/docker")]
    pub var_lib_docker: VarLibDocker,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VarLibDocker {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Labels {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkSettings {
    #[serde(rename = "Bridge")]
    pub bridge: String,
    #[serde(rename = "SandboxID")]
    pub sandbox_id: String,
    #[serde(rename = "HairpinMode")]
    pub hairpin_mode: bool,
    #[serde(rename = "LinkLocalIPv6Address")]
    pub link_local_ipv6address: String,
    #[serde(rename = "LinkLocalIPv6PrefixLen")]
    pub link_local_ipv6prefix_len: i64,
    #[serde(rename = "Ports")]
    pub ports: Ports,
    #[serde(rename = "SandboxKey")]
    pub sandbox_key: String,
    #[serde(rename = "SecondaryIPAddresses")]
    pub secondary_ipaddresses: Value,
    #[serde(rename = "SecondaryIPv6Addresses")]
    pub secondary_ipv6addresses: Value,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: String,
    #[serde(rename = "Gateway")]
    pub gateway: String,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6address: String,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ipv6prefix_len: i64,
    #[serde(rename = "IPAddress")]
    pub ipaddress: String,
    #[serde(rename = "IPPrefixLen")]
    pub ipprefix_len: i64,
    #[serde(rename = "IPv6Gateway")]
    pub ipv6gateway: String,
    #[serde(rename = "MacAddress")]
    pub mac_address: String,
    #[serde(rename = "Networks")]
    pub networks: Networks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ports {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Networks {
    pub bridge: Bridge,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bridge {
    #[serde(rename = "IPAMConfig")]
    pub ipamconfig: Value,
    #[serde(rename = "Links")]
    pub links: Value,
    #[serde(rename = "Aliases")]
    pub aliases: Value,
    #[serde(rename = "NetworkID")]
    pub network_id: String,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: String,
    #[serde(rename = "Gateway")]
    pub gateway: String,
    #[serde(rename = "IPAddress")]
    pub ipaddress: String,
    #[serde(rename = "IPPrefixLen")]
    pub ipprefix_len: i64,
    #[serde(rename = "IPv6Gateway")]
    pub ipv6gateway: String,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6address: String,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ipv6prefix_len: i64,
    #[serde(rename = "MacAddress")]
    pub mac_address: String,
    #[serde(rename = "DriverOpts")]
    pub driver_opts: Value,
}
