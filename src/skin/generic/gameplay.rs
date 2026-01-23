use merge::Merge;

use crate::{derive_merge_for_all, generic::{elements::{Healthbar, Judgement, Stage}, layout::HUDLayout}};


derive_merge_for_all! {
    strategy = crate::utils::merge::skin_element::overwrite_if_data;
    #[derive(Clone, Merge)]
    pub struct Gameplay {
        pub health_bar: Healthbar,
        pub stage: Stage,
        pub judgement: Judgement,
        #[merge(skip)]
        pub layout: HUDLayout
    }
}