mod part1;
mod part2;

fn main() {
    let contents = include_str!("../input/puzzle");

    let mut game = part1::Game::from_str(contents);
    game.solve();
}

