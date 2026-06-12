use merge::Merge;
use rgskin_derive::{GetAllTextures, merge_for_all};
use crate::generic::elements::Cursor;

#[merge_for_all(strategy = crate::utils::merge::any::overwrite)]
#[derive(Clone, Merge, GetAllTextures)]
pub struct UI {
    pub cursor: Cursor,
}