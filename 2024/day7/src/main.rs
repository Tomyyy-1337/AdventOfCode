fn main() {
    let contents = std::fs::read_to_string("input/puzzle").unwrap();

    let result = find_solution(&contents, part_1);
    println!("Part 1: {}", result);
    
    let result = find_solution(&contents, part_2);
    println!("Part 2: {}", result);
}

fn find_solution(contents: &str, filter: fn(u64, u64, &[u64]) -> bool) -> u64 {
    contents
        .lines()
        .map(parse_line)
        .filter(|(target, numbers)| filter(numbers[0], *target, &numbers[1..]))
        .map(|(target, _)| target)
        .sum()
}
    
fn part_1(acc: u64, target: u64, numbers: &[u64]) -> bool {
    match numbers {
        [] => acc == target,
        [a, b @ ..] => {
            part_1(acc + a, target, b) || 
            part_1(acc * a, target, b)
        }
    }
}

fn part_2(acc: u64, target: u64, numbers: &[u64]) -> bool {
    match numbers {
        [] => acc == target,
        [a, b @ ..] => {
            part_2(acc + a, target, b) || 
            part_2(acc * a, target, b) ||
            part_2(combine(acc, *a), target, b)
        }
    }
}

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let (target, numbers) = line.split_once(": ").unwrap();
    let target = target.parse::<u64>().unwrap();
    let numbers = numbers
        .split_whitespace()
        .flat_map(str::parse)
        .collect::<Vec<_>>();
    (target, numbers)
}

fn combine(a: u64, b: u64) -> u64 {
    let mut mult = 10;
    while mult <= b {
        mult *= 10;
    }
    a * mult + b
}