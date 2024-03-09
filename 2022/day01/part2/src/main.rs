fn main() {
    let path = "input/puzzle.txt";
    let mut sums = std::fs::read_to_string(path).unwrap()
        .split("\r\n\r\n")
        .map(|block| 
            block.lines()
                .filter_map(|line| 
                    line.parse::<u32>().ok()
                )
                .sum::<u32>()
        )
        .collect::<Vec<u32>>();
    
    sums.sort();
        
    let max_sum: u32 = sums.iter()
        .rev()
        .take(3)
        .sum();

    println!("{}", max_sum);
}
