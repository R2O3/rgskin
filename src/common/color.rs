use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rgba {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Serialize for Rgba {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_hex())
    }
}

impl<'de> Deserialize<'de> for Rgba {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Rgba::from_hex(&s).map_err(serde::de::Error::custom)
    }
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

    pub fn from_hex(s: &str) -> Result<Self, String> {
        let s = s.trim().trim_start_matches('#');
        
        let bytes = match s.len() {
            6 => {
                let mut result = [0u8; 4];
                for i in 0..3 {
                    result[i] = u8::from_str_radix(&s[i*2..i*2+2], 16)
                        .map_err(|_| format!("Invalid hex digit in: '{}'", s))?;
                }
                result[3] = 255;
                result
            }
            8 => {
                let mut result = [0u8; 4];
                for i in 0..4 {
                    result[i] = u8::from_str_radix(&s[i*2..i*2+2], 16)
                        .map_err(|_| format!("Invalid hex digit in: '{}'", s))?;
                }
                result
            }
            _ => return Err(format!("expected 6 or 8 hex digits, got {}", s.len())),
        };
        
        Ok(Rgba {
            red: bytes[0],
            green: bytes[1],
            blue: bytes[2],
            alpha: bytes[3],
        })
    }

    pub fn to_hex(&self) -> String {
        if self.alpha == 255 {
            format!("#{:02x}{:02x}{:02x}", self.red, self.green, self.blue).to_uppercase()
        } else {
            format!("#{:02x}{:02x}{:02x}{:02x}", self.red, self.green, self.blue, self.alpha).to_uppercase()
        }
    }

    pub fn to_tuple(&self) -> (u8, u8, u8, u8) {
        (self.red, self.green, self.blue, self.alpha)
    }
    
    pub fn to_str(&self) -> String {
        format!("{},{},{},{}", self.red, self.green, self.blue, self.alpha).to_uppercase()
    }
}
