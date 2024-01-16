enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
    fn to_tuple(&self, distance: i32) -> (i32, i32) {
        match self {
            Direction::Up => (0, 1 * distance),
            Direction::Down => (0, -1 * distance),
            Direction::Left => (-1 * distance, 0),
            Direction::Right => (1 * distance, 0),
        }
    }
}

struct Instruction {
    direction: Direction,
    distance: i32,
}

impl Instruction {
    fn from_string(s: &str) -> Instruction {
        let mut str_iter = s.split_ascii_whitespace();
        Instruction {
            direction: Direction::from_char(str_iter.next().unwrap().chars().next().unwrap()),
            distance: str_iter.next().unwrap().parse::<i32>().unwrap(),
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

struct Grid {
    vertices: Vec<Point>,
    perimeter: i32,
}

impl Grid {
    fn from_string(str: &str) -> Grid {
        let (vertices, perimeter, ..) = str.lines()
            .map(Instruction::from_string)
            .fold((Vec::new(),0, 0,0), |(mut acc,perimeter, x, y), instruction| {
                let (dx, dy) = instruction.direction.to_tuple(instruction.distance);
                acc.push(Point { x, y });
                (acc,perimeter + instruction.distance, x + dx, y + dy)
            });
            Grid {
                vertices,
                perimeter,
            }
    }

    fn calculate_area(&self) -> i32 {
        self.vertices.windows(2).fold(0, |acc, points| {
            acc + (points[1].x - points[0].x) * (points[1].y + points[0].y)
        }).abs() / 2 + self.perimeter / 2 + 1
    }
}

fn main() {
    let path = "input/puzzle.txt";

    let grid = Grid::from_string(&std::fs::read_to_string(path).unwrap());

    let area = grid.calculate_area();

    println!("Area: {}", area);
}