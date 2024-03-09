fn main() {
    let path = "input/puzzle.txt";
    let sum = std::fs::read_to_string(path).unwrap()
        .lines()
        .map(|line| line.split_ascii_whitespace().map(|str| str.parse::<i64>().unwrap()).collect::<Vec<_>>())
        .map(|line| find_next_number(line.into_iter().rev().collect::<Vec<_>>()))
        .sum::<i64>();

    println!("Sum: {}", sum);
}

fn find_next_number(seqence: Vec<i64>) -> i64 {
    let arr: Vec<i64> = seqence.windows(2).map(|w| w[1] - w[0]).collect();
    if arr.iter().find(|&x| x != &seqence[0]).is_none() {
        return seqence.last().unwrap() + arr[0];
    }
    return seqence.last().unwrap() + find_next_number(arr);
}