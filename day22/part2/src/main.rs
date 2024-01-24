use hashbrown::HashSet;
use indicatif::ParallelProgressIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Brick {
    id: u64,
    x1: u32, y1: u32, z1: u32,
    x2: u32, y2: u32, z2: u32,
}

impl Brick {
    fn from_str(s: &str, id: u64) -> Brick {
        let cords: Vec<u32> = s.replace('~', ",").split(',').flat_map(str::parse).collect();
        Brick { id, x1: cords[0], y1: cords[1], z1: cords[2], x2: cords[3], y2: cords[4], z2: cords[5]}
    }

    fn drop(&mut self, n: u32) {
        self.z1 -= n;
        self.z2 -= n;
    }

    fn collision(&self, other: &Brick) -> bool {
        other != self && other.x2 >= self.x1 && other.x1 <= self.x2 && other.y2 >= self.y1 && other.y1 <= self.y2 && other.z2 == self.z1 - 1
    }
}


fn main() {
    let path = "input/puzzle.txt";
    let mut bricks: Vec<_> = std::fs::read_to_string(path).unwrap().lines().enumerate().map(|(i,s)| Brick::from_str(s, i as u64) ).collect();
    bricks.sort_by_key(|b| b.z2);

    while let Some((i, n)) = falling_brick(&bricks) {
        bricks[i].drop(n);
    }

    let sum: u32 = (0..bricks.len()).into_par_iter().progress().map(|i| {
        let mut bricks_clone = bricks.clone();
        bricks_clone.remove(i);
        let mut falling_set = HashSet::new();
        while let Some((i, n)) = falling_brick(&bricks_clone) {
            bricks_clone[i].drop(n);
            falling_set.insert(bricks[i].id);
        }
        falling_set.len() as u32
    }).sum();

    println!("Count: {}", sum);
}

fn falling_brick(bricks: &Vec<Brick>) -> Option<(usize, u32)> {
    for (i,brick) in bricks.iter().enumerate() {
        let mut brick = brick.clone();
        let mut drop_count = 0;

        while brick.z1 > 1 && bricks.iter().find(|&b| brick.collision(b)).is_none() {
            drop_count += 1;
            brick.drop(1);
        }
        if drop_count > 0 {
            return Some((i, drop_count));
        }
    }
    None
}