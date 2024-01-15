fn main() {
    let path = "input/puzzle.txt";
    let sum: usize = std::fs::read_to_string(path).unwrap()
        .split(&"\r\n\r\n")
        .map(| s | {
            let horizontal = horizontal_line(s);
            let vertical = horizontal_line(&flip_string_diagonal(s));
            vertical + 100 * horizontal
        }).sum();

    println!("{}", sum);
}

fn flip_string_diagonal(str: &str) -> String {
    let lines:Vec<&str> = str.lines().collect();
    let len = lines[0].len();
    let mut result = String::new();
    for i in 0..len {
        for line in lines.iter() {
            result.push(line.chars().nth(i).unwrap());
        }
        result.push('\n');
    }
    result
}

fn horizontal_line(str: &str) -> usize {
    let lines = str.lines().collect::<Vec<_>>();
    for (i, _) in lines.iter().enumerate() {
        let mut delta = 0;
        for j in 0..=i {
            if let Some(mirror_line) = lines.get(i+j+1) {
                delta += lines[i-j].chars().zip(mirror_line.chars()).filter(|(a,b)| a != b).count();
                if delta == 1 && i+j+1 == (i * 2 + 1).min(lines.len()-1) {
                    return i+1;
                }
            } 
        }
    }
    0
}