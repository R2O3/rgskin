use std::collections::HashMap;

use tinyjson::JsonValue;

use crate::skin::fluxis::layout_json::{component::Component, gameplay::*};

#[derive(Debug, Clone)]
pub struct FluXisLayout {
    pub name: String,
    pub author: String,
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
        let json: JsonValue = json_str.parse()
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;
        
        Self::from_json(&json)
    }

    pub fn from_json(json: &JsonValue) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = json.get::<HashMap<String, JsonValue>>()
            .ok_or("Expected JSON object at root")?;
        
        let name = obj.get("name")
            .and_then(|v| v.get::<String>())
            .ok_or("Missing or invalid 'name' field")?
            .clone();
        
        let author = obj.get("author")
            .and_then(|v| v.get::<String>())
            .ok_or("Missing or invalid 'author' field")?
            .clone();
        
        let gameplay = obj.get("gameplay")
            .ok_or("Missing 'gameplay' field")
            .and_then(|v| Gameplay::from_json(v))?;
        
        Ok(Self {
            name,
            author,
            gameplay,
        })
    }

    pub fn to_str(&self) -> String {
        self.to_json().stringify().unwrap()
    }
    
    pub fn to_json(&self) -> JsonValue {
        let mut map = std::collections::HashMap::new();
        
        map.insert("name".to_string(), JsonValue::String(self.name.clone()));
        map.insert("author".to_string(), JsonValue::String(self.author.clone()));
        
        map.insert("gameplay".to_string(), self.gameplay.to_json());
        
        JsonValue::Object(map)
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
