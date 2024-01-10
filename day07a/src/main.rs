#[derive(PartialEq, Eq, Clone, Copy)]
enum Card {
    Number(u8),
    T,
    J,
    Q,
    K,
    A,
}

struct Game {
    cards: Vec<Card>,
    bet: u64,
}

impl Game {
    pub fn eval_cards(&self) -> u64 {
        self.cards.iter() 
            .fold(0, |mut eval, card|{
                eval += match card {
                    Card::Number(n) => *n as u64,
                    Card::T => 10,
                    Card::J => 11,
                    Card::Q => 12,
                    Card::K => 13,
                    Card::A => 14,
                };
                eval +=  if self.count_n_pairs(5) == 1 { 6000000 }
                    else if self.count_n_pairs(4) == 1 { 5000000 }
                    else if self.count_n_pairs(3) == 1 && self.count_n_pairs(2) == 1 { 4000000 }
                    else if self.count_n_pairs(3) == 1 { 3000000 }
                    else if self.count_n_pairs(2) == 2 { 2000000 }
                    else if self.count_n_pairs(2) == 1 { 1000000 }
                    else { 0 };
                eval as u64 * 15
            })
    }

    fn count_n_pairs(&self, number: usize) -> usize {
        self.cards.iter()
            .map(| c1 | self.cards.iter().filter(|&c2| c2.eq(c1)).count())
            .filter(| &c | c == number)
            .count() / number
    }
}

fn main() {
    let path = "input/puzzle.txt";
    let mut games = std::fs::read_to_string(path).unwrap()
        .lines()
        .map(|line| line.split_ascii_whitespace())
        .map(|mut iter| {
            let cards: Vec<Card> = iter.next().unwrap()
                .chars()
                .map(|c| match c {
                    'T' => Card::T,
                    'J' => Card::J,
                    'Q' => Card::Q,
                    'K' => Card::K,
                    'A' => Card::A,
                    _ => Card::Number(c.to_digit(10).unwrap() as u8),
                }).collect::<Vec<_>>();
            let bet = iter.next().unwrap().parse::<u64>().unwrap();
            Game { cards, bet }
        }).collect::<Vec<_>>();

    games.sort_by_key(|game| game.eval_cards());
    
    let sum = games.iter()
        .enumerate()
        .map(|(i,game)| (game.bet * (i as u64 +1)) as u64)
        .sum::<u64>() ;
    
    println!("{}", sum);
}