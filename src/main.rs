use std::fs::{read_dir, DirEntry};
use std::error::Error;
use std::path::Path;
use std::io::ErrorKind;

fn main() {
    let example_dir = "./example";
    let entries = walk_dir(Path::new(example_dir));
    match entries {
        Ok(entries) => {
            for e in entries.iter() {
                println!("Found entry: {}", e.file_name().to_str().unwrap());
            }
        }
        Err(e) => {
            println!("Failed to walk directory: {}", e);
        }
    }
}

fn walk_dir(path: &Path) -> Result<Vec<DirEntry>, std::io::Error> {
    let dir = read_dir(path)?;
    let mut entries = Vec::new();
    for entry in dir {
        let entry = entry?;
        if (entry.path().is_dir()) {
            let mut rec = walk_dir(&entry.path())?;
            entries.append( &mut rec);
        }
        entries.push(entry);
    }

    return Ok(entries);
}
