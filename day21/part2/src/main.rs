use hashbrown::hash_set::HashSet;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Garden,
    Rock,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Garden {
    tiles: Vec<Tile>,
    width: i32,
    height: i32,
    start: (i32, i32),
}

impl Garden {
    fn from_str(s: &str) -> Garden {
        let width = s.lines().next().unwrap().len() as i32;
        let height = s.lines().count() as i32;
        let start_indx = s.chars().filter(|&c| c != '\n' && c != '\r').position(|c| c == 'S').unwrap() as i32;
        let start = (start_indx % width, start_indx / width);
        let tiles = s.lines()
            .map(|line| {
                line.chars().into_iter()
                .map(|c| match c {
                    '.' | 'S' => Tile::Garden,
                    '#' => Tile::Rock,
                    _ => panic!("Invalid character in garden"),
                })
            }).flatten()
            .collect::<Vec<Tile>>();
        Garden { tiles, width, height, start }
    }

    fn get_tile(&self, x: i32, y: i32) -> &Tile {
        let x = ((x % self.width) + self.width) % self.width;
        let y = ((y % self.height) + self.height) % self.height;
        self.tiles.get((y * self.width + x) as usize).unwrap()
    }

    fn reachable(&self, steps: u64) -> u64 {
        let mut stack: Vec<((i32, i32), u64)> = vec![(self.start, steps)];
        let mut reachable: HashSet<(i32, i32)> = HashSet::new();
        let mut visited: HashSet<((i32, i32), u64)> = HashSet::new();

        while let Some(((x, y), steps)) = stack.pop() {
            if !visited.insert(((x, y), steps)) {
                continue;
            }
            match self.get_tile(x, y) {
                Tile::Garden if steps == 0 => {reachable.insert((x, y));},
                Tile::Garden => {
                    stack.push(((x + 1, y), steps - 1));
                    stack.push(((x - 1, y), steps - 1));
                    stack.push(((x, y + 1), steps - 1));
                    stack.push(((x, y - 1), steps - 1));
                },
                _ => continue,
            }
        }
        reachable.len() as u64
    }

    fn reachable_big_inputs(&self, steps: u64) -> f64 {
        let x1 = (((steps as i32 % self.width) + self.width) % self.width + self.width * 4) as u64;
        let x2 = x1 + self.width as u64;
        let x3 = x2 + self.width as u64;
        let res = vec![x1, x2, x3].par_iter().map(|&x| self.reachable(x)).collect::<Vec<u64>>();

        let y1 = res[0] as f64;
        let y2 = res[1] as f64;
        let y3 = res[2] as f64;

        let x1 = x1 as f64;
        let x2 = x2 as f64;
        let x3 = x3 as f64;
        
        let a = (y1 - 2.0 * y2 + y3) / (x1 * x1 - 2.0 * x2 * x2 + x3 * x3);
        let b = (y1 - y2 - a * x1 * x1 + a * x2 * x2) / (x1 - x2);
        let c = y1 - a * x1 * x1 - b * x1;

        let steps = steps as f64;
        a * steps * steps + b * steps + c
    }
}

fn main() {
    let path = "input/puzzle.txt";
    let garden = Garden::from_str(&std::fs::read_to_string(path).unwrap());

    let reachable_big_inputs = garden.reachable_big_inputs(26501365);

    println!("Reachable big inputs: {}", reachable_big_inputs);
}