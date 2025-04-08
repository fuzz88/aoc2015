use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("ERROR: no input");
        return;
    }

    let input_file = &args[1];
    let input = fs::read_to_string(input_file).expect("Failed to read input file");

    let (replacements, medicine_molecule) = parse_input(&input);

    // Part 1: Distinct molecules after one replacement
    let distinct_molecules = find_distinct_molecules(&replacements, &medicine_molecule);
    println!("Distinct molecules: {}", distinct_molecules.len());

    // Part 2: Fewest steps to fabricate the medicine molecule
    let steps = find_fewest_steps(&replacements, &medicine_molecule);
    println!("Fewest steps to fabricate the medicine molecule: {}", steps);
}

fn parse_input(input: &str) -> (Vec<(String, String)>, String) {
    let mut replacements = Vec::new();
    let mut medicine_molecule = String::new();

    for line in input.lines() {
        if line.contains("=>") {
            let parts: Vec<&str> = line.split(" => ").collect();
            replacements.push((parts[0].to_string(), parts[1].to_string()));
        } else if !line.trim().is_empty() {
            medicine_molecule = line.trim().to_string();
        }
    }

    (replacements, medicine_molecule)
}

fn find_distinct_molecules(replacements: &[(String, String)], molecule: &str) -> HashSet<String> {
    let mut distinct_molecules = HashSet::new();

    for (from, to) in replacements {
        let mut start = 0;
        while let Some(pos) = molecule[start..].find(from) {
            let pos = start + pos;
            let mut new_molecule = molecule.to_string();
            new_molecule.replace_range(pos..pos + from.len(), to);
            distinct_molecules.insert(new_molecule);
            start = pos + 1;
        }
    }

    distinct_molecules
}

fn find_fewest_steps(replacements: &[(String, String)], target: &str) -> usize {
    let mut reverse_replacements: Vec<(String, String)> = replacements
        .iter()
        .map(|(from, to)| (to.clone(), from.clone()))
        .collect();

    // Sort by descending length of the "to" string to prioritize larger replacements
    reverse_replacements.sort_by_key(|(to, _)| -(to.len() as isize));

    let mut steps = 0;
    let mut current_molecule = target.to_string();

    while current_molecule != "e" {
        let mut replaced = false;

        for (to, from) in &reverse_replacements {
            if let Some(pos) = current_molecule.find(to) {
                current_molecule.replace_range(pos..pos + to.len(), from);
                steps += 1;
                replaced = true;
                break;
            }
        }

        if !replaced {
            panic!("No valid replacement found to reduce the molecule further.");
        }
    }

    steps
}
