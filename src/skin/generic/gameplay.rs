use crate::generic::{elements::{Healthbar, Stage}, layout::HUDLayout};

#[derive(Clone)]
pub struct Gameplay {
    pub health_bar: Healthbar,
    pub stage: Stage,
    pub layout: HUDLayout
}