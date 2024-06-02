fn main() {
    let contents = std::fs::read_to_string("input/puzzle.txt").unwrap();
    let grid = Grid::from_string(&contents);

    let max_distance = (0..grid.width*grid.height)
        .map(|i| grid.distance(i as i32 % grid.width as i32, i as i32 / grid.width as i32))
        .max()
        .unwrap();
    
    println!("Max_distance: {}", max_distance);
}

struct Grid {
    grid: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn from_string(input: &str) -> Self {
        Grid {
            grid: input.lines().flat_map(|line| {
                line.chars().map(|c| match c {
                    '0'..='9' => c.to_digit(10).unwrap() as u8,
                    _ => panic!("Invalid character in input"),
                })}).collect(),
            width: input.lines().next().unwrap().len(),
            height: input.lines().count(),
        }
    }

    fn get(&self, x: i32, y: i32) -> u8 {
        self.grid[y as usize * self.width as usize + x as usize]
    }

    fn distance(&self, x: i32, y: i32) -> i32 {
        let tree_height = self.get(x, y);
        self.distance_row(tree_height, x, y, 1, 0)
        * self.distance_row(tree_height, x, y, -1, 0)
        * self.distance_row(tree_height, x, y, 0, 1)
        * self.distance_row(tree_height, x, y, 0, -1)
    }

    fn distance_row(&self, height: u8, mut x: i32, mut y: i32, direction_x: i32, direction_y: i32) -> i32 {
        for count in 0.. {
            x += direction_x;
            y += direction_y;
            if y < 0 || y >= self.height as i32 || x < 0 || x >= self.width as i32{
                return count;
            }
            if self.get(x, y) >= height {
                return count + 1;
            }
        }
        unreachable!()
    }
}
