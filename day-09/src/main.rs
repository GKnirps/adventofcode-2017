use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;

    let (group_score, garbage_count) = score_groups(content.trim())?;
    println!("Total group score is {group_score}, the garbage count is {garbage_count}.");

    Ok(())
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum State {
    Default,
    Closed,
    Garbage,
    Cancel,
}

fn score_groups(input: &str) -> Result<(u32, u32), String> {
    let mut depth: u32 = 0;
    let mut state = State::Default;
    let mut score: u32 = 0;
    let mut garbage_count: u32 = 0;
    for (i, c) in input.chars().enumerate() {
        match (state, c) {
            (State::Default, '{') => {
                depth += 1;
                score += depth;
            }
            (State::Default, '}') | (State::Closed, '}') => {
                if depth == 0 {
                    return Err(format!("Unexpected closed curly brace at position {i}"));
                }
                depth -= 1;
                state = State::Closed;
            }
            (State::Default, '<') => {
                state = State::Garbage;
            }
            (State::Closed, ',') => {
                state = State::Default;
            }
            (State::Garbage, '>') => {
                state = State::Closed;
            }
            (State::Garbage, '!') => {
                state = State::Cancel;
            }
            (State::Garbage, _) => {
                garbage_count += 1;
            }
            (State::Cancel, _) => {
                state = State::Garbage;
            }
            (_, any) => {
                return Err(format!("unexpected char '{any}' at position {i}"));
            }
        }
    }
    if depth == 0 {
        Ok((score, garbage_count))
    } else {
        Err(format!("{depth} unclosed groups at end of input"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn score_groups_works_for_example() {
        assert_eq!(score_groups("{}"), Ok((1, 0)));
        assert_eq!(score_groups("{{{}}}"), Ok((6, 0)));
        assert_eq!(score_groups("{{},{}}"), Ok((5, 0)));
        assert_eq!(score_groups("{{{},{},{{}}}}"), Ok((16, 0)));
        assert_eq!(score_groups("{<a>,<a>,<a>,<a>}"), Ok((1, 4)));
        assert_eq!(score_groups("{{<ab>},{<ab>},{<ab>},{<ab>}}"), Ok((9, 8)));
        assert_eq!(score_groups("{{<!!>},{<!!>},{<!!>},{<!!>}}"), Ok((9, 0)));
        assert_eq!(score_groups("{{<a!>},{<a!>},{<a!>},{<ab>}}"), Ok((3, 17)));
    }
}
