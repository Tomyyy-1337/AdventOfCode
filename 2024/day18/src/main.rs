use day18::{find_first_parallel, Maze, MazeGenerator};

fn main() {
    let maze = Maze::from_path("input/puzzle", 71,71, 1024);
    let result = maze.find_shortest_path();
    
    println!("The shortest path is: {}", result.unwrap());
    
    let start = std::time::Instant::now();
    let maze_gen = MazeGenerator::from_path(71, 71, "input/puzzle");
    let result = maze_gen.find_blocking_byte().unwrap();
    
    println!("The blocking byte is at: {:?}", result);
    println!("Elapsed time: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let result = find_first_parallel("input/puzzle", 71, 71, 50).unwrap();
    println!("The blocking byte is at: {:?}", result);
    println!("Elapsed time: {:?}", start.elapsed());
    
}