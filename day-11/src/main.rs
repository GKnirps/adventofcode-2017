use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let (distance_to_child, max_distance) = track_distance(&content)?;

    println!("The distance to the child process is {distance_to_child}. The maximal distance was {max_distance}.");

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

fn track_distance(input: &str) -> Result<(u32, u32), String> {
    let (end, max) = input.trim().split(',').try_fold(
        (DirCount::default(), 0u32),
        |(mut count, max), dir| {
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
            let d = distance(count);
            Ok((count, d.max(max)))
        },
    )?;
    Ok((distance(end), max))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn track_distance_works_for_examples() {
        assert_eq!(track_distance("ne,ne,ne").unwrap(), (3, 3));
        assert_eq!(track_distance("ne,ne,sw,sw").unwrap(), (0, 2));
        assert_eq!(track_distance("ne,ne,s,s").unwrap(), (2, 2));
        assert_eq!(track_distance("se,sw,se,sw,sw").unwrap(), (3, 3));
        assert_eq!(track_distance("n,ne,se,s,sw,nw").unwrap(), (0, 2));
        assert_eq!(track_distance("n,n,sw,sw,ne,ne").unwrap(), (2, 2));
    }
}
