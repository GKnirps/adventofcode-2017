use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let layers = parse_input(&content)?;

    let severity = severity_of_immediate_trip(&layers);
    println!("The severity of the trip when you start right away is {severity}");

    Ok(())
}

fn severity_of_immediate_trip(layers: &[(u32, u32)]) -> u32 {
    layers
        .iter()
        .map(|(layer, depth)| {
            if *depth < 1 || layer % ((depth - 1) * 2) == 0 {
                layer * depth
            } else {
                0
            }
        })
        .sum()
}

fn parse_input(input: &str) -> Result<Box<[(u32, u32)]>, String> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Result<(u32, u32), String> {
    let (layer, depth) = line
        .split_once(": ")
        .ok_or_else(|| format!("unable to split line '{line}'"))?;
    let layer: u32 = layer
        .parse()
        .map_err(|e| format!("unable to parse layer in line '{line}': {e}"))?;
    let depth: u32 = depth
        .parse()
        .map_err(|e| format!("unable to parse depth in line '{line}': {e}"))?;
    Ok((layer, depth))
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"0: 3
1: 2
4: 4
6: 4
"#;

    #[test]
    fn severity_of_immediate_trip_works_for_example() {
        // given
        let layers = parse_input(EXAMPLE).expect("expected successful parsing");

        // when
        let severity = severity_of_immediate_trip(&layers);

        // then
        assert_eq!(severity, 24);
    }
}
