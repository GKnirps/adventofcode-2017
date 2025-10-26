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
    let steps: usize = content
        .trim()
        .parse()
        .map_err(|e| format!("unable to parse input '{}': {e}", content.trim()))?;

    let after_2017 = insert_2017(steps);
    println!("The value after 2017 is {after_2017}");

    let after_50_mill = insert_50_mill(steps);
    println!("The value after 0 is {after_50_mill}");

    Ok(())
}

fn insert_n(steps: usize, n: usize) -> VecDeque<usize> {
    let mut buffer: VecDeque<usize> = VecDeque::with_capacity(n + 1);
    buffer.push_back(0);
    for i in 1..=n {
        for _ in 0..steps {
            let prev = buffer.pop_front().unwrap();
            buffer.push_back(prev);
        }
        let prev = buffer.pop_front().unwrap();
        buffer.push_back(prev);
        buffer.push_front(i);
    }
    buffer
}

fn insert_2017(steps: usize) -> usize {
    let buffer = insert_n(steps, 2017);
    *buffer
        .iter()
        .skip_while(|v| **v != 2017)
        .nth(1)
        .or(buffer.front())
        .expect("expected to find 2017")
}

fn insert_50_mill(steps: usize) -> usize {
    let buffer = insert_n(steps, 50_000_000);
    *buffer
        .iter()
        .skip_while(|v| **v != 0)
        .nth(1)
        .or(buffer.front())
        .expect("expected to find 0")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert_2017_works_for_example() {
        // given
        let steps: usize = 3;

        // when
        let result = insert_2017(steps);

        // then
        assert_eq!(result, 638);
    }
}
