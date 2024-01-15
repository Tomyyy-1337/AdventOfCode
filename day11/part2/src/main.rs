#[derive(Clone, Copy)]
struct Galaxy {
    x: i64,
    y: i64,
}

struct GalaxyPairs {
    galaxys: Vec<Galaxy>,
    first_indx: usize,
    second_indx: usize,
}

impl Iterator for GalaxyPairs {
    type Item = (Galaxy, Galaxy);

    fn next(&mut self) -> Option<Self::Item> {
        if self.second_indx >= self.galaxys.len() {
            self.first_indx += 1;
            self.second_indx = self.first_indx + 1;
        }
        if self.first_indx >= self.galaxys.len() || self.second_indx >= self.galaxys.len() {
            return None;
        }
        let first = self.galaxys[self.first_indx];
        let second = self.galaxys[self.second_indx];
        self.second_indx += 1;
        Some((first, second))
    }
}

fn main() {
    let path = "input/puzzle.txt";
    let contents = std::fs::read_to_string(path).unwrap();

    let galaxys: Vec<Galaxy> = contents.lines()
        .enumerate()
        .map(|(y, line)| line.chars()
            .enumerate()
            .filter_map(|(x, char)| match char {
                '#' => {
                    let expansion = get_expansion(x as i64, y as i64, &contents);
                    Some(Galaxy { x: x as i64 + expansion.0, y: y as i64 + expansion.1})
                } 
                _ => None
            }).collect::<Vec<Galaxy>>())
        .flatten()
        .collect(); 

    let pair_iterator = GalaxyPairs{ galaxys: galaxys, first_indx: 0, second_indx: 1 };
    let sum = pair_iterator
        .map(|(first, second)| (second.x - first.x).abs() + (second.y - first.y).abs())
        .sum::<i64>();

    println!("Sum: {}", sum);
}

fn get_expansion(x: i64, y: i64, contents: &str) -> (i64, i64) {
    let expansion_y = contents.lines().enumerate()
        .filter_map(|(a, line)| if a < y as usize { Some(line) } else { None })
        .filter(|line | line.chars().find(|&c| c == '#').is_none())
        .count() as i64 * (1_000_000 - 1);
    
    let expansion_x = (0..x)
        .map(|a| contents.lines().map(|line| line.chars().nth(a as usize).unwrap()).collect::<String>())
        .filter(|line| line.chars().find(|&c| c == '#').is_none())
        .count() as i64 * (1_000_000 - 1);

    (expansion_x, expansion_y)
}