struct Rucksack {
    items : Vec<u16>,
}

impl Rucksack {
    fn from_string(input: &str) -> Rucksack {
        let mut items = input.chars().map(Rucksack::char_to_u16).collect::<Vec<_>>();
        items.sort();
        Rucksack {
            items,
        }
    }
    
    fn char_to_u16(c: char) -> u16 {
        match c {
            'a'..='z' => c as u16 - 96,
            'A'..='Z' => c as u16 - 38,
            _ => 0,
        }
    }
}

struct Group {
    rucksacks: Vec<Rucksack>,
}

impl Group {
    fn from_string(input: &[&str]) -> Group {
        let rucksacks = input.iter().map(|&l| Rucksack::from_string(l)).collect();
        Group {
            rucksacks,
        }
    }

    fn find_common_items(self) -> u16 {
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;

        while i < self.rucksacks[0].items.len() && j < self.rucksacks[1].items.len() && k < self.rucksacks[2].items.len() {
            if self.rucksacks[0].items[i] == self.rucksacks[1].items[j] && self.rucksacks[1].items[j] == self.rucksacks[2].items[k] {
                return self.rucksacks[0].items[i];
            } else if self.rucksacks[0].items[i] <= self.rucksacks[1].items[j] && self.rucksacks[0].items[i] <= self.rucksacks[2].items[k] {
                i += 1;
            } else if self.rucksacks[1].items[j] <= self.rucksacks[0].items[i] && self.rucksacks[1].items[j] <= self.rucksacks[2].items[k] {
                j += 1;
            } else {
                k += 1;
            }
        }
        return 0;
    }
}

fn main() {
    let path = "input/puzzle.txt";
    let contents = std::fs::read_to_string(path).unwrap();
    let sum = contents.lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(Group::from_string)
        .map(Group::find_common_items)
        .sum::<u16>();

    println!("{}", sum);
}