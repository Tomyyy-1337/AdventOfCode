use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSlice;

#[derive(Clone)]
struct Range {
    start_input: u64,
    start_output: u64,
    width: u64,
}

impl Range {
    pub fn map(&self, n: u64) -> u64 {
        if self.start_input <= n && n < self.start_input + self.width {
            self.start_output + n - self.start_input
        } else {
            n
        }
    }
}

#[derive(Clone)]
struct CustomMap {
    ranges: Vec<Range>,
}

impl CustomMap {
    pub fn map_recursive(&self, n: u64) -> u64 {
        self.ranges.iter().find_map(|range| {
            if range.map(n) != n {
                Some(range.map(n))
            } else {
                None
            }
        }).unwrap_or(n)
    }
}

fn main() {
    let path = "input/puzzle.txt";
    let mut contents: Vec<String> = std::fs::read_to_string(path)
        .unwrap()
        .split("\r")
        .filter(|&s| s != "\n")
        .map(|s| s.replace("\n", "").to_string()) // Convert &str to String
        .collect();
    
    contents.push("end".to_string());
    let maps: Vec<CustomMap> = contents[1..]
        .iter()
        .fold((Vec::new(), None), |(mut maps, mut map), line| {
            if ('0'..='9').contains(&line.chars().nth(0).unwrap()) {
                let nums: Vec<u64> = line
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect();
                map.get_or_insert_with(|| CustomMap { ranges: Vec::new() })
                    .ranges
                    .push(Range {
                        start_input: nums[1],
                        start_output: nums[0],
                        width: nums[2],
                    });
            } else if let Some(map) = map.take() {
                maps.push(map);
            }
            (maps, map)
        }).0;

    let min = contents[0]
        .split_ascii_whitespace()
        .skip(1)
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<_>>()
        .par_chunks_exact(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .flatten()
        .map(|seed| {
            maps.iter()
                .fold(seed, |n, map| map.map_recursive(n))})
        .min()
        .unwrap();
    
    println!("min: {}", min);
}