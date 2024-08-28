use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let dir_count = count_directions(&content)?;

    let distance_to_child = distance(dir_count);
    println!("The distance to the child process is {distance_to_child}");

    Ok(())
}

fn distance(mut dir_count: DirCount) -> u32 {
    for _ in 0..2 {
        let nwne = dir_count.ne.min(dir_count.nw);
        dir_count.ne -= nwne;
        dir_count.nw -= nwne;
        dir_count.n += nwne;

        let nse = dir_count.n.min(dir_count.se);
        dir_count.n -= nse;
        dir_count.se -= nse;
        dir_count.ne += nse;

        let nes = dir_count.ne.min(dir_count.s);
        dir_count.ne -= nes;
        dir_count.s -= nes;
        dir_count.se += nes;

        let swse = dir_count.se.min(dir_count.sw);
        dir_count.se -= swse;
        dir_count.sw -= swse;
        dir_count.s += swse;

        let snw = dir_count.s.min(dir_count.nw);
        dir_count.s -= snw;
        dir_count.nw -= snw;
        dir_count.sw += snw;

        let swn = dir_count.sw.min(dir_count.n);
        dir_count.sw -= swn;
        dir_count.n -= swn;
        dir_count.nw += swn;

        let sn = dir_count.n.min(dir_count.s);
        dir_count.n -= sn;
        dir_count.s -= sn;

        let senw = dir_count.se.min(dir_count.nw);
        dir_count.se -= senw;
        dir_count.nw -= senw;

        let swne = dir_count.sw.min(dir_count.ne);
        dir_count.sw -= swne;
        dir_count.ne -= swne;
    }

    dir_count.se + dir_count.s + dir_count.sw + dir_count.nw + dir_count.n + dir_count.ne
}

#[derive(Copy, Clone, Debug, Default)]
struct DirCount {
    se: u32,
    s: u32,
    sw: u32,
    nw: u32,
    n: u32,
    ne: u32,
}

fn count_directions(input: &str) -> Result<DirCount, String> {
    input
        .trim()
        .split(',')
        .try_fold(DirCount::default(), |mut count, dir| {
            match dir {
                "se" => {
                    count.se += 1;
                }
                "s" => {
                    count.s += 1;
                }
                "sw" => {
                    count.sw += 1;
                }
                "nw" => {
                    count.nw += 1;
                }
                "n" => {
                    count.n += 1;
                }
                "ne" => {
                    count.ne += 1;
                }
                _ => {
                    return Err(format!("unknown direction: '{dir}'"));
                }
            }
            Ok(count)
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn distance_works_for_examples() {
        assert_eq!(distance(count_directions("ne,ne,ne").unwrap()), 3);
        assert_eq!(distance(count_directions("ne,ne,sw,sw").unwrap()), 0);
        assert_eq!(distance(count_directions("ne,ne,s,s").unwrap()), 2);
        assert_eq!(distance(count_directions("se,sw,se,sw,sw").unwrap()), 3);
        assert_eq!(distance(count_directions("n,ne,se,s,sw,nw").unwrap()), 0);
        assert_eq!(distance(count_directions("n,n,sw,sw,ne,ne").unwrap()), 2);
    }
}
