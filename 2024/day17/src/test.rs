#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_computer_part_1_test() {
        let (computer, instructions) = Computer::from_str("input/test");
        let output = computer.run_programm(&instructions);
        assert_eq!(output, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_computer_part_1_puzzle() {
        let (computer, instructions) = Computer::from_str("input/puzzle");
        let output = computer.run_programm(&instructions);
        assert_eq!(output, "4,1,7,6,4,1,0,2,7");
    }

    #[test]
    fn test_computer_part_2_puzzle() {
        let (computer, instructions) = Computer::from_str("input/puzzle");
        let program = Computer::get_program_str("input/puzzle");
        let result = computer.solve_recursive(&instructions, &program);
        assert_eq!(result[0], 164279024971453);
    }

}
