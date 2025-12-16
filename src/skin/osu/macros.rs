
#[macro_export]
macro_rules! add_section {
    ($result:expr, $keymode:expr, $section_name:expr, $section_body:expr) => {{
        let mut section = String::new();
        ($section_body)(&mut section);
        if !section.is_empty() {
            $result.push_str(&format!("// --= {} | {}k =--\n", $section_name, $keymode));
            $result.push_str(&section);
            $result.push('\n');
        }
    }};
}