#![forbid(unsafe_code)]

use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let (start_value_a, start_value_b) = parse(&content)?;

    let count = judge_generators(start_value_a, start_value_b, 40_000_000);
    println!("The judge's final count is {count}");

    let count = judge_generators_picky(start_value_a, start_value_b, 5_000_000);
    println!("When the generators are picky, the judge's final count is {count}");

    Ok(())
}

const FACTOR_A: u64 = 16807;
const FACTOR_B: u64 = 48271;
const PICKY_MOD_A: u64 = 4;
const PICKY_MOD_B: u64 = 8;
const PATTERN: u64 = 0b1111111111111111;

fn judge_generators(start_a: u64, start_b: u64, n: u64) -> u64 {
    let mut count: u64 = 0;
    let mut a = start_a;
    let mut b = start_b;

    for _ in 0..n {
        a = next(FACTOR_A, a);
        b = next(FACTOR_B, b);
        if a & PATTERN == b & PATTERN {
            count += 1;
        }
    }

    count
}

fn next(factor: u64, previous: u64) -> u64 {
    (previous * factor) % 2147483647
}

fn judge_generators_picky(start_a: u64, start_b: u64, n: u64) -> u64 {
    let mut count: u64 = 0;
    let mut a = start_a;
    let mut b = start_b;

    for _ in 0..n {
        a = next_picky(FACTOR_A, PICKY_MOD_A, a);
        b = next_picky(FACTOR_B, PICKY_MOD_B, b);
        if a & PATTERN == b & PATTERN {
            count += 1;
        }
    }

    count
}

fn next_picky(factor: u64, picky_mod: u64, mut previous: u64) -> u64 {
    loop {
        previous = next(factor, previous);
        if previous % picky_mod == 0 {
            return previous;
        }
    }
}

fn parse(input: &str) -> Result<(u64, u64), String> {
    let mut lines = input.lines();
    let a = lines
        .next()
        .ok_or_else(|| "expected two lines".to_string())?
        .strip_prefix("Generator A starts with ")
        .ok_or_else(|| "invalid prefix in first line".to_string())?
        .parse::<u64>()
        .map_err(|e| format!("unable to parse first line: {e}"))?;
    let b = lines
        .next()
        .ok_or_else(|| "expected two lines".to_string())?
        .strip_prefix("Generator B starts with ")
        .ok_or_else(|| "invalid prefix in second line".to_string())?
        .parse::<u64>()
        .map_err(|e| format!("unable to parse second line: {e}"))?;

    Ok((a, b))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn judge_generators_works_for_example() {
        // given
        let a = 65;
        let b = 8921;

        // when
        let count = judge_generators(a, b, 5);

        // then
        assert_eq!(count, 1);
    }

    #[test]
    fn judge_generatores_picky_works_for_example() {
        // given
        let a = 65;
        let b = 8921;

        // when
        let count = judge_generators_picky(a, b, 1056);

        // then
        assert_eq!(count, 1);
    }
}
