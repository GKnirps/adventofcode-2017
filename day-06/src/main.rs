use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let banks: Vec<u32> = content
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().map_err(|e| e.to_string()))
        .collect::<Result<Vec<u32>, String>>()?;

    let relocates_until_repeat = count_relocate_until_repeat(banks);
    println!(
        "{} relocates until the memory pattern repeats",
        relocates_until_repeat
    );

    Ok(())
}

fn count_relocate_until_repeat(banks: Vec<u32>) -> usize {
    let mut current: Vec<u32> = banks;
    let mut seen: HashSet<Vec<u32>> = HashSet::with_capacity(256);
    while !seen.contains(&current) {
        seen.insert(current.clone());
        current = relocate(current);
    }
    seen.len()
}

fn relocate(mut banks: Vec<u32>) -> Vec<u32> {
    let (index, max_blocks): (usize, u32) = match banks.iter().enumerate().max_by(
        |(il, bl), (ir, br)| {
            if bl == br {
                ir.cmp(il)
            } else {
                bl.cmp(br)
            }
        },
    ) {
        Some((i, m)) => (i, *m),
        None => {
            return banks;
        }
    };
    let n_banks = banks.len();
    banks[index] = 0;
    for i in 1..=(max_blocks as usize) {
        banks[(index + i) % n_banks] += 1;
    }
    banks
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn relocate_works_for_example() {
        // given
        let banks = vec![0, 2, 7, 0];

        // when
        let result = relocate(banks);

        // then
        assert_eq!(&result, &[2, 4, 1, 2]);
    }

    #[test]
    fn count_relocate_until_repeat_works_for_example() {
        // given
        let banks = vec![0, 2, 7, 0];

        // when
        let count = count_relocate_until_repeat(banks);

        // then
        assert_eq!(count, 5);
    }
}
