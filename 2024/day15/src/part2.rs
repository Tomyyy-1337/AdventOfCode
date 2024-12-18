use std::{collections::HashMap, hash::Hash};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_tuple(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq)]
enum Tile {
    Box,
    Wall,
}

pub struct Game {
    map: HashMap<(isize, isize), Tile>,
    instructions: Vec<Direction>,
    player_position: (isize, isize),
    width: usize,
}

impl Game {
    pub fn from_str(input: &str) -> Self {
        let (maze, instructions) = input.split_once("\r\n\r\n").unwrap();

        let width = maze.lines().next().unwrap().len();

        let mut map = HashMap::new();
        let mut player_position = (0, 0);

        for (y,line) in maze.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let x = x as isize * 2;
                let y = y as isize;
                match c {
                    '#' => {
                        map.insert((x, y), Tile::Wall);
                        map.insert((x + 1, y), Tile::Wall);
                    },
                    'O' => {
                        map.insert((x, y), Tile::Box);
                    },
                    '@' => {
                        player_position = (x as isize, y as isize);
                    }
                    _ => (),
                }
            }
        }

        let instructions: Vec<Direction> = instructions.chars().filter_map(Direction::from_char).collect();

        Game {
            map,
            instructions,
            player_position,
            width,
        }
    }

    fn use_instructions(&mut self) {
        for dir in &self.instructions {
            let (dx, dy) = dir.to_tuple();
            let (x, y) = self.player_position;
            let new_pos = (x as isize + dx, y as isize + dy);

            
        }
    }

}