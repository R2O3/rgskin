use merge::Merge;
use rgskin_derive::merge_for_all;

#[merge_for_all(strategy = crate::utils::merge::skip)]
#[derive(Clone, Merge)]
pub struct Metadata {
    pub name: String,
    pub creator: String,
    pub version: String,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            creator: "Unknown".to_string(),
            version: "latest".to_string()
        }
    }
}
