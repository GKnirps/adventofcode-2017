use std::collections::{HashMap, HashSet};
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

    if let Some(corrected_weight) = find_corrected_weight(&progs, root_name) {
        println!("The corrected weight is {corrected_weight}");
    } else {
        println!("Either there is no weight to be corrected or something is wrong with the tree datastcutures");
    }

    Ok(())
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
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

fn calculate_prog_weights<'a, 'w>(
    progs: &'a HashMap<&'a str, &Prog>,
    root: &'a str,
    weights: &'w mut HashMap<&'a str, u32>,
) -> u32 {
    let mut weight = 0;
    if let Some(prog) = progs.get(root) {
        weight += prog.weight;
        for child in &prog.children {
            weight += calculate_prog_weights(progs, child, weights);
        }
        weights.insert(root, weight);
    }
    weight
}

fn find_corrected_weight_recursive(
    prog: &Prog,
    progs_by_name: &HashMap<&str, &Prog>,
    weights: &HashMap<&str, u32>,
) -> Option<u32> {
    if prog.children.is_empty() {
        return None;
    }
    // if a child cannot be found, something is wrong with the tree data and we won't
    // look any further
    let ref_prog = progs_by_name.get(prog.children[0])?;
    let ref_weight = *weights.get(ref_prog.name)?;

    let mut different: Option<(&Prog, u32)> = None;
    for child_name in &prog.children[1..] {
        let child_prog = progs_by_name.get(child_name)?;
        let child_weight = weights.get(child_prog.name)?;

        if let Some((oref_prog, oref_weight)) = different {
            // Oh, btw, this will panic if the children of the node alone are heavier than
            // the other two because then we would require a negative weight
            return Some(if oref_weight == *child_weight {
                // look recursively: if everything is balanced there, you have the wrong weight
                find_corrected_weight_recursive(ref_prog, progs_by_name, weights)
                    .unwrap_or(oref_weight - (ref_weight - ref_prog.weight))
            } else {
                find_corrected_weight_recursive(oref_prog, progs_by_name, weights)
                    .unwrap_or(ref_weight - (oref_weight - oref_prog.weight))
            });
        } else if ref_weight != *child_weight {
            different = Some((child_prog, *child_weight));
        }
    }
    None
}

fn find_corrected_weight(progs: &[Prog], root: &str) -> Option<u32> {
    let progs_by_name: HashMap<&str, &Prog> = progs.iter().map(|prog| (prog.name, prog)).collect();
    let mut weights: HashMap<&str, u32> = HashMap::with_capacity(progs.len());
    calculate_prog_weights(&progs_by_name, root, &mut weights);

    let root_prog = progs_by_name.get(root)?;
    find_corrected_weight_recursive(root_prog, &progs_by_name, &weights)
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

    #[test]
    fn find_corrected_weight_works_for_example() {
        // given
        let progs = parse_progs(EXAMPLE).expect("Expected successful parsing");

        // when
        let weight = find_corrected_weight(&progs, "tknk");

        // then
        assert_eq!(weight, Some(60));
    }
}
