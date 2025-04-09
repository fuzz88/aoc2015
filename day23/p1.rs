use std::fs;

#[derive(Debug, Clone)]
struct State {
    a: u32,
    b: u32,
    pc: isize,
}

impl State {
    fn new() -> State {
        State { a: 0, b: 0, pc: 0 }
    }
}

struct Instr {
    reg: Option<String>,
    offset: Option<isize>,
    apply: fn(&Instr, State) -> State,
}

impl Instr {
    fn tpl(&self, mut state: State) -> State {
        if let Some(reg) = &self.reg {
            if reg == "a" {
                state.a *= 3;
            } else {
                state.b *= 3;
            }
        }
        state.pc += 1;
        state
    }

    fn inc(&self, mut state: State) -> State {
        if let Some(reg) = &self.reg {
            if reg == "a" {
                state.a += 1;
            } else {
                state.b += 1;
            }
        }
        state.pc += 1;
        state
    }

    fn hlf(&self, mut state: State) -> State {
        if let Some(reg) = &self.reg {
            if reg == "a" {
                state.a /= 2;
            } else {
                state.b /= 2;
            }
        }
        state.pc += 1;
        state
    }

    fn jmp(&self, mut state: State) -> State {
        if let Some(offset) = self.offset {
            state.pc += offset;
        }
        state
    }

    fn jie(&self, mut state: State) -> State {
        if let (Some(reg), Some(offset)) = (&self.reg, self.offset) {
            let value = if reg == "a" { state.a } else { state.b };
            if value % 2 == 0 {
                state.pc += offset;
            } else {
                state.pc += 1;
            }
        }
        state
    }

    fn jio(&self, mut state: State) -> State {
        if let (Some(reg), Some(offset)) = (&self.reg, self.offset) {
            let value = if reg == "a" { state.a } else { state.b };
            if value == 1 {
                state.pc += offset;
            } else {
                state.pc += 1;
            }
        }
        state
    }
}

fn parse_instruction(line: &str) -> Instr {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    match tokens[0] {
        "hlf" => Instr {
            reg: Some(tokens[1].to_string()),
            offset: None,
            apply: Instr::hlf,
        },
        "tpl" => Instr {
            reg: Some(tokens[1].to_string()),
            offset: None,
            apply: Instr::tpl,
        },
        "inc" => Instr {
            reg: Some(tokens[1].to_string()),
            offset: None,
            apply: Instr::inc,
        },
        "jmp" => Instr {
            reg: None,
            offset: Some(tokens[1].parse::<isize>().unwrap()),
            apply: Instr::jmp,
        },
        "jie" => Instr {
            reg: Some(tokens[1].trim_end_matches(',').to_string()),
            offset: Some(tokens[2].parse::<isize>().unwrap()),
            apply: Instr::jie,
        },
        "jio" => Instr {
            reg: Some(tokens[1].trim_end_matches(',').to_string()),
            offset: Some(tokens[2].parse::<isize>().unwrap()),
            apply: Instr::jio,
        },
        _ => panic!("Unknown instruction: {}", tokens[0]),
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("ERROR: failed to read input.txt");
    let source: Vec<String> = input.lines().map(|el| el.to_string()).collect();

    let mut state = State::new();
    let program: Vec<Instr> = source.iter().map(|line| parse_instruction(line)).collect();

    while (state.pc as usize) < program.len() {
        let instr = &program[state.pc as usize];
        state = (instr.apply)(instr, state.clone());
    }

    println!("Final state: {:?}", state);
    println!("Value in register b: {}", state.b);
}
