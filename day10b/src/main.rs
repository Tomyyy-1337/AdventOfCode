use core::num;
use std::iter;

#[derive(PartialEq, Eq, Clone)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn as_tuple(&self) -> (i32, i32) {
        match self {
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Up => (0, -1),
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
        }
    }
}

#[derive(PartialEq, Eq)]
enum Tile {
    Pipe(Vec<Direction>),
    Ground,
    Start,
}

impl Tile {
    pub fn new(c: char) -> Tile {
        match c {
            '|' => Tile::Pipe(vec![Direction::Up, Direction::Down]),
            '-' => Tile::Pipe(vec![Direction::Left, Direction::Right]),
            'L' => Tile::Pipe(vec![Direction::Up, Direction::Right]),
            'J' => Tile::Pipe(vec![Direction::Up, Direction::Left]),
            '7' => Tile::Pipe(vec![Direction::Down, Direction::Left]),
            'F' => Tile::Pipe(vec![Direction::Down, Direction::Right]),
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!("Invalid pipe character: {}", c),
        }
    }
}

struct Map {
    width: usize,
    height: usize,
    contents: Vec<Tile>,
}

impl Map {
    fn new(contents: &str) -> Self {
        Self {
            width:    contents.lines().next().unwrap().len(),
            height:   contents.lines().count(),
            contents: contents.chars().filter(|&c| c != '\n' && c != '\r').map(|c| Tile::new(c)).collect(),
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.contents.get(y * self.width + x)
    }

    fn set(&mut self, x: usize, y: usize, tile: Tile) {
        self.contents[y * self.width + x] = tile;
    }

    fn get_next(&self, x: usize, y: usize, dir: &Direction) -> Option<&Tile> {
        let (dx, dy) = dir.as_tuple();
        self.get((x as i32 + dx) as usize, (y as i32 + dy) as usize)
    }

    fn get_connected_pipes(&self,from: &Direction, x: usize, y: usize) -> Direction {
        let tile = self.get(x, y);
        match tile {
            Some(Tile::Pipe(directions)) => {
                directions.iter().filter(|&d| d != &from.opposite()).next().unwrap().clone() 
            },
            Some(Tile::Start) => {
                for direction in vec![Direction::Right, Direction::Down, Direction::Left, Direction::Up] {
                    if let Some(Tile::Pipe(directions)) = self.get_next(x, y, &direction) {
                        if directions.contains(&direction.opposite()) {
                            return direction;
                        }
                    }
                }
                panic!("Could not find connected pipe from start tile");
            },
            _ => panic!("Could not find connected pipe"),
        }
    }
}


fn main() {
    let path = "input/test.txt";
    let map = Map::new(&std::fs::read_to_string(path).unwrap());
    
    let mut num_enclosed = 0;
    for x in 0..map.width {
        for y in 0..map.height {
            if let Some(Tile::Ground) = map.get(x, y) {
                let mut x_pos = x;
                let mut wall_counter = 0;
                while let Some(tile) = map.get(x_pos, y) {
                    if let Tile::Pipe(dirs) = tile {
                        if dirs.contains(&Direction::Up) && dirs.contains(&Direction::Down) { 
                            wall_counter += 1
                        } else if dirs.contains(&Direction::Right) { 
                            let direction = dirs.iter().filter(|&d| d != &Direction::Right).next().unwrap().clone();
                            while let Some(Tile::Pipe(dirs)) = map.get(x_pos, y) {
                                if dirs.contains(&Direction::Left) {
                                    if dirs.contains(&direction.opposite()) {
                                        wall_counter += 1;
                                        break;
                                    }
                                    x_pos += 1;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    x_pos += 1;
                }
                if wall_counter % 2 == 1 {
                    num_enclosed += 1;
                }
            }
        }
    }

    println!("Sum: {}", num_enclosed);
}