use hashbrown::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    MirrorLeft,
    MirrorRight,
    SplitHorrizontal,
    SplitVertical,
}

impl Tile {
    fn from_char(c: char) -> Option<Tile> {
        match c {
            '.' => Some(Tile::Empty),
            '/' => Some(Tile::MirrorRight),
            '\\' => Some(Tile::MirrorLeft),
            '-' => Some(Tile::SplitHorrizontal),
            '|' => Some(Tile::SplitVertical),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Status {
    Energized,
    Cold,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_tuple(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    tiles: Vec<Tile>,
    energy: Vec<Status>,
    tested_paths: HashSet<(i32, i32, Direction)>,
    width: usize,
    height: usize,
}

impl Grid {
    fn from_string(str: &str) -> Self {
        let tiles: Vec<Tile> = str.chars()
            .filter_map(Tile::from_char)
            .collect();
        Grid {
            energy: vec![Status::Cold; tiles.len()],
            tiles,
            tested_paths: HashSet::new(),
            width: str.lines().next().unwrap().len(),
            height: str.lines().count(),
        }
    }

    fn get_tile(&self, x: i32, y: i32) -> Option<Tile> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            None
        } else {
            Some(self.tiles[(y * self.width as i32 + x) as usize])
        }
    }

    fn next_beam_step(&mut self, x: i32, y: i32, direction: Direction) {
        if self.tested_paths.contains(&(x, y, direction)) {
            return;
        }
        self.tested_paths.insert((x, y, direction));

        if let Some(tile) = self.get_tile(x, y) {
            self.energy[(y * self.width as i32 + x) as usize] = Status::Energized;
            match tile {
                Tile::MirrorLeft | Tile::MirrorRight => {
                    let mut dir = match direction {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    };
                    if tile == Tile::MirrorRight {
                        dir = dir.opposite();
                    }
                    let (dx, dy) = dir.to_tuple();
                    self.next_beam_step(x + dx, y + dy, dir);
                },
                Tile::SplitHorrizontal if direction == Direction::Up || direction == Direction::Down => {
                    let (dx, dy) = Direction::Left.to_tuple();
                    self.next_beam_step(x + dx, y + dy, Direction::Left);
                    let (dx, dy) = Direction::Right.to_tuple();
                    self.next_beam_step(x + dx, y + dy, Direction::Right);
                },
                Tile::SplitVertical if direction == Direction::Left || direction == Direction::Right => {
                    let (dx, dy) = Direction::Up.to_tuple();
                    self.next_beam_step(x + dx, y + dy, Direction::Up);
                    let (dx, dy) = Direction::Down.to_tuple();
                    self.next_beam_step(x + dx, y + dy, Direction::Down);
                },
                _ => {
                    let (dx, dy) = direction.to_tuple();
                    self.next_beam_step(x + dx, y + dy, direction);
                },
            }
       } 
    }
}

fn main() {
    let path ="input/puzzle.txt";

    let grid = Grid::from_string(&std::fs::read_to_string(path).unwrap());

    let mut result = 0;
    for x in 0..grid.width {
        let mut current_grid = grid.clone();
        current_grid.next_beam_step(x as i32, 0, Direction::Down);
        result = result.max(current_grid.energy.iter().filter(|x| **x == Status::Energized).count());
        let mut current_grid = grid.clone();
        current_grid.next_beam_step(x as i32, grid.height as i32 - 1, Direction::Up);
        result = result.max(current_grid.energy.iter().filter(|x| **x == Status::Energized).count());
    }
    for y in 0..grid.height {
        let mut current_grid = grid.clone();
        current_grid.next_beam_step(0, y as i32, Direction::Right);
        result = result.max(current_grid.energy.iter().filter(|x| **x == Status::Energized).count());
        let mut current_grid = grid.clone();
        current_grid.next_beam_step(grid.width as i32 - 1, y as i32, Direction::Left);
        result = result.max(current_grid.energy.iter().filter(|x| **x == Status::Energized).count());
    }

    println!("Sum: {}", result);
}