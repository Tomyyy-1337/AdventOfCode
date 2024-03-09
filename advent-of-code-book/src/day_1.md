# Day 1
[Advent of Code - Day 1](https://adventofcode.com/2022/day/1)

## Part 1


```rust
fn main() {
    let path = "./input/puzzle.txt";
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

```

## Part 2
```rust
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

```

# Execution Time:
Running on I7 12700K 

|  | Part 1 | Part 2 |
|-|-|-|
|   Single-Run              |   91.30µs   |   110µs   |
|   1000 consecutive runs   |   40.09ms   |    49.48ms |