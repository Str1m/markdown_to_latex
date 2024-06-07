use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub fn read_file_to_string<P: AsRef<Path>>(path: P) -> io::Result<String>{
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}