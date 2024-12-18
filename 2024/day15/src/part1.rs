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
    Empty,
    Box,
    Wall,
}

pub struct Game {
    map: Vec<Tile>,
    instructions: Vec<Direction>,
    player_position: (isize, isize),
    width: usize,
}

impl Game {
    pub fn from_str(input: &str) -> Self {
        let (maze, instructions) = input.split_once("\r\n\r\n").unwrap();

        let width = maze.lines().next().unwrap().len();
        let height = maze.lines().count();

        let mut map = Vec::with_capacity(width * height);
        let mut player_position = (0, 0);

        for (y,line) in maze.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => map.push(Tile::Wall),
                    '.' => map.push(Tile::Empty),
                    'O' => map.push(Tile::Box),
                    '@' => {
                        map.push(Tile::Empty);
                        player_position = (x as isize, y as isize);
                    }
                    _ => panic!("Invalid character in maze: {}", c),
                }
            }
        }

        let instructions: Vec<Direction> = instructions.chars().filter_map(Direction::from_char).collect();

        Game {
            map,
            instructions,
            player_position,
            width
        }
    }
 
    pub fn solve(&mut self) {
        self.use_instructions();
        let score = self.get_score();
        println!("{}", score);
    }

    fn use_instructions(&mut self) {
        for dir in &self.instructions {
            let (dx, dy) = dir.to_tuple();
            let (x, y) = self.player_position;
            let new_pos = (x as isize + dx, y as isize + dy);
            let new_pos_indx = self.pos_to_indx(new_pos);

            match self.map[new_pos_indx] {
                Tile::Empty => self.player_position = new_pos,
                Tile::Wall => (),
                Tile::Box => {
                    for i in 1.. {
                        let new_box_pos = (new_pos.0 + dx * i, new_pos.1 + dy * i);
                        let new_box_pos_indx = self.pos_to_indx(new_box_pos);
                        match self.map[new_box_pos_indx] {
                            Tile::Empty => {
                                self.map[new_box_pos_indx] = Tile::Box;
                                self.map[new_pos_indx] = Tile::Empty;
                                self.player_position = new_pos;
                                break;
                            },
                            Tile::Wall => break,                            
                            Tile::Box => (),
                        }
                    }
                }
            }
        }
    }

    fn indx_to_pos(&self, index: usize) -> (isize, isize) {
        ((index % self.width) as isize, (index / self.width) as isize)
    }

    fn pos_to_indx(&self, pos: (isize, isize)) -> usize {
        (pos.1 * self.width as isize + pos.0) as usize 
    }

    fn get_score(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .filter(|(_, tile)| **tile == Tile::Box)
            .map(|(i, _)| self.indx_to_pos(i))
            .map(|(x, y)| (x + y * 100) as usize)
            .sum()
    }
}