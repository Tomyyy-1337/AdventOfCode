const OFFSETS:[(i32,i32); 8] = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)];

#[derive(PartialEq, Eq, Debug)]
enum Component {
    Digit(u32),
    Symbol(char),
    Empty,
}

struct Schematic {
    width: u32,
    height: u32,
    components: Vec<Component>,
}

impl Schematic {
    fn get_component(&self, x: u32, y: u32) -> &Component {
        &self.components[(y * self.width + x) as usize]
    }

    pub fn gear_ratios(&self) -> Vec<(u32, u32)> {
        (0..self.width*self.height)
            .filter_map(|i| self.gear_ratio_at(i % self.width, i / self.width))
            .collect()
    }

    fn gear_ratio_at(&self, x: u32, y: u32) -> Option<(u32,u32)> {
        if let Component::Symbol('*') = self.get_component(x, y) {
            let numbers = OFFSETS.iter()
                .filter_map(|(dx, dy)| self.get_number_at((x as i32 + dx) as u32, ((y as i32) + dy) as u32))
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .map(|(a, _)| a)
                .collect::<Vec<_>>();
            if numbers.len() == 2 {
                return Some((numbers[0], numbers[1]))
            }
        }
        None
    }

    fn get_number_at(&self, x: u32, y: u32) -> Option<(u32, u32)> {
        if let Component::Digit(_) = self.get_component(x, y) {
            let mut start = x as i32;
            while let Component::Digit(_) = self.get_component(start as u32, y) {
                start -= 1;
                if start < 0 {
                    break;
                }
            }
            start += 1;
            let start_pos = start as u32;
            let mut num = 0;
            while let Component::Digit(digit) = self.get_component(start as u32, y) {
                num = num * 10 + digit;
                start += 1;
                if start as u32 >= self.width {
                    break;
                }
            }
            return Some((num,start_pos as u32));
        }
        None
    }
}

fn main() {
    let path = "input/test.txt";
    let contents = std::fs::read_to_string(path).expect("Failed to read file");
    let sum = Schematic {
        width: contents.lines().next().unwrap().len() as u32,
        height: contents.lines().count() as u32,
        components: contents.chars()
            .filter_map(|c| match c {
                '0'..='9' => Some(Component::Digit(c as u32 - '0' as u32)),
                '.' => Some(Component::Empty),
                '\r' | '\n' => None,
                c => Some(Component::Symbol(c)),
            }).collect(),
    }.gear_ratios()
    .iter()
    .map(|(a, b)| a * b)
    .sum::<u32>();

    println!("Summe = {}", sum);
}