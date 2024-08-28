use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let lengths = parse_input(&content)?;

    let result_1 = run_for_lengths((0..=255u8).collect(), &lengths);
    println!("The product of the first two number is {result_1}");

    Ok(())
}

fn run_for_lengths(mut list: Box<[u8]>, lengths: &[usize]) -> u32 {
    let mut pos = 0;
    for (skip, length) in lengths.iter().enumerate() {
        reverse(&mut list, pos, *length);
        pos += length + skip;
    }

    list.get(0).copied().unwrap_or(0) as u32 * list.get(1).copied().unwrap_or(0) as u32
}

fn reverse(list: &mut [u8], pos: usize, length: usize) {
    for i in 0..length / 2 {
        let i1 = (i + pos) % list.len();
        let i2 = (pos + length + list.len() - i - 1) % list.len();
        let first = list[i1];
        list[i1] = list[i2];
        list[i2] = first;
    }
}

fn parse_input(input: &str) -> Result<Box<[usize]>, String> {
    input
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<usize>()
                .map_err(|e| format!("error parsing input '{s}': {e}"))
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_for_lengths_works_for_example() {
        // given
        let list = Box::new([0, 1, 2, 3, 4]);
        let lengths = &[3, 4, 1, 5];

        // when
        let result = run_for_lengths(list, lengths);

        // then
        assert_eq!(result, 12);
    }
}
