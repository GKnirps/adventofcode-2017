#![forbid(unsafe_code)]

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

    Ok(())
}

fn insert_2017(steps: usize) -> usize {
    let mut buffer: Vec<usize> = Vec::with_capacity(2018);
    buffer.push(0);
    let mut pos = 0;
    for i in 1..=2017 {
        pos = (pos + steps) % buffer.len() + 1;
        buffer.insert(pos, i);
    }
    buffer[(pos + 1) % buffer.len()]
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
