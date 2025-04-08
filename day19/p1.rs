use std::env;
use std::fs;
use std::io;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("ERROR: no input");
        return;
    }

    let input_file = &args[1];
    println!("{}", input_file);
}
