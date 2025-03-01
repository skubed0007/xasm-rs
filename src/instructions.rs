use crate::init::Register;


/// ## Instruction
///
/// Represents an assembly instruction for x86‑64 architectures.
/// and providing a brief Rust example.
#[derive(Debug)]
pub enum Instruction {
    /// Moves a variable into a register.
    ///
    /// **Assembly:** `mov <reg>, <var_name>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::MovVar { reg: Register::RSI, var_name: "v1" });
    /// ```
    MovVar {
        reg: Register,
        var_name: &'static str,
    },
    /// Moves an immediate value into a register.
    ///
    /// **Assembly:** `mov <dst>, <imm>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::MovImm { dst: Register::RAX, imm: 42 });
    /// ```
    MovImm { dst: Register, imm: i64 },
    /// Moves the value from one register to another.
    ///
    /// **Assembly:** `mov <dst>, <src>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::Mov { dst: Register::RBX, src: Register::RAX });
    /// ```
    Mov { dst: Register, src: Register },
    /// Adds the value of one register to another.
    ///
    /// **Assembly:** `add <dst>, <src>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::Add { dst: Register::RAX, src: Register::RBX });
    /// ```
    Add { dst: Register, src: Register },
    /// Subtracts the value of one register from another.
    ///
    /// **Assembly:** `sub <dst>, <src>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::Sub { dst: Register::RAX, src: Register::RCX });
    /// ```
    Sub { dst: Register, src: Register },
    /// Multiplies the value of one register with another.
    ///
    /// **Assembly:** `mul <dst>, <src>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::Mul { dst: Register::RAX, src: Register::RDX });
    /// ```
    Mul { dst: Register, src: Register },
    /// Divides the value of one register by another.
    ///
    /// **Assembly:** `div <dst>, <src>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::Div { dst: Register::RAX, src: Register::RBX });
    /// ```
    Div { dst: Register, src: Register },
    /// Performs a bitwise AND between two registers.
    ///
    /// **Assembly:** `and <dst>, <src>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::And { dst: Register::RAX, src: Register::RCX });
    /// ```
    And { dst: Register, src: Register },
    /// Performs a bitwise OR between two registers.
    ///
    /// **Assembly:** `or <dst>, <src>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::Or { dst: Register::RAX, src: Register::RCX });
    /// ```
    Or { dst: Register, src: Register },
    /// Performs a bitwise XOR between two registers.
    ///
    /// **Assembly:** `xor <dst>, <src>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::Xor { dst: Register::RAX, src: Register::RCX });
    /// ```
    Xor { dst: Register, src: Register },
    /// Performs a bitwise NOT on a register.
    ///
    /// **Assembly:** `not <reg>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::Not { reg: Register::RAX });
    /// ```
    Not { reg: Register },
    /// Shifts the value in a register to the left by the number of bits in another register.
    ///
    /// **Assembly:** `shl <dst>, <src>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::Shl { dst: Register::RAX, src: Register::RCX });
    /// ```
    Shl { dst: Register, src: Register },
    /// Shifts the value in a register to the right by the number of bits in another register.
    ///
    /// **Assembly:** `shr <dst>, <src>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::Shr { dst: Register::RAX, src: Register::RCX });
    /// ```
    Shr { dst: Register, src: Register },
    /// Pushes a register onto the stack.
    ///
    /// **Assembly:** `push <reg>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::Push { reg: Register::RAX });
    /// ```
    Push { reg: Register },
    /// Pops a register from the stack.
    ///
    /// **Assembly:** `pop <reg>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::Pop { reg: Register::RAX });
    /// ```
    Pop { reg: Register },
    /// Calls a function.
    ///
    /// **Assembly:** `call <function>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::Call("function_name".to_string()));
    /// ```
    Call(String),
    /// Returns from a function.
    ///
    /// **Assembly:** `ret`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::Ret);
    /// ```
    Ret,
    /// Unconditionally jumps to a label.
    ///
    /// **Assembly:** `jmp <label>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::Jmp("label".to_string()));
    /// ```
    Jmp(String),
    /// Defines a label.
    ///
    /// **Assembly:** `<label>:`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::Label("start".to_string()));
    /// ```
    Label(String),
    /// Compares two registers.
    ///
    /// **Assembly:** `cmp <op1>, <op2>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::Cmp { op1: Register::RAX, op2: Register::RBX });
    /// ```
    Cmp { op1: Register, op2: Register },
    /// Jumps to a label if the previous comparison resulted in equality.
    ///
    /// **Assembly:** `je <label>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::Je("equal_label".to_string()));
    /// ```
    Je(String),
    /// Jumps to a label if the previous comparison resulted in inequality.
    ///
    /// **Assembly:** `jne <label>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::Jne("not_equal_label".to_string()));
    /// ```
    Jne(String),
    /// Jumps to a label if the first operand is greater than the second.
    ///
    /// **Assembly:** `jg <label>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::Jg("greater_label".to_string()));
    /// ```
    Jg(String),
    /// Jumps to a label if the first operand is greater than or equal to the second.
    ///
    /// **Assembly:** `jge <label>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::Jge("greater_equal_label".to_string()));
    /// ```
    Jge(String),
    /// Jumps to a label if the first operand is less than the second.
    ///
    /// **Assembly:** `jl <label>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::Jl("less_label".to_string()));
    /// ```
    Jl(String),
    /// Jumps to a label if the first operand is less than or equal to the second.
    ///
    /// **Assembly:** `jle <label>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::Jle("less_equal_label".to_string()));
    /// ```
    Jle(String),
    /// Triggers a system call.
    ///
    /// **Assembly:** `syscall`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// linuxx86.emit(Instruction::SYSCALL);
    /// ```
    SYSCALL,
}
