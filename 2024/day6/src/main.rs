use rayon::iter::{IntoParallelIterator, ParallelIterator};

mod part1;
mod part2;
mod bool_array_8;
mod gui;

fn main() {
    let contents = std::fs::read_to_string("input/puzzle").unwrap();

    // Part 1
    let start = std::time::Instant::now();

    let maze = part1::Maze::from_str(&contents);
    let maze = maze.solve();

    let elapsed = start.elapsed();
    println!("Found part 1 solution: {} in {:?}", maze.solution(), elapsed);

    // Part 2
    let start = std::time::Instant::now();
    let indexes = maze.get_visited_index();    

    let maze = part2::Maze::from_str(&contents);
    let result = indexes
        .into_par_iter()
        .filter(|&index| match maze.create_with_extra_wall(index) {
            Some(maze) => maze.has_loop(),
            None => false,
        })
        .count();
    
    let elapsed = start.elapsed();
    println!("Found part 2 solution: {} in {:?}", result, elapsed);

    // Part 2 Visualisation
    nannou::app(gui::Model::model).update(gui::Model::update).run();
}
