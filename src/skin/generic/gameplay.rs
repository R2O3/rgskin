use crate::generic::{elements::{Healthbar, Judgement, Stage}, layout::HUDLayout};

#[derive(Clone)]
pub struct Gameplay {
    pub health_bar: Healthbar,
    pub stage: Stage,
    pub judgement: Judgement,
    pub layout: HUDLayout
}