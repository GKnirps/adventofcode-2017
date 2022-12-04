use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let instructions = parse_instructions(&content)?;

    let (registers, max_reg_total) = execute_instructions(&instructions);

    if let Some(max_register) = max_register_value(&registers) {
        println!("Highest register value is {max_register}");
    } else {
        println!("There are no known registers after execution");
    }
    println!("The maximum value that any register had during execution was {max_reg_total}");

    Ok(())
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Instruction<'a> {
    reg: &'a str,
    op: Operation,
    value: i32,
    cond_reg: &'a str,
    cond_comp: Comp,
    cond_value: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Operation {
    Inc,
    Dec,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Comp {
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}

impl Comp {
    fn compare(self, left: i32, right: i32) -> bool {
        match self {
            Comp::Eq => left == right,
            Comp::Ne => left != right,
            Comp::Lt => left < right,
            Comp::Gt => left > right,
            Comp::Le => left <= right,
            Comp::Ge => left >= right,
        }
    }
}

fn parse_instructions(content: &str) -> Result<Vec<Instruction>, String> {
    content.lines().map(parse_instruction).collect()
}

fn parse_instruction(line: &str) -> Result<Instruction, String> {
    let (operation, condition) = line
        .split_once(" if ")
        .ok_or_else(|| format!("Unable to parse line '{line}': no condition statement found"))?;
    let mut op_split = operation.splitn(3, ' ');
    let reg = op_split
        .next()
        .ok_or_else(|| format!("Unable to parse line '{line}': no register"))?;
    let op = op_split
        .next()
        .ok_or_else(|| format!("Unable to parse line '{line}': missing operation"))
        .and_then(parse_operation)?;
    let value = op_split
        .next()
        .ok_or_else(|| format!("Unable to parse line '{line}': missing value"))
        .and_then(|v| {
            v.parse::<i32>()
                .map_err(|e| format!("Unable to parse value '{v}': {e}"))
        })?;

    let mut cond_split = condition.splitn(3, ' ');
    let cond_reg = cond_split
        .next()
        .ok_or_else(|| format!("Unable to parse line '{line}': no condition register"))?;
    let cond_comp = cond_split
        .next()
        .ok_or_else(|| format!("Unable to parse line '{line}': missing comparator"))
        .and_then(parse_comp)?;
    let cond_value = cond_split
        .next()
        .ok_or_else(|| format!("Unable to parse line '{line}': missing condition value"))
        .and_then(|v| {
            v.parse::<i32>()
                .map_err(|e| format!("Unable to parse value '{v}': {e}"))
        })?;

    Ok(Instruction {
        reg,
        op,
        value,
        cond_reg,
        cond_comp,
        cond_value,
    })
}

fn parse_operation(input: &str) -> Result<Operation, String> {
    match input {
        "inc" => Ok(Operation::Inc),
        "dec" => Ok(Operation::Dec),
        _ => Err(format!("invalid operation: '{input}'")),
    }
}

fn parse_comp(input: &str) -> Result<Comp, String> {
    match input {
        "==" => Ok(Comp::Eq),
        "!=" => Ok(Comp::Ne),
        "<" => Ok(Comp::Lt),
        ">" => Ok(Comp::Gt),
        "<=" => Ok(Comp::Le),
        ">=" => Ok(Comp::Ge),
        _ => Err(format!("invalid comparator: '{input}'")),
    }
}

fn execute_instructions<'a>(instructions: &'a [Instruction]) -> (HashMap<&'a str, i32>, i32) {
    let mut registers: HashMap<&str, i32> = HashMap::with_capacity(128);
    let mut max_reg: i32 = 0;
    for instruction in instructions {
        if let Some(changed) = execute_instruction(instruction, &mut registers) {
            max_reg = max_reg.max(changed);
        }
    }
    (registers, max_reg)
}

fn execute_instruction<'a, 'reg>(
    instruction: &'a Instruction,
    registers: &'reg mut HashMap<&'a str, i32>,
) -> Option<i32> {
    let cond_reg_value = registers.get(instruction.cond_reg).copied().unwrap_or(0);
    if instruction
        .cond_comp
        .compare(cond_reg_value, instruction.cond_value)
    {
        let register = registers.entry(instruction.reg).or_insert(0);
        *register += match instruction.op {
            Operation::Inc => instruction.value,
            Operation::Dec => -instruction.value,
        };
        Some(*register)
    } else {
        None
    }
}

fn max_register_value(registers: &HashMap<&str, i32>) -> Option<i32> {
    registers.values().max().copied()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
"#;

    #[test]
    fn execute_instructions_works_correctly() {
        // given
        let inst = parse_instructions(EXAMPLE).expect("Expected successful parsing");

        // when
        let (registers, max_total) = execute_instructions(&inst);
        let max = max_register_value(&registers);

        // then
        assert_eq!(max, Some(1));
        assert_eq!(max_total, 10);
    }
}
