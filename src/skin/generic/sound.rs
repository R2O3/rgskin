use merge::Merge;

use crate::utils;
use rgskin_derive::merge_for_all;

#[merge_for_all(strategy = crate::utils::merge::any::overwrite)]
#[derive(Clone, Merge)]
pub struct Sounds {
    pub ui: UISounds,
    pub gameplay: GenericGameplaySounds,
    pub mania: ManiaGameplaySounds
}

#[merge_for_all(strategy = crate::utils::merge::any::overwrite)]
#[derive(Clone, Merge, Debug)]
pub struct UISounds {
    pub menu_back_click: Option<String>,
    pub ui_click: Option<String>,
    pub ui_select: Option<String>,
    pub ui_hover: Option<String>,
}

#[merge_for_all(strategy = crate::utils::merge::any::overwrite)]
#[derive(Clone, Merge)]
pub struct ManiaGameplaySounds {
    pub hit: Option<String>,
}

#[merge_for_all(strategy = crate::utils::merge::any::overwrite)]
#[derive(Clone, Merge)]
pub struct GenericGameplaySounds {
    pub miss: Option<String>,
    pub fail: Option<String>,
    pub restart: Option<String>
}
