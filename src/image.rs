#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Image {
    pub created: u64,
    pub id: String,
    pub parent_id: String,
    pub repo_tags: Vec<String>,
    pub size: u64,
    pub virtual_size: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
// NOTE(qix-): I guess this doesn't need PascalCase?
pub struct ImageStatus {
    pub status: Option<String>,
    pub error: Option<String>,
}
