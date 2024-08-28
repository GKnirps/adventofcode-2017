use std::env;
use std::fmt::Write;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let lengths = parse_input(&content)?;

    let hash_1 = round((0..=255u8).collect(), &lengths, 0, 0).0;
    let result_1 =
        hash_1.first().copied().unwrap_or(0) as u32 * hash_1.get(1).copied().unwrap_or(0) as u32;
    println!("The product of the first two number is {result_1}");

    let ascii_lengths = parse_ascii_input(&content);
    let hash = knot_hash(&ascii_lengths);
    println!("The Knot Hash of the input is {hash}");

    Ok(())
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

fn parse_ascii_input(input: &str) -> Box<[usize]> {
    input
        .trim()
        .as_bytes()
        .iter()
        .map(|b| *b as usize)
        .chain([17, 31, 73, 47, 23])
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn round_works_for_example() {
        // given
        let list = Box::new([0, 1, 2, 3, 4]);
        let lengths = &[3, 4, 1, 5];

        // when
        let (hash, pos, skip) = round(list, lengths, 0, 0);

        // then
        assert_eq!(&hash as &[u8], &[3, 4, 2, 1, 0]);
        assert_eq!(pos, 4);
        assert_eq!(skip, 4);
    }

    #[test]
    fn knot_hash_works_for_examples() {
        assert_eq!(
            &knot_hash(&parse_ascii_input("")),
            "a2582a3a0e66e6e86e3812dcb672a272"
        );
        assert_eq!(
            &knot_hash(&parse_ascii_input("AoC 2017")),
            "33efeb34ea91902bb2f59c9920caa6cd"
        );
        assert_eq!(
            &knot_hash(&parse_ascii_input("1,2,3")),
            "3efbe78a8d82f29979031a4aa0b16a9d"
        );
        assert_eq!(
            &knot_hash(&parse_ascii_input("1,2,4")),
            "63960835bcdc130f0b66d7ff4f6a5a8e"
        );
    }
}
