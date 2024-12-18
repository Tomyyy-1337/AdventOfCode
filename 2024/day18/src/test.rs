#[cfg(test)]
mod tests {
    use crate::{Maze, MazeGenerator};

    #[test]
    fn test_maze() {
        let maze = Maze::from_path("input/test", 7,7, 12);
        assert_eq!(maze.find_shortest_path(), Some(22));
    }

    #[test]
    fn test_maze2() {
        let maze = Maze::from_path("input/puzzle", 71,71, 1024);
        assert_eq!(maze.find_shortest_path(), Some(354));
    }

    #[test]
    fn test_maze_generator() {
        let maze_gen = MazeGenerator::from_path(7, 7, "input/test");
        assert_eq!(maze_gen.find_blocking_byte(), Some((6, 1)));
    }
    
    #[test]
    fn test_maze_generator2() {
        let maze_gen = MazeGenerator::from_path(71, 71, "input/puzzle");
        assert_eq!(maze_gen.find_blocking_byte(), Some((36, 17)));
    }
}
