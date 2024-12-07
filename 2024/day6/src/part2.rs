use crate::bool_array_8::BoolArray8;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn right_turn(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

pub enum SolveStatus {
    Pending,
    LoopFound,
    OutOfMaze,
}

#[derive(Clone)]
pub enum Cell {
    Wall{ visited: BoolArray8 },
    Empty,
}

pub struct Maze {
    maze: Vec<Cell>,
    width: usize,
    height: usize,
    guard_pos: (usize, usize),
    guard_direction: Direction,
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
                    '#' => maze.push(Cell::Wall{ visited: BoolArray8::default() }),
                    '.' => maze.push(Cell::Empty),
                    '^' | 'v' | '<' | '>' => {
                        maze.push(Cell::Empty);
                        guard_pos = (x, y);
                        guard_direction = match c {
                            '^' => Direction::Up,
                            'v' => Direction::Down,
                            '<' => Direction::Left,
                            '>' => Direction::Right,
                            _ => unreachable!(),
                        };    
                    },
                    _ => panic!("Invalid character in maze: {}", c),
                }
            }
        }
        Self { maze, width, height, guard_pos, guard_direction }	
    }

    fn get_cell_in_front_mut(&mut self) -> Option<&mut Cell> {
        let (x, y) = self.guard_pos;
        match self.guard_direction {
            Direction::Up if y > 0 => self.maze.get_mut((y - 1) * self.width + x),
            Direction::Down if y < self.height - 1 => self.maze.get_mut((y + 1) * self.width + x),
            Direction::Left if x > 0 => self.maze.get_mut(y * self.width + x - 1),
            Direction::Right if x < self.width - 1 => self.maze.get_mut(y * self.width + x + 1),
            _ => None,
        }
    }

    pub fn step(&mut self) -> SolveStatus {
        let (x, y) = self.guard_pos;
        let guard_direction = self.guard_direction;
        match self.get_cell_in_front_mut() {
            Some(Cell::Wall{ visited }) if visited.get(guard_direction as usize) => {
                return SolveStatus::LoopFound
            },
            Some(Cell::Wall{ visited }) => {
                visited.set(guard_direction as usize, true);
                self.guard_direction = guard_direction.right_turn();
            },
            Some(Cell::Empty) => {
                self.guard_pos = match self.guard_direction {
                    Direction::Up => (x, y - 1),
                    Direction::Down => (x, y + 1),
                    Direction::Left => (x - 1, y),
                    Direction::Right => (x + 1, y),
                };
            },
            None => return SolveStatus::OutOfMaze,
        }
        SolveStatus::Pending
    }

    pub fn has_loop(mut self) -> bool { 
        loop {
            match self.step() {
                SolveStatus::Pending => continue,
                SolveStatus::LoopFound => return true,
                SolveStatus::OutOfMaze => return false,
            }
        } 
    }

    pub fn create_with_extra_wall(&self, index: usize) -> Option<Self> {
        if self.guard_pos == (index % self.width, index / self.width) {
            return None;
        }
        match self.maze.get(index) {
            Some(Cell::Empty) => {
                let mut maze = self.maze.clone();
                maze[index] = Cell::Wall{ visited: BoolArray8::default() };
                Some(Self {
                    maze,
                    ..*self
                })
            },
            _ => return None,
        }
    } 

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        self.maze.get(y * self.width + x)
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn get_guard_pos(&self) -> (usize, usize) {
        self.guard_pos
    }
}