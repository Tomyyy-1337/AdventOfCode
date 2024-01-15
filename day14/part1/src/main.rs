#[derive(Clone, Copy, PartialEq)]
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

#[derive(Clone)]
struct Grid {
    tiles: Vec<Tile>,
    width: usize,
    height: usize
}

impl Grid {
    fn new(str: &str) -> Grid {
        let tiles: Vec<Tile> = str.lines().flat_map(|l| l.chars().map(Tile::from_char)).collect();
        let width = str.lines().next().unwrap().len();
        let height = str.lines().count();
        Grid {
            tiles,
            width,
            height
        }
    }

    fn tilt(&self) -> Self {
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
        Grid {
            tiles: new_tiles,
            width: self.width,
            height: self.height
        }
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

    let sum = Grid::new(&std::fs::read_to_string(path).unwrap())
        .tilt()
        .calculate();

    println!("{}", sum);
}   