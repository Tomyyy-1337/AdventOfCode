use std::collections::HashMap;

enum Direction {
    Left,
    Right,
}

struct Path {
    left: String,
    right: String,
}

impl Path {
    pub fn get_path(&self, direction: &Direction) -> &str {
        match direction {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

fn ggt(a: u64, b: u64) -> u64 {
    if b == 0 { return a }
    ggt(b, a % b)
}

fn kgv(a: u64, b: u64) -> u64 {
    a * b / ggt(a, b)
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

    let result = contents.lines().skip(2)
        .filter(|line| line.chars().nth(2) == Some('A'))
        .map(|line| line.get(0..=2).unwrap())
        .map(| mut position | {
            let mut indx = 0;
            while position.chars().nth(2) != Some('Z') {
                let direction = &directions[indx % directions.len()];
                position = lookup.get(position).unwrap().get_path(direction);
                indx += 1;
            }
            indx as u64
        }).fold(1, |acc, num| kgv(acc, num));
    
    println!("{:?} steps required", result);
}