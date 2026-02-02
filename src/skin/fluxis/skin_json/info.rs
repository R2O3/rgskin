use merge::Merge;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};

use crate::derive_merge_for_all;

derive_merge_for_all! {
    strategy = crate::utils::merge::any::overwrite;
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    #[derive(Clone, Debug, Serialize, Deserialize, Merge)]
    #[serde(default)]
    pub struct Info {
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
        pub name: String,
        
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
        pub creator: String,
        
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
        pub accent: String,
    }
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
