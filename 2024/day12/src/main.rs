mod part1;
mod part2;


fn main() {
    let contents = include_str!("../input/puzzle");
    
    let grid = part1::Grid::from_str(contents);
    grid.solve();

    let start = std::time::Instant::now();
    let grid = part2::Grid::from_str(contents);
    grid.solve();
    println!("Part 2: {:?}", start.elapsed());
}

