use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Output};
use tempfile::NamedTempFile;

pub fn interpret_temp_file_with_content(content: &str) -> io::Result<Output> {
    let mut temp_file = NamedTempFile::new()?;
    write!(temp_file, "{}", content)?;

    let filename = temp_file.path();
    let output = Command::new("bash")
        .arg("your_program.sh")
        .arg(filename)
        .output()?;

    Ok(output)
}

#[allow(dead_code)]
pub fn read_temp_file(filename: &Path) -> io::Result<String> {
    let contents = fs::read_to_string(filename)?;
    Ok(contents)
}
