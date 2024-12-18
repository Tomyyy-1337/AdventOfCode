use day17::Computer;

fn main() {
    let (computer, instructions) = Computer::from_str("input/puzzle");
    let output = computer.run_programm(&instructions);
    println!("Part 1 result: {}", output);

    let path = "input/puzzle";
    let (computer, instructions) = Computer::from_str(path);
    let program = Computer::get_program_str(path);
    let result = computer.solve_recursive(&instructions, &program);

    println!("Part 2 result: {}", result[0]);
}
