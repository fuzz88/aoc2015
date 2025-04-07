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

fn step_by_container<F>(
    solution: &[usize],
    containers: &[u32],
    current_index: usize,
    total_volume: u32,
    on_valid: &mut F,
) -> u32
where
    F: FnMut(&[usize]) -> u32,
{
    let mut count = 0;

    for index in current_index..containers.len() {
        if solution.contains(&index) {
            continue;
        }

        let mut new_solution = solution.to_vec();
        new_solution.push(index);

        let volume: u32 = new_solution.iter().map(|&i| containers[i]).sum();

        count += match volume.cmp(&total_volume) {
            Ordering::Greater => 0,
            Ordering::Equal => on_valid(&new_solution),
            Ordering::Less => step_by_container(&new_solution, containers, index, total_volume, on_valid),
        };
    }

    count
}

fn count_placements(containers: &[u32], total_volume: u32) -> (u32, i32) {
    let total_count = step_by_container(&[], containers, 0, total_volume, &mut |_| 1);

    let mut min_len = usize::MAX;
    let mut min_count = 0;

    let mut min_count_fn = |solution: &[usize]| -> u32 {
        match solution.len().cmp(&min_len) {
            Ordering::Less => {
                min_len = solution.len();
                min_count = 1;
            },
            Ordering::Equal => {
                min_count += 1;
            },
            _ => {}
        }
        1
    };

    step_by_container(&[], containers, 0, total_volume, &mut min_count_fn);

    (total_count, min_count)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <total_volume>", args[0]);
        return;
    }

    let input_file = &args[1];
    let total_volume: u32 = args[2].parse().expect("Invalid total_volume");

    let input = read_input(input_file).expect("Failed to read input");

    let (total, minimal) = count_placements(&input, total_volume);
    println!("Total combinations: {}\nMinimum container combinations: {}", total, minimal);
}
