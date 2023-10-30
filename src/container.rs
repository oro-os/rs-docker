use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Container {
    pub id: String,
    pub image: String,
    pub status: String,
    pub command: String,
    pub created: u64,
    pub names: Vec<String>,
    pub ports: Vec<Port>,
    pub size_rw: Option<u64>, // I guess it is optional on Mac.
    pub size_root_fs: u64,
    pub labels: Option<HashMap<String, String>>,
    pub host_config: HostConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Port {
    #[serde(rename = "IP")]
    pub ip: Option<String>,
    pub private_port: u64,
    pub public_port: Option<u64>,
    #[serde(rename = "Type")]
    pub ty: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct HostConfig {
    pub network_mode: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInfo {
    pub app_armor_profile: String,
    pub args: Vec<String>,
    // Config
    pub created: String,
    pub driver: String,
    pub exec_driver: String,
    // ExecIDs
    // HostConfig
    pub hostname_path: String,
    pub hosts_path: String,
    pub log_path: String,
    pub id: String,
    pub image: String,
    pub mount_label: String,
    pub name: String,
    // NetworkSettings
    pub path: String,
    pub process_label: String,
    pub resolv_conf_path: String,
    pub restart_count: u64,
    // State
    pub volumes: HashMap<String, String>,
    #[serde(rename = "VolumesRW")]
    pub volumes_rw: HashMap<String, bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PortBinding {
    pub host_ip: Option<String>,
    pub host_port: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct HostConfigCreate {
    pub network_mode: Option<String>,
    pub publish_all_ports: Option<bool>,
    pub port_bindings: Option<HashMap<String, Vec<PortBinding>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerCreate {
    pub image: String,
    pub labels: Option<HashMap<String, String>>,
    pub exposed_ports: Option<HashMap<String, HashMap<i32, i32>>>,
    pub host_config: Option<HostConfigCreate>,
}
