use std::fs;
use std::env;

#[derive(Debug, Clone)]
struct Item {
    cost: u32,
    damage: u32,
    armor: u32,
}

#[derive(Debug)]
struct Character {
    hit_points: i32,
    damage: u32,
    armor: u32,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("ERROR: no input filename");
        return;
    }

    let input_file = &args[1];
    let input = fs::read_to_string(input_file).unwrap();

    let boss = parse(&input);

    let weapons = vec![
        Item { cost: 8, damage: 4, armor: 0 },
        Item { cost: 10, damage: 5, armor: 0 },
        Item { cost: 25, damage: 6, armor: 0 },
        Item { cost: 40, damage: 7, armor: 0 },
        Item { cost: 74, damage: 8, armor: 0 },
    ];

    let armors = vec![
        Item { cost: 0, damage: 0, armor: 0 }, // No armor
        Item { cost: 13, damage: 0, armor: 1 },
        Item { cost: 31, damage: 0, armor: 2 },
        Item { cost: 53, damage: 0, armor: 3 },
        Item { cost: 75, damage: 0, armor: 4 },
        Item { cost: 102, damage: 0, armor: 5 },
    ];

    let rings = vec![
        Item { cost: 0, damage: 0, armor: 0 }, // No ring
        Item { cost: 25, damage: 1, armor: 0 },
        Item { cost: 50, damage: 2, armor: 0 },
        Item { cost: 100, damage: 3, armor: 0 },
        Item { cost: 20, damage: 0, armor: 1 },
        Item { cost: 40, damage: 0, armor: 2 },
        Item { cost: 80, damage: 0, armor: 3 },
    ];

    let mut min_cost = u32::MAX;
    let mut max_cost = 0;

    for weapon in &weapons {
        for armor in &armors {
            for (i, ring1) in rings.iter().enumerate() {
                for (j, ring2) in rings.iter().enumerate() {
                    if i != j || i == 0 {
                        let total_cost = weapon.cost + armor.cost + ring1.cost + ring2.cost;
                        let player_damage = weapon.damage + armor.damage + ring1.damage + ring2.damage;
                        let player_armor = weapon.armor + armor.armor + ring1.armor + ring2.armor;

                        let player = Character {
                            hit_points: 100,
                            damage: player_damage,
                            armor: player_armor,
                        };

                        if does_player_win(&player, &boss) {
                            min_cost = min_cost.min(total_cost);
                        } else {
                            max_cost = max_cost.max(total_cost);
                        }
                    }
                }
            }
        }
    }

    println!("Minimum cost to win: {}", min_cost);
    println!("Maximum cost to lose: {}", max_cost);
}

fn parse(input: &str) -> Character {
    let mut lines = input.lines();
    let hit_points = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let damage = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let armor = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();

    Character {
        hit_points,
        damage,
        armor,
    }
}

fn does_player_win(player: &Character, boss: &Character) -> bool {
    let player_damage = (player.damage as i32 - boss.armor as i32).max(1);
    let boss_damage = (boss.damage as i32 - player.armor as i32).max(1);

    let player_turns_to_win = (boss.hit_points + player_damage - 1) / player_damage;
    let boss_turns_to_win = (player.hit_points + boss_damage - 1) / boss_damage;

    player_turns_to_win <= boss_turns_to_win
}
