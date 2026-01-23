use merge::Merge;

use crate::{derive_merge_for_all, generic::elements::Cursor};

derive_merge_for_all! {
    strategy = crate::utils::merge::any::overwrite;
    #[derive(Clone, Merge)]
    pub struct UI {
        pub cursor: Cursor,
    }
}