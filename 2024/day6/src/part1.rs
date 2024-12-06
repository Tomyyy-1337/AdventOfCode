use std::marker::PhantomData;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Pending;
pub struct Solved;

#[derive(Debug)]
enum Cell {
    Wall,
    Empty { visited: bool },
}

#[derive(Debug)]
pub struct Maze<State = Pending> {
    maze: Vec<Cell>,
    width: usize,
    height: usize,
    guard_pos: (usize, usize),
    guard_direction: Direction,
    state: PhantomData<State>,
}

impl Maze {
    pub fn from_str(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let mut maze = Vec::with_capacity(width * height);
        let mut guard_pos = (0, 0);
        let mut guard_direction = Direction::Up;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => maze.push(Cell::Wall),
                    '.' => maze.push(Cell::Empty { visited: false }),
                    '^' | 'v' | '<' | '>' => {
                        maze.push(Cell::Empty { visited: true });
                        guard_pos = (x, y);
                        guard_direction = match c {
                            '^' => Direction::Up,
                            'v' => Direction::Down,
                            '<' => Direction::Left,
                            '>' => Direction::Right,
                            _ => unreachable!(),
                        };
                    }
                    _ => panic!("Invalid character in maze: {}", c),
                }
            }
        }
        Self {
            maze,
            width,
            height,
            guard_pos,
            guard_direction,
            state: PhantomData,
        }
    }
}

impl Maze<Pending> {
    fn get_cell_in_front(&self) -> &Cell {
        let (x, y) = self.guard_pos;
        let cell = match self.guard_direction {
            Direction::Up => self.maze.get((y - 1) * self.width + x),
            Direction::Down => self.maze.get((y + 1) * self.width + x),
            Direction::Left => self.maze.get(y * self.width + x - 1),
            Direction::Right => self.maze.get(y * self.width + x + 1),
        };
        cell.unwrap_or(&Cell::Empty { visited: false })
    }

    fn step(&mut self) -> bool {
        self.maze[self.guard_pos.1 * self.width + self.guard_pos.0] = Cell::Empty { visited: true };
        let (x, y) = self.guard_pos;
        match self.get_cell_in_front() {
            Cell::Wall => {
                self.guard_direction = match self.guard_direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
            }
            Cell::Empty { visited: _ } => {
                self.guard_pos = match self.guard_direction {
                    Direction::Up if y > 0 => (x, y - 1),
                    Direction::Down if y < self.height - 1 => (x, y + 1),
                    Direction::Left if x > 0 => (x - 1, y),
                    Direction::Right if x < self.width - 1 => (x + 1, y),
                    _ => return false,
                };
            }
        }
        true
    }

    pub fn solve(mut self) -> Maze<Solved> {
        while self.step() {}
        Maze {
            maze: self.maze,
            width: self.width,
            height: self.height,
            guard_pos: self.guard_pos,
            guard_direction: self.guard_direction,
            state: PhantomData,
        }
    }
}

impl Maze<Solved> {
    pub fn solution(&self) -> usize {
        self.maze
            .iter()
            .filter(|c| match c {
                Cell::Empty { visited: true } => true,
                _ => false,
            })
            .count()
    }
}
