use std::fs;

#[derive(Debug)]
struct State {
    a: u32,
    b: u32,
    pc: usize,
}

impl State {
    fn new() -> State {
        State { a: 0, b: 0, pc: 0 }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("ERROR: failed to read input.txt");

    let program: Vec<&str> = input.lines().collect();

    let state = State::new();

    println!("program len={}", program.len());

    loop {
        if state.pc >= program.len() {
            dbg!(state);
            break;
        }
        let instr = program[state.pc];
        let instr: Vec<&str> = instr.split(" ").collect();
        dbg!(instr);
        break;
    }
}
