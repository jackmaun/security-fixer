use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::error::Error;

pub fn apply_fix(file_path: &str, line_number: usize, new_code: &str) -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();
    fs::File::open(file_path)?.read_to_string(&mut contents)?;

    let mut lines: Vec<&str> = contents.lines().collect();
    let original_code = lines[line_number - 1].to_string();

    if line_number > 0 && line_number <= lines.len() {
        lines[line_number - 1] = new_code;
    }

    let mut file = OpenOptions::new().write(true).truncate(true).open(file_path)?;
    file.write_all(lines.join("\n").as_bytes())?;

    println!("\n[-] Original Code:\n{}", original_code);
    println!("[+] Fixed Code:\n{}", new_code);

    Ok(())
}
