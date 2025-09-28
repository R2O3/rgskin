pub fn from_ini<F>(string: &str, mut lambda: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(&str, &str) -> Result<(), Box<dyn std::error::Error>>,
{
    let mut current_content = String::with_capacity(string.len());
    let mut current_section = "";
    
    for line in string.lines().map(str::trim) {
        if line.starts_with("//") || line.starts_with(";") { continue };

        if line.starts_with('[') && line.ends_with(']') {
            if !current_content.is_empty() && !current_section.is_empty() {
                lambda(current_section, &current_content)?;
                current_content.clear();
            }
            current_section = &line[1..line.len()-1];
        } else {
            if !current_content.is_empty() {
                current_content.push('\n');
            }
            current_content.push_str(line);
        }
    }
    
    if !current_content.is_empty() && !current_section.is_empty() {
        lambda(current_section, &current_content)?;
    }
    
    Ok(())
}