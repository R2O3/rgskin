use merge::Merge;

use crate::derive_merge_for_all;

derive_merge_for_all! {
    strategy = crate::utils::merge::any::overwrite;
    #[derive(Clone, Merge)]
    pub struct Metadata {
        pub name: String,
        pub creator: String,
        pub version: String,
    }
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