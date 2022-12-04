use std::collections::HashSet;
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
    let words: Vec<&str> = passphrase.trim().split_whitespace().collect();

    let mut seen: HashSet<&str> = HashSet::with_capacity(words.len());
    for word in words {
        if seen.contains(word) {
            return false;
        }
        seen.insert(word);
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_valid_passphrases_counts_correctly() {
        // given
        let phrases = &[
            " aa bb cc dd ee ".to_owned(),
            "aa bb cc dd aa".to_owned(),
            "aa bb cc dd aaa".to_owned(),
        ];

        // when
        let count = count_valid_passphrases(phrases);

        // then
        assert_eq!(count, 2);
    }

    #[test]
    fn passphrase_valid_should_recognize_valid_passphrases() {
        // given
        let valid_phrases = &[" aa bb cc dd ee ", "aa bb cc dd aaa"];

        // when/then
        for passphrase in valid_phrases {
            assert!(passphrase_valid(passphrase));
        }
    }

    #[test]
    fn passphrase_valid_should_recognize_invalid_passphrase() {
        // given
        let invalid_phrase = "aa bb cc dd aa";

        // when
        let result = passphrase_valid(invalid_phrase);

        // then
        assert!(!result);
    }
}
