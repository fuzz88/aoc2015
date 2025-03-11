use std::fs;
use std::env;

fn parse_input(file: &str) -> Vec<Vec<i32>> {
    fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(|line| line.split_whitespace()
            .filter_map(|s| s.trim_end_matches(',').parse().ok())
            .collect::<Vec<_>>())
        .collect()
}

fn calculate_score(ingredients: &[Vec<i32>], amounts: &[i32]) -> (i32, i32) {
    let properties_count = ingredients[0].len() - 1;
    let (properties, calories) = ingredients.iter()
        .zip(amounts)
        .fold((vec![0; properties_count], 0), |(mut props, mut cal), (ingredient, &amount)| {
            props.iter_mut().zip(&ingredient[..properties_count]).for_each(|(p, &v)| *p += v * amount);
            cal += ingredient[properties_count] * amount;
            (props, cal)
        });
    (properties.into_iter().map(|p| p.max(0)).product(), calories)
}

fn best_score(ingredients: &[Vec<i32>], calorie_limit: Option<i32>, amounts: &[i32], index: usize, remaining: i32) -> i32 {
    if index == ingredients.len() - 1 {
        let final_amounts = [amounts, &[remaining]].concat();
        let (score, calories) = calculate_score(ingredients, &final_amounts);
        return if calorie_limit.map_or(true, |limit| calories == limit) { score } else { 0 };
    }
    
    (0..=remaining)
        .map(|i| best_score(ingredients, calorie_limit, &[amounts, &[i]].concat(), index + 1, remaining - i))
        .max()
        .unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let ingredients = parse_input(&args[1]);
    
    println!("Best cookie score: {}", best_score(&ingredients, None, &[], 0, 100));
    println!("Best cookie score with 500 calories: {}", best_score(&ingredients, Some(500), &[], 0, 100));
}