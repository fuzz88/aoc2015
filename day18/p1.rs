use std::env;
use std::fs;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_filename>", args[0]);
        return;
    }
    let input_filename = &args[1];
    match read_grid_from_file(input_filename) {
        Ok(grid) => println!("Grid loaded successfully: {:?}", grid),
        Err(e) => eprintln!("Error reading grid: {}", e),
    }
}

fn read_grid_from_file(filename: &str) -> io::Result<Vec<Vec<bool>>> {
    let content = fs::read_to_string(filename)?;
    let grid = content
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();
    Ok(grid)
}