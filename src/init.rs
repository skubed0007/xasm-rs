use std::collections::VecDeque;
use std::fmt;

/// ## LinuxX8664
///
/// `LinuxX8664` is the core parent struct for the Linux x86‑64 assembly generator.
/// It encapsulates the underlying assembly generator (`Xasm`), which in turn manages
/// register allocation, instruction emission, and variable management.
pub struct LinuxX8664 {
    /// The underlying assembly generator.
    pub xasm: Xasm,
}

impl LinuxX8664 {
    /// Creates a new `LinuxX8664` instance with an empty assembly generator.
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            xasm: Xasm::new(),
        }
    }
}

/// ## Register
///
/// Represents the CPU registers available in x86‑64.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    RAX, RBX, RCX, RDX, RSI, RDI, RBP, RSP,
    R8, R9, R10, R11, R12, R13, R14, R15,
}

/// ## RegisterAllocator
///
/// Manages the allocation and tracking of registers.
#[derive(Debug)]
pub struct RegisterAllocator {
    free_regs: VecDeque<Register>,
    used_regs: Vec<Register>,
}

impl RegisterAllocator {
    /// Creates a new `RegisterAllocator` with all registers available.
    #[inline(always)]
    pub fn new() -> Self {
        let free_regs = VecDeque::from(vec![
            Register::RAX, Register::RBX, Register::RCX, Register::RDX,
            Register::RSI, Register::RDI, Register::RBP, Register::RSP,
            Register::R8,  Register::R9,  Register::R10, Register::R11,
            Register::R12, Register::R13, Register::R14, Register::R15,
        ]);
        Self { free_regs, used_regs: Vec::new() }
    }

    /// Allocates a free register. If `force` is true and no free register is available,
    /// it will free the most recently used register and allocate it.
    #[inline(always)]
    pub fn allocate(&mut self, force: bool) -> Register {
        if let Some(reg) = self.free_regs.pop_front() {
            self.used_regs.push(reg);
            reg
        } else if force && !self.used_regs.is_empty() {
            let reg = self.used_regs.pop().unwrap();
            self.free_regs.push_back(reg);
            let new_reg = self.free_regs.pop_front().expect("No register available after forcing");
            self.used_regs.push(new_reg);
            new_reg
        } else if let Some(reg) = self.free_regs.pop_front() {
            self.used_regs.push(reg);
            reg
        } else {
            panic!("No registers available");
        }
    }

    /// Retrieves a specific register if available; if not, allocates another free register.
    /// If `force` is true and the specific register is in use, it will free it first.
    #[inline(always)]
    pub fn get_specific(&mut self, reg: Register, force: bool) -> Register {
        if let Some(pos) = self.free_regs.iter().position(|&r| r == reg) {
            self.free_regs.remove(pos);
            self.used_regs.push(reg);
            reg
        } else if self.used_regs.contains(&reg) {
            if force {
                self.free(reg);
                if let Some(pos) = self.free_regs.iter().position(|&r| r == reg) {
                    self.free_regs.remove(pos);
                    self.used_regs.push(reg);
                    reg
                } else {
                    self.allocate(false)
                }
            } else {
                self.allocate(false)
            }
        } else {
            self.allocate(false)
        }
    }

    /// Frees a register, returning it to the pool of free registers.
    #[inline(always)]
    pub fn free(&mut self, reg: Register) {
        if let Some(pos) = self.used_regs.iter().position(|&r| r == reg) {
            self.used_regs.remove(pos);
            self.free_regs.push_back(reg);
        }
    }

    /// Resets the allocator by freeing all used registers.
    #[inline(always)]
    pub fn reset(&mut self) {
        while let Some(reg) = self.used_regs.pop() {
            self.free_regs.push_back(reg);
        }
    }

    /// Returns a slice of registers currently in use.
    #[inline(always)]
    pub fn used(&self) -> &[Register] {
        &self.used_regs
    }

    /// Returns a vector of all free registers.
    #[inline(always)]
    pub fn free_list(&self) -> Vec<Register> {
        self.free_regs.iter().copied().collect()
    }
}

