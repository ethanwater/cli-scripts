use std::path::Path;
use std::{
    fs,
    io::{self},
    process,
};

pub fn linecount<P>(directory_path: P) -> io::Result<u128>
where
    P: AsRef<Path>,
{
    let mut total_lines: u128 = 0;

    for entry in fs::read_dir(directory_path)? {
        let entry = entry?.path();
        let path = entry.as_path();

        let metadata = fs::metadata(path)?;
        if metadata.file_type().is_file() {
            for _line in fs::read_to_string(path).unwrap().lines() {
                total_lines += 1;
            }
        };
    }

    Ok(total_lines)
}

fn main() -> io::Result<()> {
    let output = process::Command::new("pwd").output()?;
    let mut current_dir = String::from_utf8_lossy(&output.stdout).into_owned();
    current_dir.pop();

    let result = linecount(Path::new(&current_dir))?;
    println!("linecount: {result}");
    Ok(())
}
