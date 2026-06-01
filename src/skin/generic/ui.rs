use merge::Merge;
use rgskin_derive::merge_for_all;
use crate::generic::elements::Cursor;

#[merge_for_all(strategy = crate::utils::merge::any::overwrite)]
#[derive(Clone, Merge)]
pub struct UI {
    pub cursor: Cursor,
}