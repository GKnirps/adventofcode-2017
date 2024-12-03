use std::collections::HashSet;
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

    let regions = count_regions(&grid);
    println!("There are {regions} distinct regions.");

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

fn count_regions(grid: &[String]) -> usize {
    let height = grid.len();
    let width = grid.first().map(|hash| hash.len() * 4).unwrap_or(0);
    if height == 0 || width == 0 {
        return 0;
    }

    let mut visited_active: HashSet<(usize, usize)> = HashSet::with_capacity(height * width);
    let mut stack: Vec<(usize, usize)> = Vec::with_capacity(height * width);
    let mut n_regions = 0;

    for y in 0..height {
        for x in 0..width {
            if is_used(grid, x, y) && !visited_active.contains(&(x, y)) {
                stack.clear();
                stack.push((x, y));
                visited_active.insert((x, y));
                n_regions += 1;
                while let Some((x, y)) = stack.pop() {
                    if x > 0 && is_used(grid, x - 1, y) && !visited_active.contains(&(x - 1, y)) {
                        stack.push((x - 1, y));
                        visited_active.insert((x - 1, y));
                    }
                    if is_used(grid, x + 1, y) && !visited_active.contains(&(x + 1, y)) {
                        stack.push((x + 1, y));
                        visited_active.insert((x + 1, y));
                    }
                    if y > 0 && is_used(grid, x, y - 1) && !visited_active.contains(&(x, y - 1)) {
                        stack.push((x, y - 1));
                        visited_active.insert((x, y - 1));
                    }
                    if is_used(grid, x, y + 1) && !visited_active.contains(&(x, y + 1)) {
                        stack.push((x, y + 1));
                        visited_active.insert((x, y + 1));
                    }
                }
            }
        }
    }

    n_regions
}

fn is_used(grid: &[String], x: usize, y: usize) -> bool {
    grid.get(y)
        .and_then(|row| {
            let supercol = row.as_bytes().get(x / 4)?;
            Some((char::from(*supercol).to_digit(16)? >> (3 - x % 4)) & 1 == 1)
        })
        .unwrap_or(false)
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

    #[test]
    fn count_regions_works_for_example() {
        // given
        let grid = hashes("flqrgnkx");

        // when
        let regions = count_regions(&grid);

        // then
        assert_eq!(regions, 1242);
    }
}
