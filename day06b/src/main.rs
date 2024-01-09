fn main() {
    let path = "input/puzzle.txt";
    let contents = std::fs::read_to_string(path).unwrap()
        .lines()
        .map(|s| s.split_ascii_whitespace().skip(1).collect::<String>().parse::<u64>().unwrap())
        .collect::<Vec<_>>();
        
        let (time, distance) = (contents[0], contents[1]);
        let result = (1..time).into_iter()
            .filter(|i|  i * (time - i) > distance )
            .count();
    
    println!("{}", result);
}