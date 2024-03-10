struct Rucksack {
    first_compartment: Vec<u16>,
    second_compartment: Vec<u16>,
}

impl Rucksack {
    fn from_string(input: &str) -> Rucksack {
        let (a,b) = input.split_at(input.len()/2);
        Rucksack {
            first_compartment: a.chars().map(Rucksack::char_to_u16).collect(),
            second_compartment: b.chars().map(Rucksack::char_to_u16).collect(),
        }
    }

    fn char_to_u16(c: char) -> u16 {
        match c {
            'a'..='z' => c as u16 - 96,
            'A'..='Z' => c as u16 - 38,
            _ => 0,
        }
    }

    fn commom_type(mut self) -> u16 {
        self.first_compartment.sort();
        self.second_compartment.sort();
        let mut i = 0;
        let mut j = 0;

        while i < self.first_compartment.len() && j < self.second_compartment.len() {
            if self.first_compartment[i] == self.second_compartment[j] {
                return self.first_compartment[i];
            } else if self.first_compartment[i] < self.second_compartment[j] {
                i += 1;
            } else {
                j += 1;
            }
        }
        return 0;
    }
}

fn main() {
    let path = "input/puzzle.txt";
    let contents = std::fs::read_to_string(path).unwrap();
    let sum = contents.lines()
        .map(Rucksack::from_string)
        .map(Rucksack::commom_type)
        .sum::<u16>();

    println!("{}", sum);
}