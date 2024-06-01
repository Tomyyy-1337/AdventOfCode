use std::collections::HashSet;

fn main() {
    let path = "input/puzzle.txt";
    let position = std::fs::read_to_string(path).unwrap()
        .chars()
        .collect::<Vec<char>>()
        .windows(14)
        .position(|w| {
            w.iter().collect::<HashSet<_>>().len() == 14
        });

    println!("{:?}", position.unwrap() + 14);
}