mod test;

#[derive(Debug, Clone)]
pub struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    instruction_pointer: u64,
}

impl Computer {
    pub fn solve_recursive(&self ,instructions: &[Instruction], result: &str) -> Vec<u64> {
        let nums = match result.len() {
            1 => vec![0],
            _ => self.solve_recursive(instructions, &result[2..]),
        };

        nums.iter()
            .map(|x| x << 3)
            .flat_map(|num | (num..=num + 7)
                .into_iter()
                .filter(|&i| {
                    let computer = self.set_register_a(i);
                    let output = computer.run_programm(instructions);
                    output == result
                })
            ).collect::<Vec<u64>>()
    }

    pub fn set_register_a(&self, value: u64) -> Computer {
        Computer {
            register_a: value,
            register_b: self.register_b,
            register_c: self.register_c,
            instruction_pointer: 0,
        }
    }

    pub fn get_program_str(path: &str) -> String {
        let content = std::fs::read_to_string(path).unwrap();
        let (_, instructions) = content.split_once("\r\n\r\n").unwrap();
        instructions.strip_prefix("Program: ").unwrap().to_string()
    }

    pub fn from_str(path: &str) -> (Computer, Vec<Instruction>) {
        let content = std::fs::read_to_string(path).unwrap();
        let (registers, instructions) = content.split_once("\r\n\r\n").unwrap();

        let mut registers = registers
            .lines()
            .flat_map(|line| line.split(": ").skip(1).next().unwrap().parse::<u64>());

        let computer  = Computer {
            register_a: registers.next().unwrap(),
            register_b: registers.next().unwrap(),
            register_c: registers.next().unwrap(),
            instruction_pointer: 0,
        };

        let instructions = Computer::read_instructions(instructions);

        (computer, instructions)
    }

    fn read_instructions(instructions: &str) -> Vec<Instruction> {
        let instructions = instructions
            .strip_prefix("Program: ")
            .unwrap()
            .split(",")
            .map(|s| s.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
    
        let instructions = instructions
            .chunks(2)
            .map(|chunk| Instruction::from_raw(chunk[0], chunk[1]))
            .collect::<Vec<Instruction>>();
        
        instructions
    }

    pub fn run_programm(mut self, instructions: &[Instruction]) -> String {
        let mut output_buffer = Vec::new();
        while self.instruction_pointer < instructions.len() as u64 {
            let mut increment = true;
            let instruction = instructions[self.instruction_pointer as usize];
            let operand = self.evaluate_operand(instruction.operand);
            match instruction.instruction_type {
                InstructionType::BXL => self.register_b ^= operand,
                InstructionType::BST => self.register_b = operand & 0b111,
                InstructionType::OUT => output_buffer.push(operand & 0b111),
                InstructionType::BXC => self.register_b ^= self.register_c,
                InstructionType::ADV => self.register_a = self.register_a >> operand,
                InstructionType::BDV => self.register_b = self.register_a >> operand,
                InstructionType::CDV => self.register_c = self.register_a >> operand,
                InstructionType::JNZ => {
                    if self.register_a != 0 {
                        self.instruction_pointer = operand >> 1;
                        increment = false;
                    }
                }
            }
            if increment {
                self.instruction_pointer += 1;
            }
        }
        output_buffer.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")
    }

    fn evaluate_operand(&self, operand: Operand) -> u64 {
        match operand {
            Operand::Register( Register::A ) => self.register_a,
            Operand::Register( Register::B ) => self.register_b,
            Operand::Register( Register::C ) => self.register_c,
            Operand::Value( value ) => value as u64,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum InstructionType {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV
}

impl InstructionType {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => InstructionType::ADV,
            1 => InstructionType::BXL,
            2 => InstructionType::BST,
            3 => InstructionType::JNZ,
            4 => InstructionType::BXC,
            5 => InstructionType::OUT,
            6 => InstructionType::BDV,
            7 => InstructionType::CDV,
            _ => panic!("Invalid instruction type"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Instruction {
    instruction_type: InstructionType,
    operand: Operand,
}

impl Instruction {
    fn from_raw(instruction: u8, operand: u8) -> Instruction {
        Instruction {
            instruction_type: InstructionType::from_u8(instruction),
            operand: Operand::from_raw(instruction, operand),
        }
    }
}

#[derive(Copy, Clone, Debug)]  
enum Operand {
    Register( Register ),
    Value( u8 ),
}

impl Operand {
    fn from_raw(operand: u8, value: u8) -> Self {
        match (operand, value) {
            (1, value) | (3, value) | (4, value) => Operand::Value(value),
            (_, 4) => Operand::Register(Register::A),
            (_, 5) => Operand::Register(Register::B),
            (_, 6) => Operand::Register(Register::C),
            (_, value @ 0..=3) => Operand::Value(value),
            _ => panic!("Invalid operand"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Register {
    A,
    B,
    C,
}