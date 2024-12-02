fn main() {
    let contents = std::fs::read_to_string("input/puzzle").unwrap();

    part1(&contents);

    part2(&contents);
}

fn part1(contents: &str) {
    let result = contents.lines()
        .map(|line| line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .filter(|x|{
            (x.windows(2).all(|y| y[0] < y[1])
            || x.windows(2).all(|y| y[0] > y[1]))
            && x.windows(2).all(|y| (1..=3).contains(&(y[0] - y[1]).abs())) 
        })
        .count();

    println!("Part_1 result: {}", result);
}   

fn part2(contents: &str) {  
    let numbers = contents.lines()
        .map(|line| line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .map(|x| {
            (0..x.len()).map(move |i| {
                let mut x = x.clone();
                x.remove(i);
                x
            })
            .filter(|x|{
                (x.windows(2).all(|y| y[0] < y[1])
                || x.windows(2).all(|y| y[0] > y[1]))
                && x.windows(2).all(|y| (1..=3).contains(&(y[0] - y[1]).abs())) 
            })
        })
        .map(|mut x| x.next().is_some())
        .filter(|x| *x)
        .count();

    println!("Part_2 result: {}", numbers);
}
