#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct FilesystemChange {
    pub path: String,
    pub kind: u8,
}
