use std::fs;
use std::env;

fn parse_input(file: &str) -> Vec<Vec<i32>> {
    fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(|line| line.split_whitespace()
            .filter_map(|s| s.trim_end_matches(',').parse().ok())
            .collect())
        .collect()
}

fn calculate_score(ingredients: &Vec<Vec<i32>>, amounts: &Vec<i32>) -> (i32, i32) {
    let mut properties = vec![0; ingredients[0].len() - 1];
    let mut calories = 0;
    
    for (ingredient, &amount) in ingredients.iter().zip(amounts.iter()) {
        for (i, &value) in ingredient.iter().enumerate() {
            if i == properties.len() {
                calories += value * amount;
            } else {
                properties[i] += value * amount;
            }
        }
    }
    
    let score = properties.iter().map(|&p| p.max(0)).product();
    (score, calories)
}

fn best_score(ingredients: &Vec<Vec<i32>>, calorie_limit: Option<i32>, amounts: &mut Vec<i32>, index: usize, remaining: i32) -> i32 {
    if index == ingredients.len() - 1 {
        amounts.push(remaining);
        let (score, calories) = calculate_score(ingredients, amounts);
        amounts.pop();
        return if calorie_limit.is_none() || calories == calorie_limit.unwrap() { score } else { 0 };
    }
    
    (0..=remaining).map(|i| {
        amounts.push(i);
        let best = best_score(ingredients, calorie_limit, amounts, index + 1, remaining - i);
        amounts.pop();
        best
    }).max().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let ingredients = parse_input(&args[1]);
    
    println!("Best cookie score: {}", best_score(&ingredients, None, &mut vec![], 0, 100));
    println!("Best cookie score with 500 calories: {}", best_score(&ingredients, Some(500), &mut vec![], 0, 100));
}