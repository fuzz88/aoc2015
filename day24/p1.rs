use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let weights: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();
    let total_weight: u64 = weights.iter().sum();
    let group_weight = total_weight / 3;

    let mut best_qe = u64::MAX;
    let mut min_group_size = usize::MAX;

    for group_size in 1..weights.len() {
        let mut found = false;
        for group in combinations(&weights, group_size) {
            if group.iter().sum::<u64>() == group_weight {
                let remaining: Vec<u64> = weights.iter().copied().filter(|w| !group.contains(w)).collect();
                if can_split_into_two_groups(&remaining, group_weight) {
                    found = true;
                    let qe: u64 = group.iter().product();
                    if qe < best_qe {
                        best_qe = qe;
                    }
                }
            }
        }
        if found {
            min_group_size = group_size;
            break;
        }
    }

    println!("Quantum Entanglement: {}", best_qe);
}

fn can_split_into_two_groups(weights: &[u64], target_weight: u64) -> bool {
    for group_size in 1..=weights.len() {
        for group in combinations(weights, group_size) {
            if group.iter().sum::<u64>() == target_weight {
                return true;
            }
        }
    }
    false
}

fn combinations<T: Clone>(items: &[T], k: usize) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    generate_combinations(items, k, 0, &mut current, &mut result);
    result
}

fn generate_combinations<T: Clone>(
    items: &[T],
    k: usize,
    start: usize,
    current: &mut Vec<T>,
    result: &mut Vec<Vec<T>>,
) {
    if current.len() == k {
        result.push(current.clone());
        return;
    }
    for i in start..items.len() {
        current.push(items[i].clone());
        generate_combinations(items, k, i + 1, current, result);
        current.pop();
    }
}
