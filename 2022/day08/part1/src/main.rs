fn main() {
    let contents = std::fs::read_to_string("input/puzzle.txt").unwrap();
    let grid = Grid::from_string(&contents);

    let visible = (0..grid.width*grid.height)
        .filter(|i| grid.is_visible(i % grid.width, i / grid.width))
        .count();
    
    println!("Visible: {}", visible);
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

    fn get(&self, x: usize, y: usize) -> u8 {
        self.grid[y * self.width + x]
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        let tree_height = self.get(x, y);
        
        self.row(tree_height, y, 0, x)
        || self.row(tree_height, y, x+1, self.width)
        || self.collumn(tree_height, x, 0, y)
        || self.collumn(tree_height, x, y+1, self.height)
    }

    fn row(&self, tree_height: u8, fix: usize, lower: usize, upper: usize ) -> bool {
        (lower..upper).find(|&i| tree_height <= self.get(i, fix)).is_none()
    }

    fn collumn(&self, tree_height: u8, fix: usize, lower: usize, upper: usize ) -> bool {
        (lower..upper).find(|&i| tree_height <= self.get(fix, i)).is_none()
    }
}
