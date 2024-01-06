use rayon::{str::ParallelString, iter::ParallelIterator};

fn main() {
    let path = "input/puzzle.txt";
    let sum = std::fs::read_to_string(path)
        .unwrap()
        .par_lines()
        .map(|line| {
            let first_digit = get_first_digit(&line.chars().into_iter().collect(), false);
            let last_digit  = get_first_digit(&line.chars().into_iter().collect(), true);
            first_digit * 10 + last_digit
        })
        .sum::<u32>();
    println!("Summe: {}", sum);
}

fn get_first_digit(line: &Vec<char>, inverse: bool) -> u32 {
    for i in 0..line.len() {
        let index = if inverse { line.len() - i - 1 } else { i };
        let char = line[index];
        if ('0'..='9').contains(&char) {
            return char as u32 - '0' as u32;
        }
        if let Some(n) = is_nuber(index, line) {
            return n;
        }
    };
    0
}

fn is_nuber(index: usize, line: &Vec<char>) -> Option<u32> {
    let numbers: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for (i,&number) in numbers.iter().enumerate() {
        if let Some(chars) = line.get(index..index + number.len()) {
            if chars.iter().collect::<String>() == number {
                return Some(i as u32 + 1);
            }
        }
    }
    None
}