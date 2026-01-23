#[inline]
pub fn parse_u16_list(value: &str) -> Vec<u16> {
    value.split(',')
            .map(|s| s.trim().parse().unwrap_or(0))
            .collect()
}

#[inline]
pub fn parse_u32_list(value: &str) -> Vec<u32> {
    value.split(',')
            .map(|s| s.trim().parse().unwrap_or(0))
            .collect()
}

#[inline]
pub fn parse_f32_list(value: &str) -> Vec<f32> {
    value.split(',')
            .map(|s| s.trim().parse().unwrap_or(0.0))
            .collect()
}

#[inline]
pub fn parse_bool(value: &str) -> bool {
    value == "1"
}

#[inline]
pub fn parse_key_value(raw_str: &str) -> Option<(&str, &str)> {
    if let Some(colon_pos) = raw_str.find(':') {
        Some((raw_str[..colon_pos].trim(), raw_str[colon_pos + 1..].trim()))
    } else {
        None
    }
}

#[inline]
pub fn serialize_bool(value: bool) -> u8 {
    if value { 1 } else { 0 }
}

#[inline]
pub fn serialize_u16_slice(values: &[u16]) -> String {
    values.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",")
}

#[inline]
pub fn serialize_u32_slice(values: &[u32]) -> String {
    values.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",")
}

#[inline]
pub fn serialize_f32_slice(values: &[f32]) -> String {
    values.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",")
}

#[inline]
pub fn serialize_vec_if_not_empty<T: ToString>(result: &mut String, vec: &[T], key: &str) {
    if !vec.is_empty() {
        let str_val = vec.iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");
        add_key_value(result, key, ": ", &str_val, "\n");
    }
}

#[inline]
pub fn serialize_bool_vec_if_not_empty(result: &mut String, vec: &[bool], key: &str) {
    if !vec.is_empty() {
        let bool_str = vec.iter()
            .map(|&b| if b { "1" } else { "0" })
            .collect::<Vec<_>>()
            .join(",");
        add_key_value(result, key, ": ", &bool_str, "\n");
    }
}

#[inline]
pub fn add_key_value(template: &mut String, key: &str, sep: &str, value: &str, end: &str) {
    template.reserve(key.len() + value.len() + 3);
    template.push_str(key);
    template.push_str(sep);
    template.push_str(value);
    template.push_str(end);
}

#[inline]
pub fn add_key_value_if_not_default<T>(
    result: &mut String, 
    key: &str, 
    value: &T, 
    default_value: &T
) 
where
    T: PartialEq + std::fmt::Display,
{
    if value != default_value {
        let value_str = value.to_string();
        if !value_str.is_empty() {
            add_key_value(result, key, ": ", &value_str, "\n");
        }
    }
}

pub fn set_vec_element(vec: &mut Vec<String>, idx: usize, value: &str) {
    if idx < vec.len() {
        vec[idx] = value.to_string();
    }
}
