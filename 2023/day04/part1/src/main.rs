use rayon::{str::ParallelString, iter::ParallelIterator};

struct Card {
    wining_numbers: Vec<u32>,
    outcome: Vec<u32>,
}

impl Card {
    fn new(str: &str) -> Card {
        let row: Vec<&str> = str
            .split(":")
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .split("|")
            .collect();
        Card {
            wining_numbers: Card::parse_numbers(row.get(0).unwrap()),
            outcome: Card::parse_numbers(row.get(1).unwrap()),
        }
    }

    fn parse_numbers(str: &str) -> Vec<u32> {
        str.split(" ")
            .filter_map(|n| n.parse::<u32>().ok())
            .collect()
    }

    pub fn get_points(&self) -> u32 {
        let count: u32 = self.outcome
            .iter()
            .filter(|n| self.wining_numbers.contains(n))
            .count() as u32;
        if count == 0 {
            return 0;
        }
        2u32.pow(count - 1)
    }
}

fn main() {
    let path = "input/puzzle.txt";
    let sum: u32 = std::fs::read_to_string(path)
        .unwrap()
        .par_lines()
        .map(|line| Card::new(line).get_points())
        .sum();

    println!("sum: {}", sum);
}