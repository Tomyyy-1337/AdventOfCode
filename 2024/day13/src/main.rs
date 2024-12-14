fn main() {
    let contents = include_str!("../input/puzzle");

    solve(contents, 0);
    solve(contents, 10000000000000);
}

fn solve(contents: &str, prize_offset: i64) {
    let puzzles: Vec<_> = contents
        .split("\r\n\r\n")
        .map(|x| Machine::from_str(x, prize_offset))
        .collect();

    let result: i64 = puzzles
        .iter()
        .flat_map(|x| x.solve())
        .sum();

    println!("Result: {}", result);
}

#[derive(Debug)]
struct Machine {
    a_button: (i64,i64),
    b_button: (i64,i64),
    prize: (i64,i64),
}

impl Machine {
    fn from_str(input: &str, prize_offset: i64) -> Self {
        let re = regex::Regex::new(r"(\d+)[^\d]+(\d+)").unwrap();
        let mut iter = input.lines();
        let a_button = re.captures(iter.next().unwrap()).unwrap();
        let b_button = re.captures(iter.next().unwrap()).unwrap();
        let prize = re.captures(iter.next().unwrap()).unwrap();

        Machine {
            a_button: (a_button[1].parse().unwrap(), a_button[2].parse().unwrap()),
            b_button: (b_button[1].parse().unwrap(), b_button[2].parse().unwrap()),
            prize: (prize[1].parse::<i64>().unwrap() + prize_offset, prize[2].parse::<i64>().unwrap() + prize_offset),
        }
    }

    #[allow(dead_code)]
    fn solve_brute_force(&self, max_depth: i64) -> Option<i64> {
        (0..=max_depth).find_map(|a| (0..max_depth).find_map(|b| {
            if self.a_button.0 * a + self.b_button.0 * b == self.prize.0 && self.a_button.1 * a + self.b_button.1 * b == self.prize.1 {
                Some(a * 3 + b)
            } else {
                None
            }
        }))
    }

    fn solve(&self) -> Option<i64> {
        let b_num = self.prize.0 * self.a_button.1 - self.prize.1 * self.a_button.0;
        let b_den = self.b_button.0 * self.a_button.1 - self.b_button.1 * self.a_button.0;
        if b_den == 0 || b_num % b_den != 0 {
            return None;
        } 
        let b = b_num / b_den;
        let a = self.prize.0 - self.b_button.0 * b;
        if self.a_button.0 == 0 || a % self.a_button.0 != 0 {
            return None;
        }
        let a = a / self.a_button.0;
        Some(a * 3 + b)
    }
}