#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SystemInfo {
    pub containers: u64,
    pub images: u64,
    pub driver: String,
    pub driver_status: Vec<(String, String)>,
    pub execution_driver: String,
    pub kernel_version: String,
    #[serde(rename = "NCPU")]
    pub ncpu: u64,
    pub mem_total: u64,
    pub name: String,
    #[serde(rename = "ID")]
    pub id: String,
    pub debug: u64, // bool
    pub n_fd: u64,
    pub n_goroutines: u64,
    pub n_events_listener: u64,
    pub init_path: String,
    pub init_sha1: String,
    pub index_server_address: String,
    pub memory_limit: u64, // bool
    pub swap_limit: u64,   // bool
    #[serde(rename = "IPv4Forwarding")]
    pub ipv4_forwarding: u64, // bool
    pub labels: Option<Vec<String>>,
    pub docker_root_dir: String,
    pub operating_system: String,
}
