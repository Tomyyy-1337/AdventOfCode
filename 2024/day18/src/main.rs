use day18::{Maze, MazeGenerator};

fn main() {
    let maze = Maze::from_path("input/puzzle", 71,71, 1024);
    let result = maze.find_shortest_path();
    
    println!("The shortest path is: {}", result.unwrap());
    
    let start = std::time::Instant::now();
    let maze_gen = MazeGenerator::from_path(71, 71, "input/puzzle");
    let result = maze_gen.find_blocking_byte().unwrap();
    println!("Elapsed time: {:?}", start.elapsed());
    
    println!("The blocking byte is at: {:?}", result);
    
}