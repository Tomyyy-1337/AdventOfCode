fn main() {
    let path = "input/puzzle.txt";
    let max = std::fs::read_to_string(path).unwrap()
        .split("\r\n\r\n")
        .map(|block| 
            block.lines()
                .filter_map(|line| 
                    line.parse::<u32>().ok()
                )
                .sum::<u32>()
        )
        .max()
        .unwrap();

    println!("{}", max);
}
