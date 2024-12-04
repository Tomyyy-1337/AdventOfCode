use regex::Regex;

fn main() {
    let contents = std::fs::read_to_string("input/puzzle").unwrap();

    let start = std::time::Instant::now();
    part1(&contents);
    let time_part1 = start.elapsed();
    let start = std::time::Instant::now();
    part_1_alt(&contents);
    let time_part1_alt = start.elapsed();
    part2(&contents);

    println!("Time: {}µs", time_part1.as_micros());
    println!("Time: {}µs", time_part1_alt.as_micros());
}

fn part1(contents: &str) {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let result = re.captures_iter(contents)
        .map(|cap| {
            let a: i32 = cap[1].parse().unwrap();
            let b: i32 = cap[2].parse().unwrap();
            a * b
        })
        .sum::<i32>();

    println!("Part_1 Result: {}", result);
}

fn part2(contents: &str) {
    let re_mult = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))").unwrap();

    let result = re.find_iter(contents)
        .fold((0, true), |(sum, active), m| match m.as_str() {
            "do()" => (sum, true),
            "don't()" => (sum, false),
            m if active => {
                let cap = re_mult.captures(m).unwrap();
                let a: i32 = cap[1].parse().unwrap();
                let b: i32 = cap[2].parse().unwrap();
                (sum + a * b, active)
            }
            _ => (sum, active)
        }).0;

    println!("Part_2 Result: {}", result); 
}

fn part_1_alt(contents: &str) {
    let chars = contents.as_bytes();

    let result = chars.windows(12)
        .filter_map(|w| {
            w.strip_prefix(b"mult(")
                .and_then(|w| w.strip_suffix(b")"))
                .map(|w| w.split(|&c| c == b','))
        })
        .map(|mut it| (it.next().unwrap(), it.next().unwrap()))
        .map(|(a, b)| {
            let a = a.iter().rev().fold(0, |acc, &c| acc * 10 + (c - b'0') as i32);
            let b = b.iter().rev().fold(0, |acc, &c| acc * 10 + (c - b'0') as i32);
            a * b
        })
        .sum::<i32>();

    println!("Part_1 Result: {}", result);
}