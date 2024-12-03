use regex::Regex;

fn main() {
    let contents = std::fs::read_to_string("input/puzzle").unwrap();

    part1(&contents);
    part2(&contents);
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
    let re_do = Regex::new(r"do\(\)").unwrap(); 
    let re_dont = Regex::new(r"don't\(\)").unwrap();

    let mults = re_mult.captures_iter(contents)
        .map(|cap| {
            let start_index = cap.get(0).unwrap().start();
            let v1: u32 = cap[1].parse().unwrap();
            let v2: u32 = cap[2].parse().unwrap();
            (start_index, v1 * v2)
        })
        .collect::<Vec<(usize, u32)>>();

    let mut dos = re_do.captures_iter(contents)
        .map(|cap| {
            let start_index: usize = cap.get(0).unwrap().start();
            start_index
        })
        .peekable();

    let mut donts = re_dont.captures_iter(contents)
        .map(|cap| {
            let start_index = cap.get(0).unwrap().start();
            start_index
        })
        .peekable();

    let mut active_instructions = Vec::new();   
    while let (Some(doo), Some(dont)) = (dos.peek(), donts.peek()) {
        if doo < dont {
            active_instructions.push((*doo, true));
            dos.next();
        } else {
            active_instructions.push((*dont, false));
            donts.next();
        }
    }
    dos.for_each(|doo| active_instructions.push((doo, true)));
    donts.for_each(|dont| active_instructions.push((dont, false)));

    let mut active_instructions = active_instructions.iter().peekable();

    let mut result = 0;
    let mut active = true;
    for (i, mult) in mults.iter() {
        while let Some((index, is_do)) = active_instructions.peek() {
            if *index < *i {
                active = *is_do;
                active_instructions.next();
            } else {
                break;
            }
        }
        if active {
            result += mult;
        }
    }

    println!("Part_2 Result: {}", result);
}