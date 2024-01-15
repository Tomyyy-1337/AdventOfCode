fn main() {
    let path = "input/puzzle.txt";
    let contents = std::fs::read_to_string(path).unwrap()
        .lines()
        .map(|s| s.split_ascii_whitespace().skip(1).map(|s| s.parse::<u32>().unwrap() ).collect::<Vec<_>>())
        .collect::<Vec<_>>();
        
    let product = contents[0].iter().zip(contents[1].iter())
        .map(|(&t,&d)| (1..t).into_iter().filter(|i|  i * (t - i) > d ).count())
        .product::<usize>();
    
    println!("{}", product);
}