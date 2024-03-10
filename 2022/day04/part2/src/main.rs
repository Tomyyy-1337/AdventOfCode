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

    fn is_overlapping(&self) -> bool {
        self.range_1.min <= self.range_2.max && self.range_1.max >= self.range_2.min
    }
}

fn main() {
    let path = "input/puzzle.txt";
    
    let count_pairs = std::fs::read_to_string(path).unwrap()
        .lines()
        .map(Pair::from_str)    
        .filter(Pair::is_overlapping)
        .count();
    
    println!("Count pairs: {}", count_pairs);
}