use rayon::{str::ParallelString, iter::ParallelIterator};

enum Token {
    Digit(u32),
    PlayerChar(Color),
    NextRound,
}

enum Color {
    Red,
    Blue,
    Green,
}

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

struct Round {
    red: u32,
    blue: u32,
    green: u32,
}

fn main() {
    let path = "input/puzzle.txt";
    let sum = std::fs::read_to_string(path).unwrap()
        .par_lines()
        .map(|line| {
            parse_line(line.get(5..).unwrap())})
        .filter(|Game { rounds, .. }| {
            rounds.iter()
                .find(|Round { red, blue, green }| *red > 12 || *blue > 14 || *green > 13 )}
                .is_none())
        .map(|Game { id, ..}| { id })
        .sum::<u32>(); 

    println!("Summe: {}", sum);
}

fn parse_line(game: &str) -> Game {
    let id = game.chars()
        .take_while(|c| ('0'..='9').contains(c))
        .fold(0, |acc, c| 10 * acc + c as u32 - 48);
    let mut rounds = Vec::new();
    let mut round = Round { red: 0, blue: 0, green: 0 };
    let mut num = 0;
    for token in parse_token(&game.split(" ").skip(1).collect::<String>()) {
        match token {
            Token::Digit(n) => num = num * 10 + n,
            Token::PlayerChar(p) => {
                if num != 0 {
                    match p {
                        Color::Red => round.red = num,
                        Color::Blue => round.blue = num,
                        Color::Green => round.green = num,
                    }
                    num = 0;
                }
            },
            Token::NextRound => {
                rounds.push(round.into());
                round = Round { red: 0, blue: 0, green: 0 };
            },
        }
    }
    rounds.push(round);
    Game {
        id,
        rounds,
    }
}

fn parse_token(game: &str) -> Vec<Token> {
    game.chars()
        .map(|c| {
            match c {
                '0'..='9' => Some(Token::Digit(c as u32 - 48)),
                'r' => Some(Token::PlayerChar(Color::Red)),
                'b' => Some(Token::PlayerChar(Color::Blue)),
                'g' => Some(Token::PlayerChar(Color::Green)),
                ';' => Some(Token::NextRound),
                _ => None,
            }})
        .flatten()
        .collect()
}