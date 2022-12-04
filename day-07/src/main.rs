use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let progs = parse_progs(&content)?;

    let root_name = find_root_program(&progs).ok_or_else(|| {
        "Did not find root program in input, are you sure this is a tree?".to_owned()
    })?;
    println!("The root program is '{root_name}'");

    Ok(())
}

struct Prog<'a> {
    name: &'a str,
    weight: u32,
    children: Vec<&'a str>,
}

fn parse_prog(line: &str) -> Result<Prog, String> {
    let mut split = line.splitn(2, " -> ");
    let (name, weight_s) = split
        .next()
        .expect("expected at least one element in a split string")
        .split_once(' ')
        .ok_or_else(|| {
            format!("Unable to parse line '{line}': unable to split name from weight")
        })?;
    let weight: u32 = weight_s
        .strip_prefix('(')
        .ok_or_else(|| format!("Missing open paranthesis before weight in '{line}'"))?
        .strip_suffix(')')
        .ok_or_else(|| format!("Missing closed paranthesis after weight in '{line}'"))?
        .parse::<u32>()
        .map_err(|e| format!("Unable to parse weight in line '{line}': {e}"))?;
    let children: Vec<&str> = split
        .next()
        .map(|l| l.split(", ").collect())
        .unwrap_or_else(Vec::new);
    Ok(Prog {
        name,
        weight,
        children,
    })
}

fn parse_progs(content: &str) -> Result<Vec<Prog>, String> {
    content.lines().map(parse_prog).collect()
}

// if there are multiple roots, it will arbitrarily pick one
fn find_root_program<'a>(progs: &'a [Prog]) -> Option<&'a str> {
    let mut candidates: HashSet<&str> = HashSet::with_capacity(progs.len());
    for prog in progs {
        candidates.insert(prog.name);
    }
    for prog in progs {
        for child in &prog.children {
            candidates.remove(child);
        }
    }
    candidates.iter().next().copied()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)
"#;

    #[test]
    fn find_root_program_works_for_example() {
        // given
        let progs = parse_progs(EXAMPLE).expect("Expected successful parsing");

        // when
        let root = find_root_program(&progs);

        // then
        assert_eq!(root, Some("tknk"));
    }
}
