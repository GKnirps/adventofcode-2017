use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        None => {
            println!("No file name given");
            return;
        }
        Some(filename) => {
            let input = read_lines(&Path::new(filename));
            match input {
                Ok(lines) => {
                    println!(
                        "Number of steps to jump out: {}",
                        count_jumps_till_exit(parse_lines(&lines))
                    );
                }
                Err(err) => {
                    println!("Unable to read input: {}", err);
                }
            }
        }
    }
}

fn parse_lines(lines: &[String]) -> Vec<i64> {
    lines
        .iter()
        .filter_map(|line| line.parse::<i64>().ok())
        .collect()
}

fn read_lines(path: &Path) -> std::io::Result<Vec<String>> {
    let mut file = File::open(path)?;
    let mut input = String::with_capacity(1024);
    file.read_to_string(&mut input)?;
    Ok(input.lines().map(|s| s.to_owned()).collect())
}

fn count_jumps_till_exit(mut instructions: Vec<i64>) -> u64 {
    let mut steps: u64 = 0;
    let mut pos: i64 = 0;

    while pos >= 0 && pos < instructions.len() as i64 {
        steps += 1;
        let oldpos = pos;
        let offset = instructions[pos as usize];
        pos += offset;
        let new_offset = if offset >= 3 { offset - 1 } else { offset + 1 };
        instructions[oldpos as usize] = new_offset;
    }

    return steps;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_jumps_till_exit_counts_correctly_and_exits() {
        // given
        let instructions = vec![0, 3, 0, 2, -3];

        // when
        let jumps = count_jumps_till_exit(instructions);

        // then
        assert_eq!(jumps, 6);
    }
}
