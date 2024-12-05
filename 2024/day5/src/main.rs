use std::collections::HashMap;

fn main() {
    let contents = std::fs::read_to_string("input/puzzle").unwrap();

    part_1(&contents);
}
    
fn part_1(contents: &str) {
    let (part_1, part_2) = contents.split_once("\r\n\r\n").unwrap();
    
    let rules = part_1
        .lines()
        .map(|line| {
            let mut numbers = line.split("|").map(|x| x.trim().parse::<usize>().unwrap());
            let a = numbers.next().unwrap();
            let b = numbers.next().unwrap();
            (a, b)
        })
        .fold(HashMap::new(), |mut acc, (a, b)| {
            acc.entry(b).or_insert_with(Vec::new).push(a);
            acc
        });
    
    let messages = part_2
        .lines()
        .map(|line| line.split(",").map(|x| x.trim().parse::<usize>().unwrap()).collect::<Vec<_>>());
    
    let mut result = 0;
    
    'outer_loop: for message in messages {
        for (i, character) in message.iter().enumerate() {
            if let Some(previous) = rules.get(character) {
                if previous.iter().any(|previous_characters| !message[0..i].contains(&previous_characters) && message[i..].contains(&previous_characters)) {
                    continue 'outer_loop;
                }
            }
        }
        result += message[message.len()/2];
    }
    
    println!("Part 1: {}", result);
}
