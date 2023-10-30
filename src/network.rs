use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Network {
    pub name: String,
    pub id: String,
    pub created: String,
    pub scope: String,
    pub driver: Option<String>,
    #[serde(rename = "EnableIPv6")]
    pub enable_ipv6: bool,
    pub internal: bool,
    pub attachable: bool,
    pub ingress: bool,
    pub options: HashMap<String, String>,
    pub labels: Option<HashMap<String, String>>, //IPAM
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkCreate {
    pub name: String,
    pub check_duplicate: Option<bool>,
    pub driver: Option<String>,
    pub internal: Option<bool>,
    pub attachable: Option<bool>,
    pub ingress: Option<bool>,
    #[serde(rename = "EnableIPv6")]
    pub enable_ipv6: Option<bool>,
    pub options: Option<HashMap<String, String>>,
    pub labels: Option<HashMap<String, String>>, //IPAM
}
