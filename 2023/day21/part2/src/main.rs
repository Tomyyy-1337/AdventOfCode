use hashbrown::hash_set::HashSet;
use priority_queue::PriorityQueue;
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
        let tiles: Vec<Tile> = s.chars()
            .filter_map(|c| match c {
                '.' | 'S' => Some(Tile::Garden),
                '#'       => Some(Tile::Rock),
                _         => None,
            })
            .collect();
        Garden { tiles, width, height, start }
    }

    fn get_tile(&self, x: i32, y: i32) -> &Tile {
        let x = ((x % self.width) + self.width) % self.width;
        let y = ((y % self.height) + self.height) % self.height;
        self.tiles.get((y * self.width + x) as usize).unwrap()
    }

    fn reachable(&self, steps: u32) -> u32 {
        let mut queue: PriorityQueue<(i32,i32), u32> = PriorityQueue::new();
        queue.push(self.start, steps);
        let mut visited: HashSet<((i32, i32), u32)> = HashSet::new();

        while let Some(((x,y), steps)) = queue.pop() {
            match self.get_tile(x, y) {
                Tile::Garden if visited.insert(((x, y), steps % 2)) && steps > 0 => {
                    queue.push((x + 1, y), steps - 1);
                    queue.push((x - 1, y), steps - 1);
                    queue.push((x, y + 1), steps - 1);
                    queue.push((x, y - 1), steps - 1);
                },
                _ => (),
            }
        }
        visited.iter().filter(|&(_, s)| *s == 0).count() as u32
    }

    fn reachable_big_inputs(&self, steps: u64) -> u64 {
        let x1 = (((steps as i32 % self.width) + self.width) % self.width + self.width * 3) as u32;
        let x2 = x1 + self.width as u32;
        let x3 = x2 + self.width as u32;
        let res = [x1, x2, x3].par_iter().map(|&x| (x as f64, self.reachable(x) as f64)).collect::<Vec<(f64,f64)>>();

        let (x1, y1) = res[0];
        let (x2, y2) = res[1];
        let (x3, y3) = res[2];
        
        let a = (y1 - 2.0 * y2 + y3) / (x1 * x1 - 2.0 * x2 * x2 + x3 * x3);
        let b = (y1 - y2 - a * x1 * x1 + a * x2 * x2) / (x1 - x2);
        let c = y1 - a * x1 * x1 - b * x1;

        let steps = steps as f64;
        (a * steps * steps + b * steps + c).round() as u64
    }
}

fn main() {
    let path = "input/puzzle.txt";
    let garden = Garden::from_str(&std::fs::read_to_string(path).unwrap());

    let num_tiles = garden.reachable_big_inputs(26501365);

    println!("Reachable Tiles: {}", num_tiles);
}