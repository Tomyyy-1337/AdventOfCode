const DIRS: [Direction; 4] = [Direction::Right, Direction::Down, Direction::Left, Direction::Up];

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

#[derive(PartialEq, Eq, Clone)]
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

    pub fn from_directions(dirs: Vec<Direction>) -> Tile {
        if dirs.contains(&Direction::Up) && dirs.contains(&Direction::Down) {return Tile::new('|')}
        if dirs.contains(&Direction::Left) && dirs.contains(&Direction::Right) {return Tile::new('-')}
        if dirs.contains(&Direction::Up) && dirs.contains(&Direction::Right) {return Tile::new('L')}
        if dirs.contains(&Direction::Up) && dirs.contains(&Direction::Left) {return Tile::new('J')}
        if dirs.contains(&Direction::Down) && dirs.contains(&Direction::Left) {return Tile::new('7')}
        if dirs.contains(&Direction::Down) && dirs.contains(&Direction::Right) {return Tile::new('F')}
        panic!("Invalid directions");
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

    fn get_next_tile(&self, x: usize, y: usize, dir: &Direction) -> Option<&Tile> {
        let (dx, dy) = dir.as_tuple();
        self.get((x as i32 + dx) as usize, (y as i32 + dy) as usize)
    }

    fn next_direction(&self,from: &Direction, x: usize, y: usize) -> Direction {
        let tile = self.get(x, y);
        match tile {
            Some(Tile::Pipe(directions)) => {
                directions.iter().filter(|&d| d != &from.opposite()).next().unwrap().clone() 
            },
            Some(Tile::Start) => {
                for direction in DIRS {
                    if let Some(Tile::Pipe(directions)) = self.get_next_tile(x, y, &direction) {
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

    fn replace_start(&mut self) {
        let (dirs, i) = self.contents.iter()
            .enumerate()
            .filter_map(|(i,tile)| {
                if let Tile::Start = tile {
                    let dirs = DIRS.iter().filter_map(|direction| {
                        if let Some(Tile::Pipe(directions)) = self.get_next_tile(i % self.width, i / self.width, &direction) {
                            if directions.contains(&direction.opposite()) {
                                return Some(direction.clone());
                            }
                        }
                        None
                    }).collect::<Vec<_>>(); 
                    return Some((dirs, i))
                } 
                None
            }).next().unwrap();
        self.contents[i] = Tile::from_directions(dirs);
    }
}


fn main() {
    let path = "input/puzzle.txt";
    let mut map = Map::new(&std::fs::read_to_string(path).unwrap());

    let start_indx = map.contents.iter().position(|t| t == &Tile::Start).unwrap();
    let mut x = start_indx % map.width;
    let mut y = start_indx / map.width;
    let mut dir = map.next_direction(&Direction::Up, x, y);
    let mut loop_tiles: Vec<Tile> = vec![Tile::Ground; map.width * map.height];
    loop {
        loop_tiles[y * map.width + x] = map.get(x, y).unwrap().clone();
        dir = map.next_direction(&dir, x, y);
        let (dx, dy) = dir.as_tuple();
        x = (x as i32 + dx) as usize;
        y = (y as i32 + dy) as usize;
        if map.get(x, y) == Some(&Tile::Start) {
            break;
        }
    }
    map.contents = loop_tiles;
    map.replace_start();
    
    let num_enclosed: i32 = map.contents.iter()
        .enumerate()
        .filter(|(_, tile)| tile == &&Tile::Ground)
        .map(|(i, _)| count_walls(&map, i % map.width, i / map.width) % 2)
        .sum();

    println!("Enclosed Tiles: {}", num_enclosed);
}

fn count_walls(map: &Map, mut x: usize, y: usize) -> i32 {
    let mut walls = 0;
    while map.get(x, y).is_some() {
        x += 1;
        if let Some(Tile::Pipe(directions)) = map.get(x, y) {
            if directions.contains(&Direction::Up) && directions.contains(&Direction::Down) {
                walls += 1;
            } else if directions.contains(&Direction::Right) && !directions.contains(&Direction::Left) {
                let from_dir = directions.iter().filter(|&dir| dir != &Direction::Right).next().unwrap();
                let mut x_tmp = x + 1;
                while map.get(x_tmp, y).is_some() {
                    if let Some(Tile::Pipe(directions)) = map.get(x_tmp, y) {
                        if directions.contains(&from_dir.opposite()) && directions.contains(&Direction::Left) {
                            walls += 1;
                            break;
                        } else if !directions.contains(&Direction::Left) || !directions.contains(&Direction::Right) {
                            break;
                        }
                    }
                    x_tmp += 1;
                }
            }
        }    
    }
    walls
}