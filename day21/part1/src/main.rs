use hashbrown::hash_set::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Garden,
    Rock,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Garden {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    start: (usize, usize),
}

impl Garden {
    fn from_str(s: &str) -> Garden {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();
        let start_indx = s.chars().filter(|&c| c != '\n' && c != '\r').position(|c| c == 'S').unwrap();
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

    fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(y * self.width + x)
    }

    fn reachable(&self, steps: u32) -> u64 {
        let mut stack: Vec<((usize, usize), u32)> = vec![(self.start, steps)];
        let mut reachable: HashSet<usize> = HashSet::new();
        let mut visited: HashSet<((usize, usize), u32)> = HashSet::new();

        while let Some(((x, y), steps)) = stack.pop() {
            if !visited.insert(((x, y), steps)) {
                continue;
            }
            match self.get_tile(x, y) {
                Some(Tile::Garden) if steps == 0 => {reachable.insert(x + y * self.width);},
                Some(Tile::Garden) => {
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

}


fn main() {
    let path = "input/puzzle.txt";
    let garden = Garden::from_str(&std::fs::read_to_string(path).unwrap());
    
    let reachable = garden.reachable(64);
    
    println!("Reachable: {}", reachable);
}
