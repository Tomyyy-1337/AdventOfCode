#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Brick {
    x1: u32, y1: u32, z1: u32,
    x2: u32, y2: u32, z2: u32,
}

impl Brick {
    fn from_str(s: &str) -> Brick {
        let cords: Vec<u32> = s.replace('~', ",").split(',').flat_map(str::parse).collect();
        Brick { x1: cords[0], y1: cords[1], z1: cords[2], x2: cords[3], y2: cords[4], z2: cords[5]}
    }

    fn drop(&mut self, n: u32) {
        self.z1 -= n;
        self.z2 -= n;
    }
}


fn main() {
    let path = "input/puzzle.txt";
    let mut bricks: Vec<_> = std::fs::read_to_string(path).unwrap().lines().map(Brick::from_str).collect();

    while let Some((i, n)) = falling_brick(&bricks) {
        bricks[i].drop(n);
    }
    let count = (0..bricks.len()).filter(|&i| {
        let mut bricks_clone = bricks.clone();
        bricks_clone.remove(i);
        falling_brick(&bricks_clone).is_none()
    }).count();

    println!("Count: {}", count);
}

fn falling_brick(bricks: &Vec<Brick>) -> Option<(usize, u32)> {
    for (i,brick) in bricks.iter().enumerate() {
        let mut brick = brick.clone();
        let mut drop_count = 0;

        while brick.z1 > 1 && bricks.iter().find(|&&b| b != brick && b.x2 >= brick.x1 && b.x1 <= brick.x2 && b.y2 >= brick.y1 && b.y1 <= brick.y2 && b.z2 == brick.z1 - 1).is_none() {
            drop_count += 1;
            brick.drop(1);
        }
        if drop_count > 0 {
            return Some((i, drop_count));
        }
    }
    None
}