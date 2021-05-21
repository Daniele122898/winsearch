use std::fs::{read_dir, DirEntry};
use std::error::Error;
use std::path::Path;
use std::io::ErrorKind;

fn main() {
    let example_dir = "./example";
    let entries = walk_dir(Path::new(example_dir)).expect("Failed to walk dir");

    for e in entries.iter() {
        println!("Found entry: {}", e.file_name().to_str().unwrap());
    }

    let res = sliding_window("astring".to_string(), 3).expect("Failed to split string");
    println!("Sliding window result:");
    for x in res.iter() {
        println!("{}", x);
    }
}

fn sliding_window(seq: String, n: usize) -> Option<Vec<String>> {
    if seq.len() == 0 {
        return None;
    }

    let mut split: Vec<String> = Vec::new();
    for i in 0..(seq.len() - n +1) {
        // We actually want to copy here bcs we dont know what's gonna happen to the split string
        let sub = (&seq[i..i+n]).to_string();
        split.push(sub);
    }

    return Some(split);
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
