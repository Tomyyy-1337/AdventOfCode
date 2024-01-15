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
    pub fn new(str: &str) -> Schematic {
        Schematic {
            width: str.lines().next().unwrap().len() as u32,
            height: str.lines().count() as u32,
            components: str.chars()
                .filter_map(|c| match c {
                    '0'..='9' => Some(Component::Digit(c as u32 - '0' as u32)),
                    '.' => Some(Component::Empty),
                    '\r' | '\n' => None,
                    c => Some(Component::Symbol(c)),
                }).collect(),
        }
    }

    fn get_component(&self, x: u32, y: u32) -> &Component {
        &self.components[(y * self.width + x) as usize]
    }

    fn get_neighbour(&self, x: u32, y: u32) -> &Component {
        for (dx, dy) in &OFFSETS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if (0..self.width as i32).contains(&nx) && (0..self.height as i32).contains(&ny) {
                let component = self.get_component(nx as u32, ny as u32);
                if let &Component::Digit(_) = component {
                    continue;
                }
                if component != &Component::Empty {
                    return component;
                }
            }
        }
        &Component::Empty
    }

    fn get_number_at(&self, x: u32, y: u32) -> Option<u32> {
        if let Component::Digit(_) = self.get_component(x, y) {
            let mut start = x as i32;
            while let Component::Digit(_) = self.get_component(start as u32, y) {
                start -= 1;
                if start < 0 {
                    break;
                }
            }
            start += 1;
            let mut num = 0;
            while let Component::Digit(digit) = self.get_component(start as u32, y) {
                num = num * 10 + digit;
                start += 1;
                if start as u32 >= self.width {
                    break;
                }
            }
            return Some(num);
        }
        None
    }

    fn get_numbers(&self) -> Vec<u32> {
        let mut numbers = Vec::new();
        let mut skip = false;
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get_component(x, y) {
                    Component::Digit(_) => {
                        if let Some(num) = self.get_number_at(x, y) {
                            if !skip && self.get_neighbour(x, y) != &Component::Empty {
                                numbers.push(num);
                                skip = true;
                            }
                        }
                    }
                    _ => skip = false,
                }
            }
            skip = false;
        }
        numbers
    }
}

fn main() {
    let path = "input/test.txt";
    let sum = Schematic::new(&std::fs::read_to_string(path).unwrap())
        .get_numbers()
        .iter()
        .sum::<u32>();

    println!("Summe = {}", sum);
}