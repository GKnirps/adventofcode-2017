#![forbid(unsafe_code)]

use std::collections::VecDeque;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let instructions = parse_instructions(&content)?;

    if let Some(freq) = run_sound_until_recover(&instructions) {
        println!("the frequency of the first recovered sound is {freq}");
    } else {
        println!("unable to recover any sound");
    }

    let p1_send_count = run_both(&instructions);
    println!("program 1 sent {p1_send_count} values");

    Ok(())
}

fn run_sound_until_recover(instructions: &[Instruction]) -> Option<i64> {
    let mut state = SoundState::default();
    while state.ip >= 0 && (state.ip as usize) < instructions.len() {
        let recovered: bool;
        (state, recovered) = run_sound_instruction(instructions[state.ip as usize], state);
        if recovered {
            return state.last_recovered;
        }
    }
    None
}

fn run_sound_instruction(inst: Instruction, mut state: SoundState) -> (SoundState, bool) {
    let mut recovered = false;
    match inst {
        Instruction::Snd(op) => {
            state.last_played = Some(op.sound_value(&state));
            state.ip += 1;
        }
        Instruction::Set(reg, op) => {
            state.registers[reg as usize] = op.sound_value(&state);
            state.ip += 1;
        }
        Instruction::Add(reg, op) => {
            state.registers[reg as usize] += op.sound_value(&state);
            state.ip += 1;
        }
        Instruction::Mul(reg, op) => {
            state.registers[reg as usize] *= op.sound_value(&state);
            state.ip += 1;
        }
        Instruction::Mod(reg, op) => {
            state.registers[reg as usize] %= op.sound_value(&state);
            state.ip += 1;
        }
        Instruction::Rcv(reg) => {
            if state.registers[reg as usize] != 0 {
                state.last_recovered = state.last_played;
                recovered = true;
            }
            state.ip += 1;
        }
        Instruction::Jgz(l, r) => {
            if l.sound_value(&state) > 0 {
                state.ip += r.sound_value(&state) as isize;
            } else {
                state.ip += 1;
            }
        }
    };
    (state, recovered)
}

#[derive(Clone, Debug)]
struct SoundState {
    registers: [i64; 256],
    ip: isize,
    last_played: Option<i64>,
    last_recovered: Option<i64>,
}

impl Default for SoundState {
    fn default() -> Self {
        SoundState {
            registers: [0; 256],
            ip: 0,
            last_played: None,
            last_recovered: None,
        }
    }
}

fn run_both(instructions: &[Instruction]) -> usize {
    let mut p0_state = State::default();
    let mut p1_state = State::default();
    p1_state.registers[b'p' as usize] = 1;

    let mut p0_status;
    let mut p1_status;
    (p0_state, p0_status) = run_until_wait(instructions, p0_state, &mut p1_state.out_pipe);
    (p1_state, p1_status) = run_until_wait(instructions, p1_state, &mut p0_state.out_pipe);

    let mut p1_out_count = p1_state.out_pipe.len();

    while p0_status == ProcessStatus::Wait && !p1_state.out_pipe.is_empty() {
        (p0_state, p0_status) = run_until_wait(instructions, p0_state, &mut p1_state.out_pipe);
        if p1_status == ProcessStatus::Wait && !p0_state.out_pipe.is_empty() {
            let p1_out_len = p1_state.out_pipe.len();
            (p1_state, p1_status) = run_until_wait(instructions, p1_state, &mut p0_state.out_pipe);
            p1_out_count += p1_state.out_pipe.len() - p1_out_len;
        }
    }
    p1_out_count
}

fn run_until_wait(
    instructions: &[Instruction],
    mut state: State,
    in_pipe: &mut VecDeque<i64>,
) -> (State, ProcessStatus) {
    while state.ip >= 0 && (state.ip as usize) < instructions.len() {
        let waiting: bool;
        (state, waiting) = run_instruction(instructions[state.ip as usize], state, in_pipe);
        if waiting {
            return (state, ProcessStatus::Wait);
        }
    }
    (state, ProcessStatus::Term)
}

fn run_instruction(
    inst: Instruction,
    mut state: State,
    in_pipe: &mut VecDeque<i64>,
) -> (State, bool) {
    let mut waiting = false;
    match inst {
        Instruction::Snd(op) => {
            state.out_pipe.push_back(op.value(&state));
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
        Instruction::Rcv(reg) => {
            if let Some(input) = in_pipe.pop_front() {
                state.registers[reg as usize] = input;
                state.ip += 1;
            } else {
                waiting = true;
            }
        }
        Instruction::Jgz(l, r) => {
            if l.value(&state) > 0 {
                state.ip += r.value(&state) as isize;
            } else {
                state.ip += 1;
            }
        }
    };
    (state, waiting)
}

#[derive(Clone, Debug)]
struct State {
    registers: [i64; 256],
    ip: isize,
    out_pipe: VecDeque<i64>,
}

impl Default for State {
    fn default() -> Self {
        State {
            registers: [0; 256],
            ip: 0,
            out_pipe: VecDeque::new(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum ProcessStatus {
    Wait,
    Term,
}

#[derive(Copy, Clone, Debug)]
enum Operand {
    Register(u8),
    Value(i64),
}

impl Operand {
    fn sound_value(self, state: &SoundState) -> i64 {
        match self {
            Operand::Register(reg) => state.registers[reg as usize],
            Operand::Value(v) => v,
        }
    }
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
    Rcv(u8),
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
        "rcv" => {
            if operands.len() == 1 && operands.is_ascii() {
                Ok(Instruction::Rcv(operands.as_bytes()[0]))
            } else {
                Err(format!("unable to parse register in instruction '{s}'"))
            }
        }
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

    static EXAMPLE_SOUND: &str = r#"set a 1
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

    static EXAMPLE_SEND: &str = r#"snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d
"#;

    #[test]
    fn run_sound_until_recover_works_for_example() {
        // given
        let instructions = parse_instructions(EXAMPLE_SOUND).expect("expected successful parsing");

        // when
        let result = run_sound_until_recover(&instructions);

        // then
        assert_eq!(result, Some(4));
    }

    #[test]
    fn run_both_works_for_example() {
        // given
        let instructions = parse_instructions(EXAMPLE_SEND).expect("expected successful parsing");

        // when
        let result = run_both(&instructions);

        // then
        assert_eq!(result, 3);
    }
}