/// ## Instruction
///
/// Represents an assembly instruction for x86‑64 architectures.
/// Each variant’s documentation appears on hover, explaining the assembly mnemonic
/// and providing a brief Rust example.
#[derive(Debug)]
pub enum Instruction {
    /// Moves a variable into a register.
    ///
    /// **Assembly:** `mov <reg>, <var_name>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::MovVar { reg: Register::RSI, var_name: "v1" });
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
    /// xasm.emit(Instruction::MovImm { dst: Register::RAX, imm: 42 });
    /// ```
    MovImm { dst: Register, imm: i64 },
    /// Moves the value from one register to another.
    ///
    /// **Assembly:** `mov <dst>, <src>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::Mov { dst: Register::RBX, src: Register::RAX });
    /// ```
    Mov { dst: Register, src: Register },
    /// Adds the value of one register to another.
    ///
    /// **Assembly:** `add <dst>, <src>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::Add { dst: Register::RAX, src: Register::RBX });
    /// ```
    Add { dst: Register, src: Register },
    /// Subtracts the value of one register from another.
    ///
    /// **Assembly:** `sub <dst>, <src>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::Sub { dst: Register::RAX, src: Register::RCX });
    /// ```
    Sub { dst: Register, src: Register },
    /// Multiplies the value of one register with another.
    ///
    /// **Assembly:** `mul <dst>, <src>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::Mul { dst: Register::RAX, src: Register::RDX });
    /// ```
    Mul { dst: Register, src: Register },
    /// Divides the value of one register by another.
    ///
    /// **Assembly:** `div <dst>, <src>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::Div { dst: Register::RAX, src: Register::RBX });
    /// ```
    Div { dst: Register, src: Register },
    /// Performs a bitwise AND between two registers.
    ///
    /// **Assembly:** `and <dst>, <src>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::And { dst: Register::RAX, src: Register::RCX });
    /// ```
    And { dst: Register, src: Register },
    /// Performs a bitwise OR between two registers.
    ///
    /// **Assembly:** `or <dst>, <src>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::Or { dst: Register::RAX, src: Register::RCX });
    /// ```
    Or { dst: Register, src: Register },
    /// Performs a bitwise XOR between two registers.
    ///
    /// **Assembly:** `xor <dst>, <src>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::Xor { dst: Register::RAX, src: Register::RCX });
    /// ```
    Xor { dst: Register, src: Register },
    /// Performs a bitwise NOT on a register.
    ///
    /// **Assembly:** `not <reg>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::Not { reg: Register::RAX });
    /// ```
    Not { reg: Register },
    /// Shifts the value in a register to the left by the number of bits in another register.
    ///
    /// **Assembly:** `shl <dst>, <src>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::Shl { dst: Register::RAX, src: Register::RCX });
    /// ```
    Shl { dst: Register, src: Register },
    /// Shifts the value in a register to the right by the number of bits in another register.
    ///
    /// **Assembly:** `shr <dst>, <src>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::Shr { dst: Register::RAX, src: Register::RCX });
    /// ```
    Shr { dst: Register, src: Register },
    /// Pushes a register onto the stack.
    ///
    /// **Assembly:** `push <reg>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::Push { reg: Register::RAX });
    /// ```
    Push { reg: Register },
    /// Pops a register from the stack.
    ///
    /// **Assembly:** `pop <reg>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::Pop { reg: Register::RAX });
    /// ```
    Pop { reg: Register },
    /// Calls a function.
    ///
    /// **Assembly:** `call <function>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::Call("function_name".to_string()));
    /// ```
    Call(String),
    /// Returns from a function.
    ///
    /// **Assembly:** `ret`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::Ret);
    /// ```
    Ret,
    /// Unconditionally jumps to a label.
    ///
    /// **Assembly:** `jmp <label>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::Jmp("label".to_string()));
    /// ```
    Jmp(String),
    /// Defines a label.
    ///
    /// **Assembly:** `<label>:`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::Label("start".to_string()));
    /// ```
    Label(String),
    /// Compares two registers.
    ///
    /// **Assembly:** `cmp <op1>, <op2>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::Cmp { op1: Register::RAX, op2: Register::RBX });
    /// ```
    Cmp { op1: Register, op2: Register },
    /// Jumps to a label if the previous comparison resulted in equality.
    ///
    /// **Assembly:** `je <label>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::Je("equal_label".to_string()));
    /// ```
    Je(String),
    /// Jumps to a label if the previous comparison resulted in inequality.
    ///
    /// **Assembly:** `jne <label>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::Jne("not_equal_label".to_string()));
    /// ```
    Jne(String),
    /// Jumps to a label if the first operand is greater than the second.
    ///
    /// **Assembly:** `jg <label>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::Jg("greater_label".to_string()));
    /// ```
    Jg(String),
    /// Jumps to a label if the first operand is greater than or equal to the second.
    ///
    /// **Assembly:** `jge <label>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::Jge("greater_equal_label".to_string()));
    /// ```
    Jge(String),
    /// Jumps to a label if the first operand is less than the second.
    ///
    /// **Assembly:** `jl <label>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::Jl("less_label".to_string()));
    /// ```
    Jl(String),
    /// Jumps to a label if the first operand is less than or equal to the second.
    ///
    /// **Assembly:** `jle <label>`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::Jle("less_equal_label".to_string()));
    /// ```
    Jle(String),
    /// Triggers a system call.
    ///
    /// **Assembly:** `syscall`
    ///
    /// **Example in Rust:**
    /// ```rust
    /// xasm.emit(Instruction::SYSCALL);
    /// ```
    SYSCALL,
}

