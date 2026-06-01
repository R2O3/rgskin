use merge::Merge;
use rgskin_derive::merge_for_all;

use crate::{generic::{elements::{Healthbar, Judgement, Stage}, layout::HUDLayout}};


#[merge_for_all(strategy = crate::utils::merge::skin_element::overwrite_if_data)]
#[derive(Clone, Merge)]
pub struct Gameplay {
    pub health_bar: Healthbar, 
    
    pub judgement: Judgement,
    
    #[merge(skip)] 
    pub layout: HUDLayout,
}