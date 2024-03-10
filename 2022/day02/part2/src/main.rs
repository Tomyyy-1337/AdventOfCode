#[derive(Clone, Copy, PartialEq)]
enum Symbol {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn from_char(c: char) -> Outcome {
        match c {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("Invalid symbol"),
        }
    }
}

impl Symbol {
    fn from_char(c: char) -> Symbol {
        match c {
            'A' => Symbol::Rock,
            'B' => Symbol::Paper,
            'C' => Symbol::Scissors,
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

    fn winns_agains (&self) -> Symbol {
        match self {
            Symbol::Rock => Symbol::Scissors,
            Symbol::Paper => Symbol::Rock,
            Symbol::Scissors => Symbol::Paper,
        }
    }

    fn looses_agains (&self) -> Symbol {
        self.winns_agains().winns_agains()
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
        let outcome = Outcome::from_char(iter.next().unwrap().chars().next().unwrap());
        let me = match outcome {
            Outcome::Win => opponent.looses_agains(),
            Outcome::Lose => opponent.winns_agains(),
            Outcome::Draw => opponent,
        };
        Game { me, opponent }
    }

    fn score(&self) -> u32 {
        self.me.value() + 
        if self.me == self.opponent {
            3
        } else if self.me.winns_agains() == self.opponent {
            6
        } else {
            0
        } 
    }
}

fn main() {
    let path = "input/puzzle.txt";
    let sum: u32 = std::fs::read_to_string(path).unwrap()
        .lines()
        .map(Game::from_str)
        .map(|game| game.score())
        .sum::<u32>();

    println!("{}", sum);
}