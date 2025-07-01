use std::{
    fs::{read_dir, File},
    io::{BufRead, BufReader},
    path::Path,
};

use crate::model::ClassifiedWallet;

pub fn load_jsonl(folder: &str) -> Vec<ClassifiedWallet> {
    let mut data = Vec::new();
    let path = Path::new(folder);

    for entry in read_dir(path).expect("Unable to read input folder") {
        let entry = entry.expect("Invalid file");
        let file_path = entry.path();
        if file_path.extension().and_then(|s| s.to_str()) != Some("jsonl") {
            continue;
        }

        let file = File::open(&file_path).expect("Failed to open file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            match serde_json::from_str::<ClassifiedWallet>(&line) {
                Ok(data_row) => data.push(data_row),
                Err(e) => eprintln!("Invalid JSON line: {}", e),
            }
        }
    }

    data
}
