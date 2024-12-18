Instruction { instruction_type: BST, operand: Register(A) }
Instruction { instruction_type: BXL, operand: Value(1) }
Instruction { instruction_type: CDV, operand: Register(B) }
Instruction { instruction_type: BXL, operand: Value(5) }
Instruction { instruction_type: BXC, operand: Value(0) }
Instruction { instruction_type: OUT, operand: Register(B) }
Instruction { instruction_type: ADV, operand: Value(3) }
Instruction { instruction_type: JNZ, operand: Value(0) }

a = n
b = 0
c = 0

while a > 0 {
    b = a & 0x111 
    b = b ^ 1
    c = a >> b
    b = b ^ 5
    b = b ^ c
    print b & 0x111
    a = a >> 3
}

