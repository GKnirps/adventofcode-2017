use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        None => {
            println!("No file name given");
            return;
        }
        Some(filename) => {
            let input = read_and_parse(&Path::new(filename));
            match input {
                Ok(digits) => {
                    println!("Checksum is: {}", calc_sum_part2(&digits));
                }
                Err(err) => {
                    println!("Unable to read input: {}", err);
                }
            }
        }
    }
}

fn read_and_parse(path: &Path) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut input = String::with_capacity(1024);
    file.read_to_string(&mut input)?;
    let input = input;

    return Ok(parse(&input));
}

fn parse(input: &str) -> Vec<u8> {
    return input
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect();
}

fn calc_sum(digits: &[u8], step: usize) -> u32 {
    let len = digits.len();
    if len < 2 {
        return 0;
    }

    let mut sum: u32 = 0;
    for i in 0..len {
        if digits[i] == digits[(i + step) % len] {
            sum = sum + (digits[i] as u32);
        }
    }
    return sum;
}

fn calc_sum_part2(digits: &[u8]) -> u32 {
    return calc_sum(digits, digits.len() / 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_cum_calculates_correctly() {
        // given
        // some sample arrays with correct sums
        let test_data: [(Vec<u8>, u32); 6] = [
            (vec![], 0),
            (vec![8], 0),
            (vec![1, 1, 2, 2], 3),
            (vec![1, 1, 1, 1], 4),
            (vec![1, 2, 3, 4], 0),
            (vec![9, 1, 2, 1, 2, 1, 2, 9], 9),
        ];

        // when/then
        for &(ref input, output) in &test_data {
            assert_eq!(calc_sum(&input, 1), output);
        }
    }

    #[test]
    fn calc_cum_part2_calculates_correctly() {
        // given
        // some sample arrays with correct sums
        let test_data: [(Vec<u8>, u32); 6] = [
            (vec![], 0),
            (vec![8], 0),
            (vec![1, 2, 1, 2], 6),
            (vec![1, 2, 2, 1], 0),
            (vec![1, 2, 3, 4, 2, 5], 4),
            (vec![1, 2, 1, 3, 1, 4, 1, 5], 4),
        ];

        // when/then
        for &(ref input, output) in &test_data {
            assert_eq!(calc_sum_part2(&input), output);
        }
    }

    #[test]
    fn parse_transforms_string_to_digit_array() {
        // given
        let input = "12ö34 r4";

        // when
        let output = parse(input);

        // then
        assert_eq!(output, [1, 2, 3, 4, 4]);
    }
}
