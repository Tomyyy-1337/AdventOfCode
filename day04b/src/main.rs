#[derive(Clone, Debug)]
struct Card {
    id: u32,
    wining_numbers: Vec<u32>,
    outcome: Vec<u32>,
}

impl Card {
    fn new(str: &str) -> Card {
        let row: Vec<&str> = str
            .split(":")
            .collect::<Vec<&str>>();
        let id = row[0]
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap();
        let numbers = row[1]
            .split("|")
            .collect::<Vec<&str>>();
        Card {
            id,
            wining_numbers: Card::parse_numbers(numbers[0]),
            outcome: Card::parse_numbers(numbers[1]),
        }
    }

    fn parse_numbers(str: &str) -> Vec<u32> {
        str.split(" ")
            .filter_map(|n| n.parse::<u32>().ok())
            .collect()
    }

    pub fn get_points(&self) -> u32 {
        self.outcome
            .iter()
            .filter(|n| self.wining_numbers.contains(n))
            .count() as u32
    }
}

fn main() {
    let path = "input/puzzle.txt";
    let contents = std::fs::read_to_string(path).unwrap();
    let table_size = contents.lines().count();
    let sum: u32 = contents
        .lines()
        .map(|line| Card::new(line))
        .rev()
        .scan(vec![1; table_size], |lookup, card| {
            let points = card.get_points();
            let start = card.id as usize;
            for d in start..(start + points as usize).min(table_size) {
                lookup[start-1] += lookup[d];
            }
            Some(lookup[start-1])
        })
        .sum();

    println!("Sum: {}", sum);
}