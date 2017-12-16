use std::io::Read;
use std::path::Path;
use std::fs::File;
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
                Ok(rows) => {
                    println!("Checksum is: {}", checksum(&rows));
                }
                Err(err) => {
                    println!("Unable to read input: {}", err);
                }
            }
        }
    }
}

fn read_and_parse(path: &Path) -> std::io::Result<Vec<Vec<i32>>> {
    let mut file = File::open(path)?;
    let mut input = String::with_capacity(1024);
    file.read_to_string(&mut input)?;
    let input = input;

    return Ok(parse_lines(&input));
}

fn parse_lines(input: &str) -> Vec<Vec<i32>> {
    return input.split("\n").map(line_to_row).collect();
}

fn line_to_row(line: &str) -> Vec<i32> {
    return line.split_whitespace()
        .filter_map(|numstr| numstr.parse::<i32>().ok())
        .collect();
}

fn checksum(rows: &Vec<Vec<i32>>) -> i32 {
    return rows.iter().map(|row| row_sum(&row)).sum();
}

fn row_sum(row: &[i32]) -> i32 {
    let min = row.iter().min().unwrap_or(&0);
    let max = row.iter().max().unwrap_or(&0);

    return max - min;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_calculate() {
        // given
        let input = "\n5 1 9 5\n 7 5 3\n 2 4 6 8\n";

        // when
        let rows = parse_lines(input);
        let result = checksum(&rows);

        // then
        assert_eq!(result, 18);
    }

    #[test]
    fn parse_lines_should_parse_lines() {
        // given
        let line = "1 2 3\n 4 5 \n 42";

        // when
        let result = parse_lines(line);

        // then
        assert_eq!(result, &[vec![1, 2, 3], vec![4, 5], vec![42]])
    }

    #[test]
    fn line_to_row_should_parse_line() {
        // given
        let line = "1\tlöo   123  89 ABC .w093re";

        // when
        let result = line_to_row(&line);

        // then
        assert_eq!(result, &[1, 123, 89]);
    }

    #[test]
    fn row_sum_should_calculate_max_minus_min() {
        // given
        let test_data: [(Vec<i32>, i32); 8] = [
            (vec![], 0),
            (vec![4], 0),
            (vec![1, 2, 3], 2),
            (vec![3, 2, 1], 2),
            (vec![9, -1, 41], 42),
            (vec![5, 1, 9, 5], 8),
            (vec![7, 5, 3], 4),
            (vec![2, 4, 6, 8], 6),
        ];

        // when/then
        for &(ref input, output) in &test_data {
            assert_eq!(row_sum(input), output);
        }
    }

    #[test]
    fn checksum_should_add_up_row_sums() {
        // given
        let rows = vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]];

        // when
        let result = checksum(&rows);

        // then
        assert_eq!(result, 18);
    }
}
