use std::fs;
use std::io::{self, Write};
use std::path::Path;
use tempfile::NamedTempFile;

pub fn create_temp_file_with_content(content: &str) -> io::Result<NamedTempFile> {
    let mut temp_file = NamedTempFile::new()?;
    write!(temp_file, "{}", content)?;
    Ok(temp_file)
}

#[allow(dead_code)]
pub fn read_temp_file(filename: &Path) -> io::Result<String> {
    let contents = fs::read_to_string(filename)?;
    Ok(contents)
}
