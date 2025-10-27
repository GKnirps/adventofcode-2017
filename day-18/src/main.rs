#![forbid(unsafe_code)]

use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let instructions = parse_instructions(&content)?;

    if let Some(freq) = run_until_recover(&instructions) {
        println!("the frequency of the first recovered sound is {freq}");
    }

    Ok(())
}

fn run_until_recover(instructions: &[Instruction]) -> Option<i64> {
    let mut state = State::default();
    while state.ip >= 0 && (state.ip as usize) < instructions.len() {
        let recovered: bool;
        (state, recovered) = run_instruction(instructions[state.ip as usize], state);
        if recovered {
            return state.last_recovered;
        }
    }
    None
}

fn run_instruction(inst: Instruction, mut state: State) -> (State, bool) {
    let mut recovered = false;
    match inst {
        Instruction::Snd(op) => {
            state.last_played = Some(op.value(&state));
            state.ip += 1;
        }
        Instruction::Set(reg, op) => {
            state.registers[reg as usize] = op.value(&state);
            state.ip += 1;
        }
        Instruction::Add(reg, op) => {
            state.registers[reg as usize] += op.value(&state);
            state.ip += 1;
        }
        Instruction::Mul(reg, op) => {
            state.registers[reg as usize] *= op.value(&state);
            state.ip += 1;
        }
        Instruction::Mod(reg, op) => {
            state.registers[reg as usize] %= op.value(&state);
            state.ip += 1;
        }
        Instruction::Rcv(op) => {
            if op.value(&state) != 0 {
                state.last_recovered = state.last_played;
                recovered = true;
            }
            state.ip += 1;
        }
        Instruction::Jgz(l, r) => {
            if l.value(&state) > 0 {
                state.ip += r.value(&state) as isize;
            } else {
                state.ip += 1;
            }
        }
    };
    (state, recovered)
}

#[derive(Clone, Debug)]
struct State {
    registers: [i64; 256],
    ip: isize,
    last_played: Option<i64>,
    last_recovered: Option<i64>,
}

impl Default for State {
    fn default() -> Self {
        State {
            registers: [0; 256],
            ip: 0,
            last_played: None,
            last_recovered: None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Operand {
    Register(u8),
    Value(i64),
}

impl Operand {
    fn value(self, state: &State) -> i64 {
        match self {
            Operand::Register(reg) => state.registers[reg as usize],
            Operand::Value(v) => v,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Snd(Operand),
    Set(u8, Operand),
    Add(u8, Operand),
    Mul(u8, Operand),
    Mod(u8, Operand),
    Rcv(Operand),
    Jgz(Operand, Operand),
}

fn parse_instructions(input: &str) -> Result<Box<[Instruction]>, String> {
    input.lines().map(parse_instruction).collect()
}

fn parse_instruction(s: &str) -> Result<Instruction, String> {
    let (operator, operands) = s
        .split_once(' ')
        .ok_or_else(|| format!("unable to split operator from operands in instruction '{s}'"))?;
    match operator {
        "snd" => Ok(Instruction::Snd(parse_operand(operands)?)),
        "set" => {
            let (reg, op) = parse_reg_and_operand(operands)?;
            Ok(Instruction::Set(reg, op))
        }
        "add" => {
            let (reg, op) = parse_reg_and_operand(operands)?;
            Ok(Instruction::Add(reg, op))
        }
        "mul" => {
            let (reg, op) = parse_reg_and_operand(operands)?;
            Ok(Instruction::Mul(reg, op))
        }
        "mod" => {
            let (reg, op) = parse_reg_and_operand(operands)?;
            Ok(Instruction::Mod(reg, op))
        }
        "rcv" => Ok(Instruction::Rcv(parse_operand(operands)?)),
        "jgz" => {
            let (r, l) = operands
                .split_once(' ')
                .ok_or_else(|| format!("unable to split operands in instruction '{s}'"))?;
            Ok(Instruction::Jgz(parse_operand(r)?, parse_operand(l)?))
        }
        _ => Err(format!(
            "Unknown operator '{operator}' in instruction '{s}'"
        )),
    }
}

fn parse_reg_and_operand(ops: &str) -> Result<(u8, Operand), String> {
    let (reg, op) = ops
        .split_once(' ')
        .ok_or_else(|| format!("unable to split operands in '{ops}'"))?;
    if reg.len() == 1 && reg.is_ascii() {
        Ok((reg.as_bytes()[0], parse_operand(op)?))
    } else {
        Err(format!("invalid register '{reg}' in operand pair '{ops}'"))
    }
}

fn parse_operand(op: &str) -> Result<Operand, String> {
    if let Ok(v) = op.parse::<i64>() {
        Ok(Operand::Value(v))
    } else if op.len() == 1 && op.is_ascii() {
        Ok(Operand::Register(op.as_bytes()[0]))
    } else {
        Err(format!("unable to parse operand '{op}'"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2
"#;

    #[test]
    fn run_until_recover_works_for_example() {
        // given
        let instructions = parse_instructions(EXAMPLE).expect("expected successful parsing");

        // when
        let result = run_until_recover(&instructions);

        // then
        assert_eq!(result, Some(4));
    }
}
