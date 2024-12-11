use hashbrown::HashMap;

fn main() { 
    let content = include_str!("../input/puzzle");

    let start = std::time::Instant::now();

    solve(content, 25);
    solve(content, 75);

    println!("Time: {:?}", start.elapsed());
}

fn solve(content: &str, iterations: u64) {
    let mut stones = content
        .split(" ")
        .flat_map(str::parse::<u64>)
        .fold(HashMap::new(), |mut map, num| {
            *map.entry(num).or_insert(0u64) += 1;
            map
        });

    for _ in 0..iterations {
        let mut new_stones = HashMap::new();

        for (num, count) in stones {
            match num {
                0 => {
                    *new_stones.entry(1).or_insert(0) += count;
                },
                _ => match self::split_number(num) {
                    Some((a,b)) => {
                        *new_stones.entry(a).or_insert(0) += count;
                        *new_stones.entry(b).or_insert(0) += count;
                    },
                    None => *new_stones.entry(num * 2024).or_insert(0) += count,
                }
            }
        }

        stones = new_stones;
    }

    let result = stones
        .iter()
        .map(|(_, count)| count)
        .sum::<u64>();

    println!("Result: {}", result);
}

fn split_number(num: u64) -> Option<(u64,u64)> {
    let mut length = 0;
    let mut a = num;
    while a > 0 {
        a /= 10;
        length += 1;
    }
    if length % 2 != 0 {
        return None;
    }
    let (mut a, mut b) = (num, 0);
    for i in 0..length/2 {
        b += (a % 10) * 10u64.pow(i);
        a /= 10;
    }
    return Some((a,b));
}
