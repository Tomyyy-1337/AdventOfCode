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
    contents: Vec<Tile>,
}

impl Map {
    fn new(contents: &str) -> Self {
        Self {
            width:    contents.lines().next().unwrap().len(),
            contents: contents.chars().filter(|&c| c != '\n' && c != '\r').map(|c| Tile::new(c)).collect(),
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.contents.get(y * self.width + x)
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
                for direction in vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
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
    let path = "input/puzzle.txt";
    let map = Map::new(&std::fs::read_to_string(path).unwrap());
    
    let start_indx = map.contents.iter().position(|t| t == &Tile::Start).unwrap();
    let mut x = start_indx % map.width;
    let mut y = start_indx / map.width;
    let mut dir = map.get_connected_pipes(&Direction::Up, x, y);
    for steps in 1.. {
        dir = map.get_connected_pipes(&dir, x, y);
        let (dx, dy) = dir.as_tuple();
        x = (x as i32 + dx) as usize;
        y = (y as i32 + dy) as usize;
        if map.get(x, y) == Some(&Tile::Start) {
            println!("Steps: {}", steps / 2);
            break;
        }
    }
}