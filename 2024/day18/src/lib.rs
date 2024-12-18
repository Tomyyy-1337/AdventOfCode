use std::{borrow::BorrowMut, iter::TakeWhile, sync::{Arc, Mutex}};

mod test;

pub fn find_first_parallel(path: &str, width: usize, height: usize, chunk_size: usize) -> Option<(usize, usize)> {
    let mut maze_gen = MazeGenerator::from_path(width, height, path);
    
    let (sender, receiver) = std::sync::mpsc::channel();

    let mut indx = 0;

    while maze_gen.has_next() {
        let sender = sender.clone();
        let chunk: Vec<_> = (0..chunk_size).map(|_| maze_gen.next_maze()).take_while(|maze| maze.is_some()).map(|maze| maze.unwrap()).collect();
        std::thread::spawn(move || {
            chunk
                .into_iter()
                .position(|maze| maze.find_shortest_path().is_none())
                .map(|i| sender.send(indx + i).unwrap());
        });
        indx += chunk_size;
    }
    drop(sender);

    receiver.iter().min().map(|i| maze_gen.corruppted[i])
}

pub struct MazeGenerator {
    width: usize,
    height: usize,
    corrupted_indx: usize,
    corruppted: Vec<(usize, usize)>,
    maze: Vec<Cell>,
}

impl MazeGenerator {
    pub fn has_next(&self) -> bool {
        self.corrupted_indx < self.corruppted.len()
    }

    pub fn from_path(width: usize, height: usize, path: &str) -> Self {
        let maze = vec![Cell::Empty{ seen: false }; width * height];
        let contents = std::fs::read_to_string(path).unwrap();
        let corruppted = contents
            .lines()
            .map(|line| line.split(",").map(|s| s.parse::<usize>().unwrap()))
            .map(|mut nums| (nums.next().unwrap(), nums.next().unwrap()))
            .collect::<Vec<(usize, usize)>>();
        Self { width, height, corrupted_indx: 0, corruppted, maze }
    }

    pub fn next_maze(&mut self) -> Option<Maze> {
        if self.corrupted_indx == self.corruppted.len() {
            return None;
        }
        let (x, y) = self.corruppted[self.corrupted_indx];
        self.maze[y * self.width + x] = Cell::Corrupted;
        self.corrupted_indx += 1;
        Some(Maze { cells: self.maze.clone(), width: self.width, height: self.height })
    }

    pub fn find_blocking_byte(mut self) -> Option<(usize,usize)> {
        while let Some(maze) = self.next_maze() {
            if maze.find_shortest_path().is_none() {
                return Some(self.corruppted[self.corrupted_indx - 1]);
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty{ seen: bool },
    Corrupted,
}

pub struct Maze {
    cells: Vec<Cell>,
    width: usize,
    height: usize,    
}

impl Maze {
    pub fn from_path(path: &str, width: usize, height: usize, num_corrupted: usize) -> Self {
        let mut cells = vec![Cell::Empty{ seen: false }; width * height];
        let contents = std::fs::read_to_string(path).unwrap();
        contents
            .lines()
            .take(num_corrupted)
            .map(|line| line.split(",").map(|s| s.parse::<usize>().unwrap()))
            .map(|mut nums| (nums.next().unwrap(), nums.next().unwrap()))
            .for_each(|(x, y)| cells[y * width + x] = Cell::Corrupted);
        Self { cells, width, height }
    } 

    fn is_empty_and_unseen(&self, x: usize, y: usize) -> bool {
        match self.cells[y * self.width + x] {
            Cell::Empty { seen } => !seen,
            Cell::Corrupted => false,
        }
    }

    pub fn find_shortest_path(mut self) -> Option<u64> {
        let mut queue = std::collections::BinaryHeap::new();
        queue.push(State { cost: 0, x: 0, y: 0 });

        while let Some(State { cost, x, y }) = queue.pop() {
            match self.cells[y * self.width + x].borrow_mut() {
                Cell::Corrupted | Cell::Empty { seen: true } => continue,
                Cell::Empty { seen } => *seen = true,
            }
            if x == self.width - 1 && y == self.height - 1 {
                return Some(cost);
            }

            if x > 0 && self.is_empty_and_unseen(x - 1, y) {
                queue.push(State { cost: cost + 1, x: x - 1, y });
            }
            if x < self.width - 1 && self.is_empty_and_unseen(x + 1, y) {
                queue.push(State { cost: cost + 1, x: x + 1, y });
            }
            if y > 0 && self.is_empty_and_unseen(x, y - 1) {
                queue.push(State { cost: cost + 1, x, y: y - 1 });
            }
            if y < self.height - 1 && self.is_empty_and_unseen(x, y + 1) {
                queue.push(State { cost: cost + 1, x, y: y + 1 });
            }

        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    cost: u64,
    x: usize,
    y: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}