impl fmt::Display for Instruction {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Instruction::*;
        match self {
            MovImm { dst, imm } => write!(f, "mov {:?}, {}", dst, imm),
            Mov { dst, src }   => write!(f, "mov {:?}, {:?}", dst, src),
            Add { dst, src }   => write!(f, "add {:?}, {:?}", dst, src),
            Sub { dst, src }   => write!(f, "sub {:?}, {:?}", dst, src),
            Mul { dst, src }   => write!(f, "mul {:?}, {:?}", dst, src),
            Div { dst, src }   => write!(f, "div {:?}, {:?}", dst, src),
            And { dst, src }   => write!(f, "and {:?}, {:?}", dst, src),
            Or { dst, src }    => write!(f, "or {:?}, {:?}", dst, src),
            Xor { dst, src }   => write!(f, "xor {:?}, {:?}", dst, src),
            Not { reg }        => write!(f, "not {:?}", reg),
            Shl { dst, src }   => write!(f, "shl {:?}, {:?}", dst, src),
            Shr { dst, src }   => write!(f, "shr {:?}, {:?}", dst, src),
            Push { reg }       => write!(f, "push {:?}", reg),
            Pop { reg }        => write!(f, "pop {:?}", reg),
            Call(func)         => write!(f, "call {}", func),
            Ret                => write!(f, "ret"),
            Jmp(label)         => write!(f, "jmp {}", label),
            Label(label)       => write!(f, "{}:", label),
            Cmp { op1, op2 }   => write!(f, "cmp {:?}, {:?}", op1, op2),
            Je(label)          => write!(f, "je {}", label),
            Jne(label)         => write!(f, "jne {}", label),
            Jg(label)          => write!(f, "jg {}", label),
            Jge(label)         => write!(f, "jge {}", label),
            Jl(label)          => write!(f, "jl {}", label),
            Jle(label)         => write!(f, "jle {}", label),
            MovVar { reg, var_name } => write!(f, "mov {:?},{}", reg, var_name),
            SYSCALL => write!(f, "syscall"),
        }
    }
}

/// ## Variables
///
/// Represents the various types of variables that can be used within the assembly generation.
#[derive(Debug)]
pub enum Variables {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Bool(bool),
    Str(&'static str),
}

/// ## Xasm
///
/// `Xasm` is the core assembly generator for Linux x86‑64 systems.
/// It manages the list of assembly instructions, register allocation via `RegisterAllocator`,
/// a register stack to track usage, and a list of defined variables.
#[derive(Debug)]
pub struct Xasm {
    pub instructions: Vec<Instruction>,
    pub reg_alloc: RegisterAllocator,
    pub reg_stack: Vec<Register>,
    pub variables: Vec<(&'static str, Variables)>,
}

impl Xasm {
    /// Creates a new `Xasm` instance with empty instruction list, register allocator, and variable list.
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            reg_alloc: RegisterAllocator::new(),
            reg_stack: Vec::new(),
            variables: Vec::new(),
        }
    }

    /// Emits an assembly instruction to the instruction list.
    #[inline(always)]
    pub fn emit(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }

    /// Allocates a register and updates the register stack.
    ///
    /// If `force` is true, forces allocation by freeing a previously used register.
    #[inline(always)]
    pub fn alloc_reg(&mut self, force: bool) -> Register {
        let reg = self.reg_alloc.allocate(force);
        self.reg_stack.push(reg);
        reg
    }

    /// Retrieves a specific register. If it is not available, returns another free register.
    ///
    /// If `force` is true, forces the retrieval by freeing the specific register if it is in use.
    #[inline(always)]
    pub fn get_reg(&mut self, reg: Register, force: bool) -> Register {
        let r = self.reg_alloc.get_specific(reg, force);
        self.reg_stack.push(r);
        r
    }

    /// Frees a register from the register stack and returns it to the allocator.
    #[inline(always)]
    pub fn free_reg(&mut self, reg: Register) {
        if let Some(pos) = self.reg_stack.iter().position(|&r| r == reg) {
            self.reg_stack.remove(pos);
            self.reg_alloc.free(reg);
        }
    }

    /// Returns a reference to the list of emitted instructions.
    #[inline(always)]
    pub fn dump(&self) -> &[Instruction] {
        &self.instructions
    }
    
    /// Adds a variable with a given name to the variable list.
    #[inline(always)]
    pub fn add_variable(&mut self, var: Variables, name: &'static str) {
        self.variables.push((name, var));
    }
}
