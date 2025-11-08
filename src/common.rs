#![allow(unused)]

use std::ops::BitOr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Anchor {
    TopLeft = Self::Y0 as u8 | Self::X0 as u8,
    TopCentre = Self::Y0 as u8 | Self::X1 as u8,
    TopRight = Self::Y0 as u8 | Self::X2 as u8,
    CentreLeft = Self::Y1 as u8 | Self::X0 as u8,
    Centre = Self::Y1 as u8 | Self::X1 as u8,
    CentreRight = Self::Y1 as u8 | Self::X2 as u8,
    BottomLeft = Self::Y2 as u8 | Self::X0 as u8,
    BottomCentre = Self::Y2 as u8 | Self::X1 as u8,
    BottomRight = Self::Y2 as u8 | Self::X2 as u8,
    
    Y0 = 1,
    Y1 = 1 << 1,
    Y2 = 1 << 2,
    X0 = 1 << 3,
    X1 = 1 << 4,
    X2 = 1 << 5,
    Custom = 1 << 6,
}

impl Anchor {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
    
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Y0),
            2 => Some(Self::Y1),
            4 => Some(Self::Y2),
            8 => Some(Self::X0),
            16 => Some(Self::X1),
            32 => Some(Self::X2),
            64 => Some(Self::Custom),
            9 => Some(Self::TopLeft),
            17 => Some(Self::TopCentre),
            33 => Some(Self::TopRight),
            10 => Some(Self::CentreLeft),
            18 => Some(Self::Centre),
            34 => Some(Self::CentreRight),
            12 => Some(Self::BottomLeft),
            20 => Some(Self::BottomCentre),
            36 => Some(Self::BottomRight),
            _ => None,
        }
    }
}

impl Default for Anchor {
    fn default() -> Self {
        Anchor::Centre
    }
}

impl TryFrom<u8> for Anchor {
    type Error = ();
    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(())
    }
}

impl BitOr for Anchor {
    type Output = u8;
    
    fn bitor(self, rhs: Self) -> Self::Output {
        (self as u8) | (rhs as u8)
    }
}

pub type Origin = Anchor;

pub struct Alignment  {
    pub anchor: Anchor,
    pub origin: Origin,
}

impl Alignment {
    pub fn new(anchor: Anchor, origin: Origin) -> Self {
        Self { anchor, origin }
    }
    
    fn anchor_offset(anchor: Anchor) -> (f32, f32) {
        let anchor_bits = anchor.as_u8();
        let x_mask = Anchor::X0.as_u8() | Anchor::X1.as_u8() | Anchor::X2.as_u8();
        let y_mask = Anchor::Y0.as_u8() | Anchor::Y1.as_u8() | Anchor::Y2.as_u8();
        
        let x = match anchor_bits & x_mask {
            x if x == Anchor::X0.as_u8() => 0.0,
            x if x == Anchor::X1.as_u8() => 0.5,
            x if x == Anchor::X2.as_u8() => 1.0,
            _ => 0.5,
        };
        
        let y = match anchor_bits & y_mask {
            y if y == Anchor::Y0.as_u8() => 0.0,
            y if y == Anchor::Y1.as_u8() => 0.5,
            y if y == Anchor::Y2.as_u8() => 1.0,
            _ => 0.5,
        };
        
        (x, y)
    }
    
    pub fn convert_pos(
        position: Vector2<f32>,
        size: Vector2<f32>,
        original_alignment: &Alignment,
        target_alignment: &Alignment,
    ) -> Vector2<f32> {
        let (orig_anchor_x_mult, orig_anchor_y_mult) = Self::anchor_offset(original_alignment.anchor);
        let (orig_origin_x_mult, orig_origin_y_mult) = Self::anchor_offset(original_alignment.origin);
        
        let origin_offset_x = size.x * orig_origin_x_mult;
        let origin_offset_y = size.y * orig_origin_y_mult;
        
        let top_left_x = position.x - origin_offset_x;
        let top_left_y = position.y - origin_offset_y;
        
        let (target_origin_x_mult, target_origin_y_mult) = Self::anchor_offset(target_alignment.origin);
        
        let target_x = top_left_x + (size.x * target_origin_x_mult);
        let target_y = top_left_y + (size.y * target_origin_y_mult);
        
        Vector2::new(target_x, target_y)
    }
    
    pub fn calculate_pos(
        container_size: Vector2<f32>,
        target_size: Vector2<f32>,
        target_alignment: &Alignment,
    ) -> Vector2<f32> {
        let (anchor_x_mult, anchor_y_mult) = Self::anchor_offset(target_alignment.anchor);
        let anchor_x = container_size.x * anchor_x_mult;
        let anchor_y = container_size.y * anchor_y_mult;
        
        let (origin_x_mult, origin_y_mult) = Self::anchor_offset(target_alignment.origin);
        let origin_offset_x = target_size.x * origin_x_mult;
        let origin_offset_y = target_size.y * origin_y_mult;
        
        Vector2::new(
            anchor_x - origin_offset_x,
            anchor_y - origin_offset_y,
        )
    }
    
    pub fn calculate_pos_offsetted(
        container_size: Vector2<f32>,
        target_size: Vector2<f32>,
        target_alignment: &Alignment,
        offset: Vector2<f32>,
    ) -> Vector2<f32> {
        let base_position = Self::calculate_pos(container_size, target_size, target_alignment);
        Vector2::new(
            base_position.x + offset.x,
            base_position.y + offset.y,
        )
    }
}

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
            format!("#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
        } else {
            format!("#{:02x}{:02x}{:02x}{:02x}", self.red, self.green, self.blue, self.alpha)
        }
    }

    pub fn to_tuple(&self) -> (u8, u8, u8, u8) {
        (self.red, self.green, self.blue, self.alpha)
    }
    
    pub fn to_str(&self) -> String {
        format!("{},{},{},{}", self.red, self.green, self.blue, self.alpha)
    }
}