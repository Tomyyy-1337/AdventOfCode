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
        let numbers = row.get(1)
            .unwrap()
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
    let mut cards: Vec<Card>  = contents
        .lines()
        .map(|line| Card::new(line))
        .collect();

    let mut index = 0;
    while index < cards.len() {
        let points = cards[index].get_points();
        let start = cards[index].id as usize;
        for d in start..(start + points as usize).min(table_size) {
            cards.push(cards[d].clone());
        }
        index += 1;
    }
    let sum = cards.len();

    println!("Sum: {}", sum);
}