use std::collections::VecDeque;
use std::fmt;
use crate::instructions::Instruction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    RAX, RBX, RCX, RDX, RSI, RDI, RBP, RSP,
    R8, R9, R10, R11, R12, R13, R14, R15,
}

#[derive(Debug)]
pub struct RegisterAllocator {
    free_regs: VecDeque<Register>,
    used_regs: Vec<Register>,
}

impl RegisterAllocator {
    pub fn new() -> Self {
        let free_regs = VecDeque::from(vec![
            Register::RAX, Register::RBX, Register::RCX, Register::RDX,
            Register::RSI, Register::RDI, Register::RBP, Register::RSP,
            Register::R8,  Register::R9,  Register::R10, Register::R11,
            Register::R12, Register::R13, Register::R14, Register::R15,
        ]);
        Self { free_regs, used_regs: Vec::new() }
    }

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

    pub fn free(&mut self, reg: Register) {
        if let Some(pos) = self.used_regs.iter().position(|&r| r == reg) {
            self.used_regs.remove(pos);
            self.free_regs.push_back(reg);
        }
    }

    pub fn reset(&mut self) {
        while let Some(reg) = self.used_regs.pop() {
            self.free_regs.push_back(reg);
        }
    }

    pub fn used(&self) -> &[Register] {
        &self.used_regs
    }

    pub fn free_list(&self) -> Vec<Register> {
        self.free_regs.iter().copied().collect()
    }
}

impl fmt::Display for Instruction {
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
    ///as is value , no changes take place , plain str writter directly no converion
    /// write entire line of assembly , no need to provide a name for variable when using ``.add_variable``
    AsIs(&'static str),
}

#[derive(Debug)]
struct XasmCore {
    instructions: Vec<Instruction>,
    reg_alloc: RegisterAllocator,
    reg_stack: Vec<Register>,
    variables: Vec<(&'static str, Variables)>,
}

impl XasmCore {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
            reg_alloc: RegisterAllocator::new(),
            reg_stack: Vec::new(),
            variables: Vec::new(),
        }
    }

    fn emit(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }

    fn alloc_reg(&mut self, force: bool) -> Register {
        let reg = self.reg_alloc.allocate(force);
        self.reg_stack.push(reg);
        reg
    }

    fn get_reg(&mut self, reg: Register, force: bool) -> Register {
        let r = self.reg_alloc.get_specific(reg, force);
        self.reg_stack.push(r);
        r
    }

    fn free_reg(&mut self, reg: Register) {
        if let Some(pos) = self.reg_stack.iter().position(|&r| r == reg) {
            self.reg_stack.remove(pos);
            self.reg_alloc.free(reg);
        }
    }

    fn dump(&self) -> (&[Instruction], &[(&'static str, Variables)]) {
        (&self.instructions, &self.variables)
    }

    fn add_variable(&mut self, var: Variables, name: &'static str) {
        self.variables.push((name, var));
    }
}

// Make the generic Xasm type internal by not marking it as public.
struct Xasm {
    core: XasmCore,
}

impl Xasm {
    fn new() -> Self {
        Self { core: XasmCore::new() }
    }

    fn emit(&mut self, instr: Instruction) {
        self.core.emit(instr)
    }

    fn alloc_reg(&mut self, force: bool) -> Register {
        self.core.alloc_reg(force)
    }

    fn get_reg(&mut self, reg: Register, force: bool) -> Register {
        self.core.get_reg(reg, force)
    }

    fn free_reg(&mut self, reg: Register) {
        self.core.free_reg(reg)
    }

    fn dump(&self) -> (&[Instruction], &[(&'static str, Variables)]) {
        self.core.dump()
    }

    fn add_variable(&mut self, var: Variables, name: &'static str) {
        self.core.add_variable(var, name)
    }
}

// Expose only the architecture-specific interface publicly.
pub struct LinuxX8664 {
    parent: Xasm,
}

impl LinuxX8664 {
    pub fn new() -> Self {
        Self { parent: Xasm::new() }
    }

    pub fn emit(&mut self, instr: Instruction) {
        self.parent.emit(instr);
    }

    pub fn alloc_reg(&mut self, force: bool) -> Register {
        self.parent.alloc_reg(force)
    }

    pub fn get_reg(&mut self, reg: Register, force: bool) -> Register {
        self.parent.get_reg(reg, force)
    }

    pub fn free_reg(&mut self, reg: Register) {
        self.parent.free_reg(reg)
    }

    pub fn dump(&self) -> (&[Instruction], &[(&'static str, Variables)]) {
        self.parent.dump()
    }

    pub fn add_variable(&mut self, var: Variables, name: &'static str) {
        self.parent.add_variable(var, name)
    }
}
