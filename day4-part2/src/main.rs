use std::collections::{HashSet, BTreeMap};
use std::path::Path;
use std::io::Read;
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
            let input = read_lines(&Path::new(filename));
            match input {
                Ok(lines) => {
                    println!(
                        "Number of valid phrases: {}",
                        count_valid_passphrases(&lines)
                    );
                }
                Err(err) => {
                    println!("Unable to read input: {}", err);
                }
            }
        }
    }
}

fn read_lines(path: &Path) -> std::io::Result<Vec<String>> {
    let mut file = File::open(path)?;
    let mut input = String::with_capacity(1024);
    file.read_to_string(&mut input)?;
    Ok(input.lines().map(|s| s.to_owned()).collect())
}

fn count_valid_passphrases(phrases: &[String]) -> usize {
    phrases
        .iter()
        .filter(|phrase| passphrase_valid(phrase))
        .count()
}

fn passphrase_valid(passphrase: &str) -> bool {
    let char_counts: Vec<BTreeMap<char, u32>> = passphrase
        .trim()
        .split_whitespace()
        .map(count_chars)
        .collect();

    let mut seen: HashSet<BTreeMap<char, u32>> = HashSet::with_capacity(char_counts.len());
    for count in char_counts {
        if seen.contains(&count) {
            return false;
        }
        seen.insert(count);
    }
    return true;
}

fn count_chars(input: &str) -> BTreeMap<char, u32> {
    let mut counter: BTreeMap<char, u32> = BTreeMap::new();
    for c in input.chars() {
        let entry = counter.entry(c).or_insert(0);
        *entry += 1;
    }
    return counter;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_valid_passphrases_counts_correctly() {
        // given
        let phrases = &[
            " abcde fghjj ".to_owned(),
            "abcde xyz ecdab".to_owned(),
            "a ab abc abd abf abj".to_owned(),
            "iiii oiii ooii oooi oooo".to_owned(),
            "oiii ioii iioi iiio".to_owned(),
        ];

        // when
        let count = count_valid_passphrases(phrases);

        // then
        assert_eq!(count, 3);
    }

    #[test]
    fn count_chars_should_count_a_words_chars() {
        // given
        let word = "foobar";

        // when
        let counter = count_chars(word);


        // then
        assert_eq!(counter.len(), 5);
        assert_eq!(counter.get(&'f'), Some(&1));
        assert_eq!(counter.get(&'o'), Some(&2));
        assert_eq!(counter.get(&'b'), Some(&1));
        assert_eq!(counter.get(&'a'), Some(&1));
        assert_eq!(counter.get(&'r'), Some(&1));
    }

    #[test]
    fn passphrase_valid_should_recognize_valid_passphrases() {
        // given
        let valid_phrases = &[
            " abcde fghij ",
            "a ab abc abd abf abj",
            "iiii oiii ooii oooi oooo",
        ];

        // when/then
        for passphrase in valid_phrases {
            assert!(passphrase_valid(passphrase));
        }
    }

    #[test]
    fn passphrase_valid_should_recognize_invalid_passphrase() {
        // given
        let invalid_phrases = &["abcde xyz ecdab", "oiii ioii iioi iiio"];

        // when/then
        for passphrase in invalid_phrases {
            assert!(!passphrase_valid(passphrase));
        }
    }

}
