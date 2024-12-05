use std::collections::HashMap;

fn main() {
    let contents = std::fs::read_to_string("input/puzzle").unwrap();

    part_1(&contents);
    part_2(&contents);
}
    
fn part_1(contents: &str) {
    let (rules, messages) = extract_data(contents);
    
    let result: usize = messages
        .iter()
        .filter(|message| is_valid(message, &rules))
        .map(|message| message[message.len() / 2])
        .sum();    
    
    println!("Part 1: {}", result);
}

fn part_2(contents: &str) {
    let (rules, mut messages) = extract_data(contents);
    
    let result = messages
        .iter_mut()
        .filter(|message| !is_valid(message, &rules))
        .map(|message| {
            message.sort_unstable_by(|a, b| custom_compare(a, b, &rules));
            message[message.len() / 2]
        })
        .sum::<usize>();
    
    println!("Part 2: {}", result);
}

fn extract_data(contents: &str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let (part_1, part_2) = contents.split_once("\r\n\r\n").unwrap();
    
    let rules = part_1
        .lines()
        .map(|line| {
            let mut numbers = line.split("|").map(|x| x.trim().parse::<usize>().unwrap());
            (numbers.next().unwrap(), numbers.next().unwrap())
        })
        .fold(HashMap::new(), |mut acc, (a, b)| {
            acc.entry(b).or_insert_with(Vec::new).push(a);
            acc
        });
    
    let messages = part_2
        .lines()
        .map(|line| line.split(",").map(|x| x.trim().parse::<usize>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    (rules, messages)
}

fn is_valid(message: &Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> bool {
    for (i, character) in message.iter().enumerate() {
        if let Some(previous) = rules.get(character) {
            if previous.iter().any(|prev| !message[0..i].contains(&prev) && message[i..].contains(&prev)) {
                return false;
            }
        }
    }
    true
}

fn custom_compare(a: &usize, b: &usize, rules: &HashMap<usize, Vec<usize>>) -> std::cmp::Ordering {
    if let Some(previous) = rules.get(b) {
        if previous.contains(a) {
            return std::cmp::Ordering::Less;
        }
    }
    std::cmp::Ordering::Equal
}