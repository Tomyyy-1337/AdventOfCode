#[derive(Debug)]
struct Cargo {
    stacks: Vec<Vec<char>>,
}

impl Cargo {
    fn from_string(str: &str) -> Self {
        let stack_count = str.lines().last().unwrap().split_ascii_whitespace().count();
        let mut stacks = Vec::with_capacity(stack_count);
        stacks.resize_with(stack_count, Vec::new);
        str.lines()
            .rev()
            .skip(1)
            .for_each(|line| line.chars()
                .enumerate()
                .filter(|&(_, c)| c != ' ' && c != '[' && c != ']' )
                .for_each(|(i, c)| stacks[i / 4].push(c))
            );
        Cargo { stacks }
    }
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn from_string(str: &str) -> Self {
        let mut parts = str.split_ascii_whitespace()
            .enumerate()
            .filter(|(i,_)| i % 2 == 1)
            .map(|(_, e)| e);
        Instruction {
            quantity: parts.next().unwrap().parse().unwrap(),
            from: parts.next().unwrap().parse().unwrap(),
            to: parts.next().unwrap().parse().unwrap(),
        }
    }
}

fn main() {
    let path = "input/puzzle.txt";

    let contents = std::fs::read_to_string(path).unwrap();
    let (stacks, instructions) = contents.split_once("\r\n\r\n").unwrap();

    let mut cargo = Cargo::from_string(stacks);
    let instructions: Vec<Instruction> = instructions.lines()
        .map(Instruction::from_string)
        .collect();

    for Instruction { quantity, from, to } in instructions {
        let split_indx = cargo.stacks[from - 1].len() - quantity;
        let moving = cargo.stacks[from - 1].split_off(split_indx);
        cargo.stacks[to - 1].extend(moving)
    }

    let message: String = cargo.stacks.iter()
        .map(|stack| stack.last().unwrap_or(&' '))
        .collect();

    println!("{}", message);
}