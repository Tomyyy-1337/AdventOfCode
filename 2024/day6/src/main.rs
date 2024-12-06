mod part1;
mod part2;

fn main() {
    let contents = std::fs::read_to_string("input/puzzle").unwrap();

    // Part 1
    let maze = part1::Maze::from_str(&contents);
    let maze = maze.solve();
    println!("Solution: {}", maze.solution());

    // Part 2
    let start = std::time::Instant::now();

    let maze = part2::Maze::from_str(&contents);
    let result = (0..maze.get_number_of_cells())
        .filter(|&index| match maze.create_with_extra_wall(index) {
            Some(maze) => maze.has_loop(),
            None => false,
        })
        .count();

    let elapsed = start.elapsed();
    println!("Elapsed: {:?}", elapsed);
    println!("Solution: {}", result);
}
