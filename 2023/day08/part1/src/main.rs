use std::collections::HashMap;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone)]
struct Path {
    left: String,
    right: String,
}

impl Path {
    pub fn get_path(&self, direction: Direction) -> &str {
        match direction {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

fn main() {
    let path = "input/puzzle.txt";
    let contents = std::fs::read_to_string(path).unwrap();

    let directions = contents.lines().next().unwrap()
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
    }).collect::<Vec<_>>(); 
    
    let lookup: HashMap<String, Path> = contents.lines().skip(2)
        .map(|line| {
            let line = line.chars().filter(|c| *c != '=' && *c != '(' && *c != ')' && *c != ',' ).collect::<String>();
            let mut iter = line.split_ascii_whitespace();
            let key = iter.next().unwrap().to_string();
            let left = iter.next().unwrap().to_string();
            let right = iter.next().unwrap().to_string();
            (key, Path{left, right})
        }).collect();

    let mut position = "AAA";
    let mut indx = 0;
    while position != "ZZZ" {
        let direction = directions[indx % directions.len()];
        position = lookup.get(position).unwrap().get_path(direction);
        indx += 1;
    }
    
    println!("{} steps required", indx);
}
