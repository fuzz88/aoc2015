use std::env;
use std::fs;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_filename>", args[0]);
        return;
    }
    let input_filename = &args[1];
    match read_grid_from_file(input_filename) {
        Ok(grid) => {
            // Without set_corners_on
            let mut grid_without_corners = grid.clone();
            for _ in 0..100 {
                grid_without_corners = next_grid_state(&grid_without_corners);
            }
            let true_count_without_corners = count_true_elements(&grid_without_corners);
            println!(
                "Number of lights on after 100 steps (without corners always on): {}",
                true_count_without_corners
            );

            // With set_corners_on
            let mut grid_with_corners = grid.clone();
            for _ in 0..100 {
                set_corners_on(&mut grid_with_corners); // Ensure corners are on initially
                grid_with_corners = next_grid_state(&grid_with_corners);
            }
            set_corners_on(&mut grid_with_corners); // Ensure corners are on initially
            let true_count_with_corners = count_true_elements(&grid_with_corners);
            println!(
                "Number of lights on after 100 steps (with corners always on): {}",
                true_count_with_corners
            );
        },
        Err(e) => eprintln!("ERROR: failed to read input {}", e),
    }
}

fn read_grid_from_file(filename: &str) -> io::Result<Vec<Vec<bool>>> {
    let content = fs::read_to_string(filename)?;
    let grid = content
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();
    Ok(grid)
}

fn count_neighbors(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> (usize, usize) {
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),         (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];
    let mut true_count = 0;
    let mut false_count = 0;

    for (dx, dy) in directions.iter() {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx >= 0 && ny >= 0 && nx < grid.len() as isize && ny < grid[0].len() as isize {
            if grid[nx as usize][ny as usize] {
                true_count += 1;
            } else {
                false_count += 1;
            }
        } else {
            false_count += 1; // Count missing neighbors as false
        }
    }

    (true_count, false_count)
}

fn set_corners_on(grid: &mut Vec<Vec<bool>>) {
    let rows = grid.len();
    let cols = grid[0].len();
    grid[0][0] = true;
    grid[0][cols - 1] = true;
    grid[rows - 1][0] = true;
    grid[rows - 1][cols - 1] = true;
}

fn next_grid_state(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut backbuffer = grid.clone();
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            let (true_count, _) = count_neighbors(grid, x, y);
            if grid[x][y] {
                backbuffer[x][y] = true_count == 2 || true_count == 3;
            } else {
                backbuffer[x][y] = true_count == 3;
            }
        }
    }
    backbuffer
}

fn count_true_elements(grid: &Vec<Vec<bool>>) -> usize {
    grid.iter().flatten().filter(|&&cell| cell).count()
}
