use std::collections::HashSet;

fn main() {
    let contents = include_str!("../input/puzzle");

    let start = std::time::Instant::now();
    let mut grid = Grid::from_str(contents);
    grid.part_1();
    grid.part_2();
    println!("Time: {:?}", start.elapsed());
}

struct Cell {
    height: u8,
}   

impl Cell {
    fn from_char(c: char) -> Cell {
        Cell {
            height: c.to_digit(10).unwrap() as u8,
        }
    }
}

struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Grid {
    fn from_str(input: &str) -> Grid {
        Grid {
            cells: input
                .lines()
                .flat_map(|line| line.chars().map(Cell::from_char))
                .collect(),
            width: input.lines().next().unwrap().len(),
            height: input.lines().count(),
        }
    }

    fn part_1(&mut self) {
        let result: u32 = (0..self.width * self.height)
            .into_iter()
            .filter_map(|i| {
                let mut seen = HashSet::new();
                match self.cells[i].height {
                    0 => Some(self.find_number_part_1(i, &mut seen)),
                    _ => None,
                }   
            })
            .sum();

        println!("Result: {}", result);
    }

    fn find_number_part_1(&mut self, index: usize, seen: &mut HashSet<usize>) -> u32 {
        if !seen.insert(index) {
            return 0;
        }
        match self.cells[index] {
            Cell { height: 9} => 1,
            Cell { height } => 
                self.get_one_heigher_neighbours(index, height)
                    .iter()
                    .map(|&neighbour| self.find_number_part_1(neighbour, seen))
                    .sum()
        }
    }

    fn part_2(&mut self) {
        let result: u32 = (0..self.width * self.height)
            .into_iter()
            .filter_map(|i| {
                match self.cells[i].height {
                    0 => Some(self.find_number_part_2(i)),
                    _ => None,
                }   
            })
            .sum();

        println!("Result: {}", result);
    }

    fn find_number_part_2(&mut self, index: usize) -> u32 {
        match self.cells[index] {
            Cell { height: 9} => 1,
            Cell { height } => 
                self.get_one_heigher_neighbours(index, height)
                    .iter()
                    .map(|&neighbour| self.find_number_part_2(neighbour))
                    .sum()
        }
    }

    fn get_one_heigher_neighbours(&self, index: usize, height: u8) -> Vec<usize> {
        let (x, y) = (index % self.width, index / self.width);
        let mut neighbours = [usize::MAX; 4];

        if x > 0 {
            neighbours[0] = index - 1;
        }
        if x < self.width - 1 {
            neighbours[1] = index + 1;
        }
        if y > 0 {
            neighbours[2] = index - self.width;
        }
        if y < self.height - 1 {
            neighbours[3] = index + self.width;
        }

        neighbours
            .into_iter()
            .filter(|&neighbour| neighbour != usize::MAX)
            .filter(|&neighbour| self.cells[neighbour].height == height + 1)
            .collect()
    }

}

