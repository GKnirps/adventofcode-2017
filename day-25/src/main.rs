use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;

    let (tm, diagnostic_step) = parse(&content)?;

    let end_configuration = run_turing_machine(&tm, diagnostic_step)?;

    let checksum = diagnostic_checksum(&end_configuration);

    println!("The diagnostic checksum is {}", checksum);

    Ok(())
}

type State = char;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct TuringMachine {
    initial_state: State,
    transitions: HashMap<(State, bool), (bool, State, Direction)>,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
struct Configuration {
    state: State,
    pos: usize,
    tape: VecDeque<bool>,
}

impl Configuration {
    fn new(state: State) -> Self {
        Configuration {
            state,
            pos: 1,
            tape: VecDeque::from([false, false, false]),
        }
    }
}

fn run_turing_machine(tm: &TuringMachine, n_cycles: usize) -> Result<Configuration, String> {
    let mut configuration = Configuration::new(tm.initial_state);
    for _ in 0..n_cycles {
        configuration = turing_step(tm, configuration)?;
    }
    Ok(configuration)
}

fn diagnostic_checksum(configuration: &Configuration) -> usize {
    configuration.tape.iter().filter(|cell| **cell).count()
}

fn turing_step(
    tm: &TuringMachine,
    mut configuration: Configuration,
) -> Result<Configuration, String> {
    let current_state = configuration.state;
    let current_cell = configuration.tape[configuration.pos];
    let (write_val, next_state, dir) = tm
        .transitions
        .get(&(current_state, current_cell))
        .ok_or_else(|| {
            format!(
                "No transiton for configuration ({}, {})",
                current_state, current_cell,
            )
        })?;

    configuration.tape[configuration.pos] = *write_val;
    configuration.state = *next_state;
    match dir {
        Direction::Left => {
            if configuration.pos == 0 {
                configuration.tape.push_front(false);
            } else {
                configuration.pos -= 1;
            }
        }
        Direction::Right => {
            if configuration.pos == configuration.tape.len() - 1 {
                configuration.tape.push_back(false);
            }
            configuration.pos += 1;
        }
    }

    Ok(configuration)
}

fn parse(content: &str) -> Result<(TuringMachine, usize), String> {
    let mut lines = content
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim().trim_end_matches(':').trim_end_matches('.'));
    let initial_state: State = lines
        .next()
        .ok_or_else(|| "File is empty".to_owned())?
        .trim_start_matches("Begin in state ")
        .chars()
        .next()
        .ok_or_else(|| "No initial state found".to_owned())?;

    let diagnostic_check_step: usize = lines
        .next()
        .ok_or_else(|| "Expected check time in second line".to_owned())?
        .trim_start_matches("Perform a diagnostic checksum after ")
        .trim_end_matches(" steps")
        .parse::<usize>()
        .map_err(|e| format!("Error parsing check time: {}", e))?;

    let mut transitions: HashMap<(State, bool), (bool, State, Direction)> =
        HashMap::with_capacity(128);
    while let Some(line) = lines.next() {
        let state: State = line
            .trim_start_matches("In state ")
            .chars()
            .next()
            .ok_or_else(|| format!("No state found in transition rule '{}'", line))?;
        for _ in 0..2 {
            let current_value = match lines.next() {
                Some("If the current value is 0") => false,
                Some("If the current value is 1") => true,
                _ => return Err("Did not find current value".to_owned()),
            };
            let write_value = match lines.next() {
                Some("- Write the value 0") => false,
                Some("- Write the value 1") => true,
                failed_line => {
                    return Err(format!(
                        "Did not find next value in line '{:?}'",
                        failed_line
                    ))
                }
            };
            let direction: Direction = match lines.next() {
                Some("- Move one slot to the left") => Direction::Left,
                Some("- Move one slot to the right") => Direction::Right,
                _ => return Err("Did not find valid direction".to_owned()),
            };
            let target_state: State = lines
                .next()
                .ok_or_else(|| "Unable to find next state".to_owned())?
                .trim_start_matches("- Continue with state ")
                .chars()
                .next()
                .ok_or_else(|| "no target state found".to_owned())?;
            transitions.insert(
                (state, current_value),
                (write_value, target_state, direction),
            );
        }
    }
    Ok((
        TuringMachine {
            initial_state,
            transitions,
        },
        diagnostic_check_step,
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = r"Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
";

    #[test]
    fn parse_works_for_example() {
        // when
        let result = parse(EXAMPLE_INPUT);

        // then
        let (turing_machine, diagnostic) = result.expect("Expected successful parsing");
        assert_eq!(diagnostic, 6);

        let mut transitions: HashMap<(State, bool), (bool, State, Direction)> =
            HashMap::with_capacity(4);
        transitions.insert(('A', false), (true, 'B', Direction::Right));
        transitions.insert(('A', true), (false, 'B', Direction::Left));
        transitions.insert(('B', false), (true, 'A', Direction::Left));
        transitions.insert(('B', true), (true, 'A', Direction::Right));

        assert_eq!(
            turing_machine,
            TuringMachine {
                initial_state: 'A',
                transitions
            }
        );
    }

    #[test]
    fn run_turing_machine_works_for_example() {
        // given
        let (turing_machine, diagnostic_step) =
            parse(EXAMPLE_INPUT).expect("Expected successful parsing");

        // when
        let result = run_turing_machine(&turing_machine, diagnostic_step);

        // then
        let end_configuration = result.expect("Expected successful run");
        assert_eq!(end_configuration.state, 'A');
        assert_eq!(end_configuration.pos, 2);
        assert_eq!(end_configuration.tape, &[true, true, false, true])
    }
}
