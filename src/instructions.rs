use crate::init::Register;

/// ## Instruction
///
/// Represents an assembly instruction for x86‑64 architectures.
/// 
/// ### Example in Rust:
/// ```rust
/// linuxx86.emit(Instruction::MovImm { dst: Register::RAX, imm: 42 });
/// ```
#[derive(Debug)]
pub enum Instruction {
    MovF {
        dst: Register,
        imm: f64,
    },
    ///Lea into var
    LeaIntoVar {
        reg: Register,
        var_name: &'static str,
    },
    /// Moves a variable into a register.
    MovIntoVar {
        reg: Register,
        var_name: &'static str,
    },
    /// Moves a register into a variable.
    MovFromVar {
        var_name: &'static str,
        reg: Register,
    },
    /// copy RCX bytes from [RSI] to [RDI]
    RepRsiRdi,
    /// Moves an immediate value into a register.
    MovImm { dst: Register, imm: i64 },
    /// Moves the value from one register to another.
    Mov { dst: Register, src: Register },
    /// Adds the value of one register to another.
    Add { dst: Register, src: Register },
    /// Subtracts the value of one register from another.
    Sub { dst: Register, src: Register },
    /// Multiplies the value of one register with another.
    Mul { dst: Register, src: Register },
    /// Divides the value of one register by another.
    Div { src: Register },
    /// Performs a bitwise AND between two registers.
    And { dst: Register, src: Register },
    /// Performs a bitwise OR between two registers.
    Or { dst: Register, src: Register },
    /// Performs a bitwise XOR between two registers.
    Xor { dst: Register, src: Register },
    /// Performs a bitwise NOT on a register.
    Not { reg: Register },
    /// Shifts the value in a register to the left.
    Shl { dst: Register, src: Register },
    /// Shifts the value in a register to the right.
    Shr { dst: Register, src: Register },
    /// Pushes a register onto the stack.
    Push { reg: Register },
    /// Pops a register from the stack.
    Pop { reg: Register },
    /// Calls a function.
    Call(String),
    /// Returns from a function.
    Ret,
    /// Unconditionally jumps to a label.
    Jmp(String),
    /// Defines a label.
    Label(String),
    /// Compares two registers.
    Cmp { op1: Register, op2: Register },
    /// Jumps to a label if equal.
    Je(String),
    /// Jumps to a label if not equal.
    Jne(String),
    /// Jumps to a label if greater.
    Jg(String),
    /// Jumps to a label if greater or equal.
    Jge(String),
    /// Jumps to a label if less.
    Jl(String),
    /// Jumps to a label if less or equal.
    Jle(String),
    /// Moves a register’s value into memory at the address held in another register.
    MovToMem { src: Register, addr: Register },
    /// Loads a value from memory at the address held in a register into another register.
    MovFromMem { addr: Register, dst: Register },
    /// Adds an immediate value directly to a register.
    AddImm { dst: Register, imm: i64 },
    /// Inserts plain assembly code “as is” into the output.
    AsIs(&'static str),
    SYSCALL,
}
