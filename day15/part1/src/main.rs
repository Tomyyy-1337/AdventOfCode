fn main() {
    let path = "input/puzzle.txt";

    let sum: u32 = std::fs::read_to_string(path).unwrap()
        .split(',')
        .map(hash)
        .sum();

    println!("sum: {}", sum);
}

fn hash(input: &str) -> u32 {
    input.chars().fold(0, |mut acc, c| {
        acc += c as u32;
        acc *= 17;
        acc % 256
    })
}