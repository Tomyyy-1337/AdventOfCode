use std::{collections::HashSet, vec};
use itertools::Itertools;

fn main() {
    let contents = include_str!("../input/puzzle");
    
    let puzzle = Puzzle::from_from_str(&contents);
    println!("part 1 Solution: {}", puzzle.solve_part_1());
    println!("Part 2 Solution: {}", puzzle.solve_part_2()); 
}

#[derive(Debug)]
struct Puzzle {
    positions_lookup: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

impl Puzzle {
    fn from_from_str(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let positions_lookup: Vec<_>= input
            .lines()
            .flat_map(str::bytes)
            .enumerate()
            .filter_map(|(i, c)| match c {
                b'A'..=b'Z' => Some((i, c - b'A')),
                b'a'..=b'z' => Some((i, c - b'a' + 26)),
                b'0'..=b'9' => Some((i, c - b'0' + 52)),
                _ => None,
            })
            .fold(vec![Vec::new(); 62], |mut acc: Vec<Vec<_>> , (i, cell)| {
                acc[cell as usize].push(i);
                acc
            });
        
        Self {
            width,
            height,
            positions_lookup,
        }
    }

    fn solve<F, T>(&self, test_pair: F) -> usize
    where
        F: Fn(&usize, &usize) -> T,
        T: IntoIterator<Item = usize>,
    {
        self.positions_lookup.iter().flat_map(|entry| 
            entry
                .iter()
                .tuple_combinations()
                .flat_map(|(a, b)| test_pair(a, b))
        )
        .collect::<HashSet<_>>()
        .len()
    }

    fn solve_part_1(&self) -> usize {
        self.solve(|a, b| self.test_pair_part_1(a, b))
    }

    fn solve_part_2(&self) -> usize {
        self.solve(|a, b| self.test_pair_part_2(a, b))
    }

    fn test_pair_part_1(&self, indx1: &usize, indx2: &usize) -> Vec<usize> {
        let (x1, y1) = (indx1 % self.width, indx1 / self.width);
        let (x2, y2) = (indx2 % self.width, indx2 / self.width);

        let dx = x2 as i32 - x1 as i32;
        let dy = y2 as i32 - y1 as i32;
        
        [(-1, x1, y1), (1, x2, y2)].into_iter()
            .map(|(step, x, y)| (x as i32 + step * dx, y as i32 + step * dy))
            .filter(|&(x, y)| x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32)
            .map(|(x, y)| x as usize + y as usize * self.width)
            .collect()
    }
    
    fn test_pair_part_2(&self, indx1: &usize, indx2: &usize) -> Vec<usize> {
        let (x1, y1) = (indx1 % self.width, indx1 / self.width);
        let (x2, y2) = (indx2 % self.width, indx2 / self.width);
        let dx = x2 as i32 - x1 as i32;
        let dy = y2 as i32 - y1 as i32;
        
        let gcd = gcd(dx.abs() as usize, dy.abs() as usize);
        let (dx, dy) = (dx / gcd as i32, dy / gcd as i32);
        
        let mut results = vec![];

        for (mut x, mut y, step) in [(x1 as i32 + dx, y1 as i32 + dy, 1), (x1 as i32, y1 as i32, -1)] {
            while x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                results.push(x as usize + y as usize * self.width);
                x += dx * step;
                y += dy * step;
            }
        }   
        results
    }
}
    
fn gcd(a: usize, b: usize) -> usize {
    match (a, b) {
        (0, a) | (a, 0) => a,
        _ => gcd(b, a % b),
    }
}   