#![forbid(unsafe_code)]

use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let moves = parse_moves(&content)?;

    let program_order = dance(&moves)?;
    println!("after dancing, the programs are standing in order '{program_order}'");

    Ok(())
}

fn dance(moves: &[Move]) -> Result<String, String> {
    let mut programs: [char; 16] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ];
    for dance_move in moves {
        programs = run_move(*dance_move, programs)?;
    }
    Ok(programs.iter().collect())
}

fn run_move(dance_move: Move, mut programs: [char; 16]) -> Result<[char; 16], String> {
    match dance_move {
        Move::Spin(n) => {
            if n > programs.len() {
                Err(format!(
                    "unable to spin more than {} programs, tried to spin {n}",
                    programs.len()
                ))
            } else {
                let mut new: [char; 16] = ['☹'; 16];
                for (i, p) in programs[programs.len() - n..].iter().enumerate() {
                    new[i] = *p;
                }
                for (i, p) in programs[..programs.len() - n].iter().enumerate() {
                    new[n + i] = *p;
                }
                Ok(new)
            }
        }
        Move::Exchange(l, r) => {
            if l >= programs.len() || r >= programs.len() {
                Err(format!(
                    "unable to exchange {l} and {r}: index out of bounds"
                ))
            } else {
                programs.swap(l, r);
                Ok(programs)
            }
        }
        Move::Partner(p1, p2) => {
            let l = programs
                .iter()
                .position(|p| *p == p1)
                .ok_or_else(|| format!("unable to find program {p1}"))?;
            let r = programs
                .iter()
                .position(|p| *p == p2)
                .ok_or_else(|| format!("unable to find program {p2}"))?;
            programs.swap(l, r);
            Ok(programs)
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn parse_moves(s: &str) -> Result<Box<[Move]>, String> {
    s.trim().split(',').map(parse_move).collect()
}

fn parse_move(s: &str) -> Result<Move, String> {
    if let Some(n) = s.strip_prefix('s') {
        let n: usize = n
            .parse()
            .map_err(|e| format!("unable to parse number of programs for move '{s}': {e}"))?;
        Ok(Move::Spin(n))
    } else if let Some(params) = s.strip_prefix('p') {
        let (l, r) = params
            .split_once('/')
            .ok_or_else(|| format!("unable to split params for move '{s}'"))?;
        let l: char = l
            .chars()
            .next()
            .ok_or_else(|| format!("left side of params is empty for move '{s}'"))?;
        let r: char = r
            .chars()
            .next()
            .ok_or_else(|| format!("left side of params is empty for move '{s}'"))?;
        Ok(Move::Partner(l, r))
    } else if let Some(params) = s.strip_prefix('x') {
        let (l, r) = params
            .split_once('/')
            .ok_or_else(|| format!("unable to split params for move '{s}'"))?;
        let l: usize = l
            .parse()
            .map_err(|e| format!("unable to left index for move '{s}': {e}"))?;
        let r: usize = r
            .parse()
            .map_err(|e| format!("unable to right index for move '{s}': {e}"))?;
        Ok(Move::Exchange(l, r))
    } else {
        Err(format!("unknown move: '{s}'"))
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
