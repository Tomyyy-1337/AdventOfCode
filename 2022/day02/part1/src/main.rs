#[derive(Clone, Copy, PartialEq)]
enum Symbol {
    Rock,
    Paper,
    Scissors,
}

impl Symbol {
    fn from_char(c: char) -> Symbol {
        match c {
            'A' | 'X' => Symbol::Rock,
            'B' | 'Y' => Symbol::Paper,
            'C' | 'Z' => Symbol::Scissors,
            _ => panic!("Invalid symbol"),
        }
    }

    fn value(&self) -> u32 {
        match self {
            Symbol::Rock => 1,
            Symbol::Paper => 2,
            Symbol::Scissors => 3,
        }
    }
}

struct Game {
    opponent: Symbol,
    me: Symbol,
}

impl Game {
    fn from_str(s: &str) -> Game {
        let mut iter = s.split_ascii_whitespace();
        let opponent = Symbol::from_char(iter.next().unwrap().chars().next().unwrap());
        let me = Symbol::from_char(iter.next().unwrap().chars().next().unwrap());
        Game { me, opponent }
    }

    fn score(&self) -> u32 {
        let symbol_value = self.me.value();
        if self.me == self.opponent {
            return symbol_value + 3;
        } else if self.winner() == self.me {
            return symbol_value + 6;
        } else {
            return symbol_value;
        }

    }

    fn winner(&self) -> Symbol {
        if self.me as u32 == (self.opponent as u32 + 1) % 3 {
            return self.me;
        } else {
            return self.opponent;
        }
    }
    
}

fn main() {
    let path = "input/puzzle.txt";
    let sum: u32 = std::fs::read_to_string(path).unwrap()
        .lines()
        .map(Game::from_str)
        .map(|game| game.score())
        .sum();

    println!("{}", sum);
}