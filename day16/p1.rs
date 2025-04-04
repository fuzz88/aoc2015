use std::collections::HashMap;
use std::fs;
use std::io;

fn read_input(file_path: &str) -> Result<Vec<HashMap<String, u32>>, io::Error> {
    let mut aunts = Vec::new();

    for line in fs::read_to_string(file_path)?.lines() {
        // "Sue 1: cars: 9, akitas: 3, goldfish: 0"
        let mut aunt = HashMap::<String, u32>::new();
        let mut parts = line.split_whitespace();
        _ = parts.next(); // skipping Sue
        _ = parts.next(); // skipping numero
        loop {
            if let (Some(compound), Some(kinds)) = (parts.next(), parts.next()) {
                let mut compound = compound.to_string();
                compound.pop();
                let kinds: u32 = kinds.trim_end_matches(",").parse().unwrap();
                aunt.insert(compound, kinds);
            } else {
                aunts.push(aunt);
                break;
            };
        }
    }

    Ok(aunts)
}

fn read_fingerprint(fingerprint_path: &str) -> Result<HashMap<String, u32>, io::Error> {
    let mut fingerprint = HashMap::new();
    for line in fs::read_to_string(fingerprint_path)?.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(compound), Some(kinds)) = (parts.next(), parts.next()) {
            if let Ok(kinds) = kinds.parse::<u32>() {
                fingerprint.insert(compound.to_string(), kinds);
            }
        }
    }

    Ok(fingerprint)
}

fn main() {
    let fingerprint_path = "fingerprint.txt";
    let fingerprint = read_fingerprint(fingerprint_path).unwrap();

    println!("{:#?}", fingerprint);

    let file_path = "input.txt";
    let input = read_input(file_path).unwrap();

    println!("{:#?}", input);
    println!("Sue count: {}", input.len());
}
