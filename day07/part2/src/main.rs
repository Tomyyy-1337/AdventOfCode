#[derive(PartialEq, Eq, Clone, Copy)]
enum Card {
    Number(u8),
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn new(c: char) -> Card {
        match c {
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => Card::Number(c.to_digit(10).unwrap() as u8),
        }
    }
}

const CARDS: [Card; 12] = [Card::Number(2), Card::Number(3), Card::Number(4), Card::Number(5), Card::Number(6), Card::Number(7), Card::Number(8), Card::Number(9), Card::Q, Card::K, Card::A, Card::T];

struct Game {
    cards: Vec<Card>,
    bet: u64,
}

impl Game {
    pub fn new(line: &str) -> Game {
        let line_vec = line.split_ascii_whitespace().collect::<Vec<_>>();
        Game { 
            cards: line_vec[0]
                .chars()
                .map(|c| Card::new(c))
                .collect::<Vec<_>>(),
            bet: line_vec[1].parse::<u64>().unwrap(),
        }
    }

    pub fn eval_cards(&self) -> u64 {
        let mut hands = vec![self.cards.clone()];
        let mut hands_indx = 0;
        while hands_indx < hands.len() {
            if let Some(index) = hands[hands_indx].iter().position(|&c| c == Card::J) {
                for c in CARDS.iter() {
                    let mut new_hand = hands[hands_indx].clone();
                    new_hand[index] = *c;
                    hands.push(new_hand);
                }
            }
            hands_indx += 1;
        }
        hands.into_iter()
            .filter(|h| !h.contains(&Card::J))
            .map(|cards| self.eval_hand(&cards))
            .max().unwrap()
        }

    fn eval_hand(&self, cards: &Vec<Card>) -> u64 {
        self.cards.iter().fold(0, |mut eval, card|{
            eval += match card {
                Card::Number(n) => *n as u64,
                Card::J => 1,
                Card::T => 10,
                Card::Q => 11,
                Card::K => 12,
                Card::A => 13,
            };
            eval as u64 * 14
        }) + if Self::count_n_pairs(cards, 5) == 1 { 60000000 }
        else if Self::count_n_pairs(cards, 4) == 1 { 50000000 }
        else if Self::count_n_pairs(cards, 3) == 1 && Self::count_n_pairs(cards, 2) == 1 { 40000000 }
        else if Self::count_n_pairs(cards, 3) == 1 { 30000000 }
        else if Self::count_n_pairs(cards, 2) == 2 { 20000000 }
        else if Self::count_n_pairs(cards, 2) == 1 { 10000000 }
        else { 0 }
    }

    fn count_n_pairs(cards: &Vec<Card>, number: usize) -> usize {
        cards.iter()
            .map(| c1 | cards.iter().filter(|&c2| c2.eq(c1)).count())
            .filter(| &c | c == number)
            .count() / number
    }
}

fn main() {
    let path = "input/puzzle.txt";
    let mut games = std::fs::read_to_string(path).unwrap()
        .lines()
        .map(|line| Game::new(line))
        .collect::<Vec<_>>();

    games.sort_by_cached_key(|game| game.eval_cards());
    
    let sum = games.iter()
        .enumerate()
        .map(|(i,game)| (game.bet * (i as u64 +1)) as u64)
        .sum::<u64>();
    
    println!("{}", sum);
}