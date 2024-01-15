use hashbrown::HashSet;
use priority_queue::PriorityQueue;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

    fn not_backwards(&self) -> Vec<Direction> {
        match self {
            Direction::Up => vec![Direction::Left, Direction::Right, Direction::Up],
            Direction::Down => vec![Direction::Left, Direction::Right, Direction::Down],
            Direction::Left => vec![Direction::Up, Direction::Down, Direction::Left],
            Direction::Right => vec![Direction::Up, Direction::Down, Direction::Right],
        }
    }

    fn orthogonal(&self) -> Vec<Direction> {
        match self {
            Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
        }
    }
}

#[derive(Clone)]
struct Cell {
    value: i32,
    visited: HashSet<(Direction, i32)>
}

impl Cell {
    fn from_char(c: char) -> Option<Cell> {
        match c {
            '0'..='9' => Some(Cell { value: c.to_digit(10).unwrap() as i32, visited: HashSet::new()}),
            _ => return None,
        }
    }
}

#[derive(Clone)]
struct Board {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Board {
    fn from_str(str: &str) -> Board {
        Board {
            cells: str.chars().into_iter().filter_map(Cell::from_char).collect(),
            width: str.lines().next().unwrap().len(),
            height: str.lines().count(),
        }
    }

    fn get_cell(&self, x: i32, y: i32) -> Option<&Cell> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }
        Some(&self.cells[(y * self.width as i32 + x) as usize])
    }

    fn find_best_path(&mut self, x: i32, y: i32, current_loss: i32, direction: Direction, dir_count: i32) -> i32 {
        let mut queue: PriorityQueue<(i32,i32,i32, Direction, i32),i32> = PriorityQueue::new();
        queue.push((x, y, current_loss, direction, dir_count), 0);
        
        while let Some(((x, y, current_loss, direction, dir_count), _)) = queue.pop() {
            if x == self.width as i32 - 1 && y == self.height as i32 - 1 {
                return current_loss;
            }
            if self.get_cell(x, y).unwrap().visited.contains(&(direction, dir_count)) {
                continue;
            }
            let dirs = if dir_count >= 3 {
                direction.orthogonal()
            } else {
                direction.not_backwards()
            };
            for dir in dirs {
                let (dx, dy) = dir.to_tuple();
                if let Some(cell) = self.get_cell(x + dx, y + dy) {
                    let new_loss = current_loss + cell.value;
                    let dir_count = if dir == direction { dir_count + 1 } else { 1 };
                    queue.push((x + dx, y + dy, new_loss, dir, dir_count), -new_loss);
                }
            }
            self.cells[(y * self.width as i32 + x) as usize].visited.insert((direction, dir_count));
        }
        0
    }
}

fn main() {
    let path = "input/puzzle.txt";

    let loss = Board::from_str(&std::fs::read_to_string(path).unwrap())
        .find_best_path(0, 0, 0, Direction::Down, 0);

    println!("Loss: {}", loss);
}