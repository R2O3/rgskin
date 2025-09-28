#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rgba {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Rgba {
    pub fn from_str(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 4 {
            return Err(format!("expected 4 comma-separated values, got {}", parts.len()));
        }
        
        let parse_component = |s: &str| -> Result<u8, String> {
            s.trim()
                .parse::<u8>()
                .map_err(|_| format!("Invalid u8 value: '{}'", s))
        };
        
        Ok(Rgba {
            red: parse_component(parts[0])?,
            green: parse_component(parts[1])?,
            blue: parse_component(parts[2])?,
            alpha: parse_component(parts[3])?,
        })
    }

    pub fn to_tuple(&self) -> (u8, u8, u8, u8) {
        (self.red, self.green, self.blue, self.alpha)
    }
    
    pub fn to_str(&self) -> String {
        format!("{},{},{},{}", self.red, self.green, self.blue, self.alpha)
    }
}