#![allow(non_snake_case)]

fn main() {
    let contents = std::fs::read_to_string("input/puzzle.txt").unwrap();

    part_1(&contents);	

    part_2(&contents);
}
    
fn part_1(contents: &str) {
    let (A, B) = read_numbers(contents);
    
    let result: i32 = A.iter().zip(B.iter())
        .map(|(a, b)| (a-b).abs())
        .sum();
    
    println!("Part 1: {}", result);
}

fn part_2(contents: &str) {
    let (A, B) = read_numbers(contents);
    
    let mut result = 0;
    let mut i = 0;
    let mut j = 0;
    let mut count = 0;
    while i < A.len() && j < B.len() {
        if A[i] == B[j] {
            count += 1;
            j += 1;
        } else if A[i] < B[j] {
            result += A[i] * count;
            i += 1;
            count = 0;
        } else {
            j += 1;
        }
    }

    println!("Part 2: {}", result);
}

fn read_numbers(contents: &str) -> (Vec<i32>, Vec<i32>) {
    let (mut a, mut b): (Vec<_>, Vec<_>) = contents.lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let a = parts.next().unwrap().parse::<i32>().unwrap();
            let b = parts.next().unwrap().parse::<i32>().unwrap();
            (a, b)
        })
        .unzip();
    
    a.sort_unstable();
    b.sort_unstable();
    (a, b)
}