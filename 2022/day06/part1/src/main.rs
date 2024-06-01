use std::collections::HashSet;

fn main() {
    let path = "input/puzzle.txt";
    let contents = std::fs::read_to_string(path).unwrap().chars().collect::<Vec<char>>();

    let position = contents.windows(4)
        .position(|w| {
            w.iter().collect::<HashSet<_>>().len() == 4
        });

    println!("{:?}", position.unwrap() + 4);
}