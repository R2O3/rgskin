pub struct Metadata {
    pub name: String,
    pub creator: String,
    pub version: String,
    pub center_cursor: bool,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            creator: "Unknown".to_string(),
            version: "latest".to_string(),
            center_cursor: true
        }
    }
}