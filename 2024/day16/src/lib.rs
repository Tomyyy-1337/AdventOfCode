use core::panic;
use std::{cmp::Ordering, collections::{BinaryHeap, HashMap, HashSet}};

mod test;

pub fn part1(path: &str) -> u64 {
    let contents = std::fs::read_to_string(path).unwrap();
    let mut maze = Maze::new(&contents);
    maze.solve()
}

pub fn part2(path: &str) -> u64 {
    let contents = std::fs::read_to_string(path).unwrap();
    let mut maze = Maze::new(&contents);
    maze.solve2()
}

enum Tile {
    Empty,
    Wall,
    Goal,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_left(&self) -> Self {
        // match self {
        //     Self::Up => Self::Left,
        //     Self::Down => Self::Right,
        //     Self::Left => Self::Down,
        //     Self::Right => Self::Up,
        // };
        unsafe { std::mem::transmute(((*self as usize + 3) % 4) as u8) }
    }
    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
    fn to_tupel(&self) -> (isize, isize) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (isize, isize),
    direction: Direction,
}

// Iverse ordering for min heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Maze {
    tiles: Vec<Vec<Tile>>,
    start: (isize, isize),
}

impl Maze {
    fn new(contents: &str) -> Self {
        let mut start_pos = (0,0);
        let tiles = contents.lines().enumerate().map(|(y, line)| {
            line.chars().enumerate().map(|(x, c)| match c {
                '.' => Tile::Empty,
                '#' => Tile::Wall,
                'E' => Tile::Goal,
                'S' => {
                    start_pos = (x as isize, y as isize);
                    Tile::Empty
                },
                _ => panic!("Invalid tile"),
            }).collect()
        }).collect();

        Self { tiles, start: start_pos }
    }

    fn solve(&mut self) -> u64 {
        let mut queue = BinaryHeap::new();
        queue.push(State { cost: 0, position: self.start, direction: Direction::Right });

        let mut seen = HashSet::new();

        while let Some(State { cost, position: (x,y), direction }) = queue.pop() {
            if !seen.insert((x,y,direction)) {
                continue;
            }

            match self.tiles[y as usize][x as usize] {
                Tile::Goal => return cost as u64,
                Tile::Wall => continue,
                _ =>  {
                    queue.push(State { cost: cost + 1, position: (x + direction.to_tupel().0, y + direction.to_tupel().1), direction });
                    queue.push(State { cost: cost + 1000, position: (x, y), direction: direction.turn_left() });
                    queue.push(State { cost: cost + 1000, position: (x, y), direction: direction.turn_right() });
                },
            }
        }

        panic!("No solution found")
    }

    fn solve2(&mut self) -> u64 {
        let mut queue = BinaryHeap::new();
        queue.push(State2 { cost: 0, position: self.start, direction: Direction::Right, path_seen: HashSet::new() });
        let mut seen = HashMap::new();

        let mut best_cost = None;
        let mut best_tiles = HashSet::new();
        
        while let Some(State2 { cost, position: (x,y), direction, mut path_seen }) = queue.pop() {
            match seen.get(&(x,y,direction)) {
                Some(&seen_cost) if seen_cost < cost => continue,
                _ => { seen.insert((x,y,direction), cost); },
            }
                 
            path_seen.insert((x,y));

            match self.tiles[y as usize][x as usize] {
                Tile::Wall => continue,
                Tile::Goal => {
                    match best_cost {
                        None => {
                            best_cost = Some(cost);
                            best_tiles = path_seen;
                        },
                        Some(best_cost) if best_cost == cost => {
                            best_tiles.extend(path_seen);
                        },
                        Some(_) => continue,
                    }
                }
                Tile::Empty =>  {
                    queue.push(State2 { cost: cost + 1, position: (x + direction.to_tupel().0, y + direction.to_tupel().1), direction, path_seen: path_seen.clone() });
                    queue.push(State2 { cost: cost + 1000, position: (x, y), direction: direction.turn_left(), path_seen: path_seen.clone() });
                    queue.push(State2 { cost: cost + 1000, position: (x, y), direction: direction.turn_right(), path_seen: path_seen.clone() });
                },
            }
        }

        // for (y, line) in self.tiles.iter().enumerate() {
        //     for (x, tile) in line.iter().enumerate() {
        //         print!("{}", match tile {
        //             Tile::Empty if best_tiles.contains(&(x as isize,y as isize)) => 'O',
        //             Tile::Empty => '.',
        //             Tile::Wall => '#',
        //             Tile::Goal => 'E',
        //         });
        //     }
        //     println!();
        // }

        best_tiles.len() as u64
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State2 {
    cost: usize,
    position: (isize, isize),
    direction: Direction,
    path_seen: HashSet<(isize, isize)>,
}

// Iverse ordering for min heap
impl Ord for State2 {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for State2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}