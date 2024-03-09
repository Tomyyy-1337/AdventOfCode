use hashbrown::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Round,
    Cube
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '.' => Tile::Empty,
            'O' => Tile::Round,
            '#' => Tile::Cube,
            _ => panic!("Invalid tile character")
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Grid {
    tiles: Vec<Tile>,
    width: usize,
    height: usize
}

impl Grid {
    fn new(str: &str) -> Grid {
        Grid {
            tiles: str.lines().flat_map(|l| l.chars().map(Tile::from_char)).collect(),
            width: str.lines().next().unwrap().len(),
            height: str.lines().count(),
        }
    }

    fn tilt(&mut self) {
        let mut new_tiles: Vec<Tile> = self.tiles.iter().map(|tile| match tile {
            Tile::Cube => Tile::Cube,
            Tile::Empty | Tile::Round  => Tile::Empty,
        }).collect();
        for (i, &tile) in self.tiles.iter().enumerate() {
            if i < self.width {
                new_tiles[i] = tile;
                continue;
            }
            if tile == Tile::Round {
                let x = i % self.width;
                let mut y = i / self.width - 1;
                while let Some(tmp_tile) = new_tiles.get(x + y * self.width) {
                    match tmp_tile {
                        Tile::Empty => if y == 0 { break } else { y -= 1 },
                        Tile::Round | Tile::Cube => {
                            y += 1;
                            break;
                        },
                    }
                }
                new_tiles[x + y * self.width] = Tile::Round;
            } 
        }
        self.tiles = new_tiles;
    }

    fn rotate_90_deg(&mut self) {
        let n = self.width;
        for i in 0..n {
            for j in i+1..n {
                self.tiles.swap(i * n + j, j * n + i);
            }
            let (mut row_start, mut row_end) = (i * n, (i+1) * n - 1);
            while row_start < row_end {
                self.tiles.swap(row_start, row_end);
                row_start += 1;
                row_end -= 1;
            }
        }
    }

    fn spin(&self) -> u32 { 
        let iterations: u64 = 1000000000;
        let mut grid = self.clone();
        let mut lookup: HashMap<(Grid, u64),(Grid, u64)> = HashMap::new();
        let mut i = 0;
        while i < iterations*4 {
            let key = (grid.clone(), i % 4);
            if let Some((tile, index)) = lookup.get(&key) {
                grid = tile.clone();
                let loop_size = i - index;
                while i + loop_size < iterations*4 {
                    i += loop_size;
                }
            } else {
                grid.tilt();
                lookup.insert(key, (grid.clone(), i));
            }
            grid.rotate_90_deg();
            i += 1;
        }
        grid.calculate()
    }

    fn calculate(&self) -> u32 {
        self.tiles.iter().enumerate().filter_map(|(i, tile)| {
            match tile {
                Tile::Round => Some(self.height - i / self.width),
                _ => None
            }
        }).sum::<usize>() as u32
    }
}


fn main() {
    let path = "input/puzzle.txt"; 

    let sum = Grid::new(&std::fs::read_to_string(path).unwrap()).spin();

    println!("{}", sum);
}   