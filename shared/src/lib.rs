use std::{fs, io};

pub fn read_file(filepath: &str) -> Result<Vec<String>, io::Error> {
    let s = fs::read_to_string(filepath)?;
    Ok(s.lines().map(|x| x.to_string()).collect())
}