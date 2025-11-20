use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Info {
    pub name: String,
    pub creator: String,
    pub accent: String,
}

impl Default for Info {
    fn default() -> Self {
        Self {
            name: String::new(),
            creator: String::new(),
            accent: String::from("#FFFFFF"),
        }
    }
}
