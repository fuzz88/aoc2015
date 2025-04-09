fn main() {
    // Input: row and column from the puzzle
    let target_row = 2978;
    let target_col = 3083;

    // Constants
    let initial_code = 20151125;
    let multiplier = 252533;
    let modulus = 33554393;

    // Calculate the position in the sequence
    let position = calculate_position(target_row, target_col);

    // Generate the code at the target position
    let code = generate_code(position, initial_code, multiplier, modulus);

    println!("The code at row {}, column {} is: {}", target_row, target_col, code);
}

/// Calculate the position in the sequence for the given row and column
fn calculate_position(row: usize, col: usize) -> usize {
    // The position is determined by the sum of the diagonal numbers
    let diagonal = row + col - 1;
    let position = (diagonal * (diagonal - 1)) / 2 + col;
    position
}

/// Generate the code at the given position
fn generate_code(position: usize, initial_code: usize, multiplier: usize, modulus: usize) -> usize {
    let mut code = initial_code;
    for _ in 1..position {
        code = (code * multiplier) % modulus;
    }
    code
}