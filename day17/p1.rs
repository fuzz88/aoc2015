use std::fs;
use std::io;
use std::env;

fn read_input(file_path: &str) -> Result<Vec<u32>, io::Error> {
    let input = fs::read_to_string(file_path)?
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<u32>>();

    Ok(input)
}

fn main() {

    let args: Vec<_> = env::args().collect();
    if args.len() == 3 {
        let input_file = &args[1];
        let input = read_input(input_file).unwrap();
    
        println!("Containers count: {}", input.len());
        println!("Liters of eggnog: {}", args[2]);
    } else {
        println!("ERROR: no input params");
    }

}
