use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let connections = parse_connections(&content)?;

    let zero_group_size = reachable_processes(&connections, 0);
    println!("In the group with program 0 are {zero_group_size} programs");

    let n_groups = count_groups(&connections);
    println!("There are {n_groups} groups");

    Ok(())
}

fn count_groups(connections: &[Box<[usize]>]) -> usize {
    let mut queue: Vec<usize> = Vec::with_capacity(connections.len());
    let mut visited: Vec<bool> = vec![false; connections.len()];
    let mut group_count: usize = 0;

    while let Some((start, _)) = visited.iter().enumerate().find(|(_, v)| !**v) {
        group_count += 1;
        queue.push(start);
        while let Some(current) = queue.pop() {
            if current >= connections.len() {
                eprintln!("tried to connect to unknown program {current}, ignoring connection");
                continue;
            }
            if !visited[current] {
                for pipe in &connections[current] {
                    queue.push(*pipe);
                }
            }
            visited[current] = true;
        }
    }
    group_count
}

fn reachable_processes(connections: &[Box<[usize]>], start: usize) -> usize {
    let mut queue: Vec<usize> = Vec::with_capacity(connections.len());
    let mut visited: Vec<bool> = vec![false; connections.len()];

    queue.push(start);
    while let Some(current) = queue.pop() {
        if current >= connections.len() {
            eprintln!("tried to connect to unknown program {current}, ignoring connection");
            continue;
        }
        if !visited[current] {
            for pipe in &connections[current] {
                queue.push(*pipe);
            }
        }
        visited[current] = true;
    }
    visited.iter().filter(|v| **v).count()
}

fn parse_connections(input: &str) -> Result<Box<[Box<[usize]>]>, String> {
    input
        .lines()
        .map(parse_line)
        .enumerate()
        .map(|(i, connections)| {
            let (pid, pipes) = connections?;
            if pid != i {
                return Err(format!(
                    "lines are not in order, expected {i} but got {pid}"
                ));
            }
            Ok(pipes)
        })
        .collect()
}

fn parse_line(line: &str) -> Result<(usize, Box<[usize]>), String> {
    let (pid, pipes) = line
        .split_once(" <-> ")
        .ok_or_else(|| format!("unable to parse line '{line}'"))?;
    let pid: usize = pid
        .parse()
        .map_err(|e| format!("unable to parse program ID '{pid}': {e}"))?;
    let pipes: Box<[usize]> = pipes
        .split(", ")
        .map(|s| {
            s.parse::<usize>()
                .map_err(|e| format!("unable to parse target '{s}' in line {pid}: {e}"))
        })
        .collect::<Result<_, _>>()?;
    Ok((pid, pipes))
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
"#;

    #[test]
    fn reachable_processes_works_for_example() {
        // given
        let connections = parse_connections(EXAMPLE).expect("expected successful parsing");

        // when
        let n = reachable_processes(&connections, 0);

        // then
        assert_eq!(n, 6);
    }

    #[test]
    fn count_groups_works_for_example() {
        // given
        let connections = parse_connections(EXAMPLE).expect("expected successful parsing");

        // when
        let n = count_groups(&connections);

        // then
        assert_eq!(n, 2);
    }
}
