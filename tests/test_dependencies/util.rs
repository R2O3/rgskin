#![allow(clippy::explicit_write, unused)]

use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use std::str;
use std::time::Instant;

pub fn read_file_to_string(file_path: &str) -> io::Result<String> {
    let mut file = fs::File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(String::from_utf8_lossy(&contents).to_string())
}

pub fn write_to_file(file_path: &str, content: &str) -> std::io::Result<()> {
    let path = Path::new(file_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let mut file = fs::File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    println!("File written successfully to {}", file_path);
    Ok(())
}

pub fn read_file_to_bytes(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = fs::File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

pub fn write_bytes_to_file(file_path: &str, content: &[u8]) -> io::Result<()> {
    let path = Path::new(file_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let mut file = fs::File::create(file_path)?;
    file.write_all(content)?;
    println!("File written successfully to {}", file_path);
    Ok(())
}

pub fn sanitize_str(filename: &str) -> String {
    const FORBIDDEN_CHARS: &[char] = &['/', '\\', '?', '%', '*', ':', '|', '"', '<', '>', '\0', '\n'];
    
    let mut sanitized = String::with_capacity(filename.len());
    
    for ch in filename.chars() {
        if FORBIDDEN_CHARS.contains(&ch) {
            sanitized.push('_');
        } else if ch.is_control() {
            sanitized.push('_');
        } else {
            sanitized.push(ch);
        }
    }
    
    sanitized = sanitized.trim_matches(|c| c == '.' || c == ' ').to_string();
    
    if sanitized.is_empty() {
        sanitized = "untitled".to_string();
    }
    
    if sanitized.len() > 255 {
        sanitized.truncate(255);
    }
    
    sanitized
}

#[inline]
pub fn println_test(func_name: &str, color_code: &str, message: &str) {
    writeln!(io::stdout(), "{} || {}{}\x1b[0m", func_name, color_code, message).unwrap();
}

#[inline]
pub fn benchmark_closure<F>(mut f: F, func_name: &str, to_what: &str, color_code: &str) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut() -> Result<(), Box<dyn std::error::Error>>,
{
    let time = Instant::now();
    f()?;
    let duration = time.elapsed();
    println_test(func_name, color_code, &format!("took {:?} {to_what}", duration));
    Ok(())
}