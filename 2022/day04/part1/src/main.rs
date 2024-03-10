struct Range {
    min: u32,
    max: u32,
}

impl Range {
    fn from_str(str: &str) -> Self {
        let parts: Vec<&str> = str.split("-").collect();
        Range {
            min: parts[0].parse().unwrap(),
            max: parts[1].parse().unwrap(),
        }
    }

    fn contains(&self, other: &Range) -> bool {
        self.min <= other.min && other.max <= self.max
    }
}

struct Pair {
    range_1: Range,
    range_2: Range,
}

impl Pair {
    fn from_str(str: &str) -> Self {
        let parts: Vec<&str> = str.split(",").collect();
        Pair {
            range_1: Range::from_str(parts[0]),
            range_2: Range::from_str(parts[1]),
        }
    }

    fn contains_subrange(&self) -> bool {
        self.range_1.contains(&self.range_2) || self.range_2.contains(&self.range_1)
    }
}

fn main() {
    let path = "input/puzzle.txt";

    let count_pairs = std::fs::read_to_string(path).unwrap()
        .lines()
        .map(Pair::from_str)    
        .filter(Pair::contains_subrange)
        .count();

    println!("Count pairs: {}", count_pairs);
}