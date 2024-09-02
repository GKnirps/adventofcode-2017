use std::env;
use std::fmt::Write;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let input_base = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;

    let grid = hashes(input_base.trim());
    let used_squares = count_used(&grid);
    println!("{used_squares} squares are used");

    Ok(())
}

fn count_used(grid: &[String]) -> u32 {
    grid.iter()
        .map(|row| {
            row.chars()
                .map(|c| {
                    c.to_digit(16)
                        .expect("expected hexadecimal hash strings")
                        .count_ones()
                })
                .sum::<u32>()
        })
        .sum()
}

fn hashes(base: &str) -> Box<[String]> {
    (0..128)
        .map(|i| {
            knot_hash(
                &format!("{base}-{i}")
                    .as_bytes()
                    .iter()
                    .map(|c| *c as usize)
                    .chain([17, 31, 73, 47, 23])
                    .collect::<Box<[usize]>>(),
            )
        })
        .collect()
}

fn format_hexadecimal(bytes: &[u8]) -> String {
    let mut s: String = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(&mut s, "{byte:02x}").expect("expected write on String to not fail");
    }
    s
}

fn knot_hash(lengths: &[usize]) -> String {
    let mut pos: usize = 0;
    let mut skip: usize = 0;
    let mut hash: Box<[u8]> = (0..=255).collect();

    for _ in 0..64 {
        (hash, pos, skip) = round(hash, lengths, pos, skip);
    }
    let mut dense: [u8; 16] = [0; 16];
    for (i, chunk) in hash.chunks_exact(16).enumerate() {
        dense[i] = chunk.iter().fold(0, |a, b| a ^ b);
    }
    format_hexadecimal(&dense)
}

fn round(
    mut list: Box<[u8]>,
    lengths: &[usize],
    mut pos: usize,
    mut skip: usize,
) -> (Box<[u8]>, usize, usize) {
    for length in lengths {
        reverse(&mut list, pos, *length);
        pos = (pos + length + skip) % list.len();
        skip += 1;
    }

    (list, pos, skip)
}

fn reverse(list: &mut [u8], pos: usize, length: usize) {
    for i in 0..length / 2 {
        let i1 = (i + pos) % list.len();
        let i2 = (pos + length + list.len() - i - 1) % list.len();
        list.swap(i1, i2);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_used_works_for_example() {
        // given
        let grid = hashes("flqrgnkx");

        // when
        let used_squares = count_used(&grid);

        // then
        assert_eq!(used_squares, 8108);
    }
}
