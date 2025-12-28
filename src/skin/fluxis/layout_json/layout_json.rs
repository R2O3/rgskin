#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::skin::fluxis::layout_json::{component::Component, gameplay::*};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluXisLayout {
    #[serde(rename = "Name")]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub name: String,
    #[serde(rename = "Author")]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
    pub author: String,
    #[serde(rename = "Gameplay")]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))] // Can't be bothered to
    pub gameplay: Gameplay,
}

impl FluXisLayout {
    pub fn new(name: String, author: String) -> Self {
        Self {
            name,
            author,
            gameplay: Gameplay {
                components: HashMap::new(),
            },
        }
    }
    
    pub fn add_component_to_gameplay(&mut self, name: String, component: Component) {
        self.gameplay.components.insert(name, component);
    }

    pub fn from_str(json_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let layout: FluXisLayout = serde_json::from_str(json_str)?;
        Ok(layout)
    }

    pub fn to_str(&self) -> Result<String, Box<dyn std::error::Error>> {
        let json_str = serde_json::to_string_pretty(self)?;
        Ok(json_str)
    }
}

impl FluXisLayout {
    pub fn add_accuracy(&mut self, name: String, accuracy: Accuracy) {
        self.gameplay.components.insert(name, accuracy.component);
    }
    
    pub fn add_attribute_text(&mut self, name: String, attr_text: AttributeText) {
        self.gameplay.components.insert(name, attr_text.component);
    }
    
    pub fn add_combo(&mut self, name: String, combo: Combo) {
        self.gameplay.components.insert(name, combo.component);
    }
    
    pub fn add_health(&mut self, name: String, health: Health) {
        self.gameplay.components.insert(name, health.component);
    }
    
    pub fn add_hit_error(&mut self, name: String, hit_error: HitError) {
        self.gameplay.components.insert(name, hit_error.component);
    }
    
    pub fn add_judgement(&mut self, name: String, judgement: Judgement) {
        self.gameplay.components.insert(name, judgement.component);
    }
    
    pub fn add_judgement_counter(&mut self, name: String, counter: JudgementCounter) {
        self.gameplay.components.insert(name, counter.component);
    }
    
    pub fn add_progress(&mut self, name: String, progress: Progress) {
        self.gameplay.components.insert(name, progress.component);
    }

    pub fn add_performance_rating(&mut self, name: String, perf_rating: PerformanceRating) {
        self.gameplay.components.insert(name, perf_rating.component);
    }
    
    pub fn add_keys_per_second(&mut self, name: String, kps: KeysPerSecond) {
        self.gameplay.components.insert(name, kps.component);
    }
}

impl Default for FluXisLayout {
    fn default() -> Self {
        let mut layout = FluXisLayout::new("Layout".to_string(), "Unknown".to_string());
        
        layout.add_accuracy("Accuracy".to_string(), Accuracy::default());
        
        layout.add_attribute_text("AttributeText#Title".to_string(), AttributeText::title());
        layout.add_attribute_text("AttributeText#Artist".to_string(), AttributeText::artist());
        layout.add_attribute_text("AttributeText#Difficulty".to_string(), AttributeText::difficulty());
        layout.add_attribute_text("AttributeText#Mapper".to_string(), AttributeText::mapper());
        
        layout.add_combo("Combo".to_string(), Combo::default());
        layout.add_performance_rating("PerformanceRating".to_string(), PerformanceRating::default());
        layout.add_keys_per_second("KeysPerSecond".to_string(), KeysPerSecond::default());
        layout.add_health("Health".to_string(), Health::default());
        layout.add_hit_error("HitError".to_string(), HitError::default());
        layout.add_judgement("Judgement".to_string(), Judgement::default());
        layout.add_judgement_counter("JudgementCounter".to_string(), JudgementCounter::default());
        layout.add_progress("Progress".to_string(), Progress::default());
        
        layout
    }
}
