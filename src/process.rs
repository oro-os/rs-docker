pub struct Process {
    pub user: String,
    pub pid: String,
    pub cpu: Option<String>,
    pub memory: Option<String>,
    pub vsz: Option<String>,
    pub rss: Option<String>,
    pub tty: Option<String>,
    pub stat: Option<String>,
    pub start: Option<String>,
    pub time: Option<String>,
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Top {
    pub titles: Vec<String>,
    pub processes: Vec<Vec<String>>,
}
