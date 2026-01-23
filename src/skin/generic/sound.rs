use merge::Merge;

use crate::{derive_merge_for_all, utils};

#[derive(Clone, Merge)]
pub struct Sounds {
    pub ui: UISounds,
    pub gameplay: GenericGameplaySounds,
    pub mania: ManiaGameplaySounds
}

derive_merge_for_all! {
    strategy = crate::utils::merge::any::overwrite;
    #[derive(Clone, Merge, Debug)]
    pub struct UISounds {
        pub menu_back_click: Option<String>,
        pub ui_click: Option<String>,
        pub ui_select: Option<String>,
        pub ui_hover: Option<String>,
    }
}

derive_merge_for_all! {
    strategy = crate::utils::merge::any::overwrite;
    #[derive(Clone, Merge)]
    pub struct ManiaGameplaySounds {
        pub hit: Option<String>,
    }
}

derive_merge_for_all! {
    strategy = crate::utils::merge::any::overwrite;
    #[derive(Clone, Merge)]
    pub struct GenericGameplaySounds {
        pub miss: Option<String>,
        pub fail: Option<String>,
        pub restart: Option<String>
    }
}
