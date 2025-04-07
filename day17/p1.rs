use std::cmp::Ordering;
use std::env;
use std::fs;
use std::io;

fn read_input(file_path: &String) -> Result<Vec<u32>, io::Error> {
    let input = fs::read_to_string(file_path)?
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<u32>>();

    Ok(input)
}

fn step_by_container(
    solution: Vec<usize>,
    containers: &Vec<u32>,
    current_index: usize,
    total_volume: u32,
) -> u32 {
    let mut count: u32 = 0;
    for index in current_index..containers.len() {
        if !solution.contains(&index) {
            let mut solution = solution.clone();
            solution.push(index);
            let volume: u32 = solution.iter().map(|index| containers[*index]).sum();
            count = count
                + match volume.cmp(&total_volume) {
                    Ordering::Greater => 0,
                    Ordering::Equal => 1,
                    Ordering::Less => step_by_container(solution, containers, index, total_volume),
                };
        }
    }
    count
}

fn count_placements(containers: Vec<u32>, total_volume: u32) -> u32 {
    let solution: Vec<usize> = Vec::new();
    step_by_container(solution, &containers, 0, total_volume)
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 3 {
        let input_file = &args[1];
        let input = read_input(input_file).unwrap();
        let volume: u32 = args[2].parse().unwrap();

        println!("Containers count: {}", input.len());
        println!("Liters of eggnog: {}", volume);

        println!(
            "Different combination of containers: {}",
            count_placements(input, volume)
        );
    } else {
        println!("ERROR: no input params");
    }
}
