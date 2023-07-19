use std::path::Path;
use std::{
    fs,
    io::{self},
    process,
};

pub fn is_hidden<P>(file_path: P) -> Option<bool>
where
    P: AsRef<Path>,
{
    let filename = file_path.as_ref().file_name()?.to_str()?;
    if filename.starts_with(".") {
        return Some(true);
    } else {
        return Some(false);
    }
}

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
            if is_hidden(&path) == Some(false) {
                let content = String::from_utf8_lossy(&fs::read(&path)?).into_owned();
                for _ in content.lines() {
                    total_lines += 1;
                }
            }
        } else if metadata.file_type().is_dir() {
            if is_hidden(&path) == Some(false) {
                let _linecount_result = linecount(Path::new(&path));
                let _linecount = match _linecount_result {
                    Ok(success) => success,
                    Err(err) => panic!("shit!{err}"),
                };

                total_lines += _linecount;
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
