use std::cmp::{Ordering};
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

fn step_by_container_min(
    solution: Vec<usize>,
    containers: &Vec<u32>,
    current_index: usize,
    total_volume: u32,
    min_len: &mut usize,
    compensator: &mut i32,
) -> i32 {
    let mut count: i32 = 0;
    for index in current_index..containers.len() {
        if !solution.contains(&index) {
            let mut solution = solution.clone();
            solution.push(index);
            let volume: u32 = solution.iter().map(|index| containers[*index]).sum();
            count = count
                + match volume.cmp(&total_volume) {
                    Ordering::Greater => 0,
                    Ordering::Equal => {
                        let mut compensate = false;
                        if *min_len > solution.len() {
                            if *min_len != usize::MAX {
                                compensate = true;
                            }
                            *min_len = solution.len();
                        }
                        if solution.len() == *min_len {
                            if compensate {
                                let ret = *compensator;
                                *compensator = 1;
                                -(ret - 1)
                            } else {
                                *compensator += 1;
                                1
                            }
                        } else {
                            0
                        }
                    },
                    Ordering::Less => step_by_container_min(solution, containers, index, total_volume, min_len, compensator),
                };
        }
    }
    count
}

fn count_placements(containers: Vec<u32>, total_volume: u32) -> (u32, i32) {
    (
        step_by_container(Vec::new(), &containers, 0, total_volume),
        step_by_container_min(Vec::new(), &containers, 0, total_volume, &mut usize::MAX.clone(), &mut 0.clone()),
    )
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 3 {
        let input_file = &args[1];
        let input = read_input(input_file).unwrap();
        let volume: u32 = args[2].parse().unwrap();

        println!("{}", input.len());
        println!("{}", volume);

        println!("{:?}", count_placements(input, volume));
    } else {
        println!("ERROR: no input params");
    }
}
