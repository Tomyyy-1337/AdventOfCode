#[derive(Clone)]
struct Range {
    start_input: u32,
    start_output: u32,
    width: u32,
}

impl Range {
    pub fn map(&self, n: u32) -> u32 {
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
    pub fn map(&self, n: u32) -> u32 {
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

    let seeds: Vec<u32> = contents[0]
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    
    contents.push("end".to_string());
    let maps: Vec<CustomMap> = contents[1..]
        .iter()
        .fold((Vec::new(), None), |(mut maps, mut map), line| {
            if ('0'..='9').contains(&line.chars().nth(0).unwrap()) {
                let nums: Vec<u32> = line
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<u32>().unwrap())
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

    let min = seeds.iter()
        .map(|seed| maps.iter().fold(*seed, |n, map| map.map(n)))
        .min()
        .unwrap();
    
    println!("min: {}", min);
}
