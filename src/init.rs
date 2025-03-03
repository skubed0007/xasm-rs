use rand::Rng;

use crate::instructions::Instruction;
use std::collections::VecDeque;
use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    rax,
    rbx,
    rcx,
    rdx,
    rsi,
    rdi,
    rbp,
    rsp,
    r8,
    r9,
    r10,
    r11,
    r12,
    r13,
    r14,
    r15,
}

#[derive(Debug)]
pub struct RegisterAllocator {
    free_regs: VecDeque<Register>,
    used_regs: Vec<Register>,
}

impl RegisterAllocator {
    pub fn new() -> Self {
        let free_regs = VecDeque::from(vec![
            Register::rax,
            Register::rbx,
            Register::rcx,
            Register::rdx,
            Register::rsi,
            Register::rdi,
            Register::rbp,
            Register::rsp,
            Register::r8,
            Register::r9,
            Register::r10,
            Register::r11,
            Register::r12,
            Register::r13,
            Register::r14,
            Register::r15,
        ]);
        Self {
            free_regs,
            used_regs: Vec::new(),
        }
    }

    pub fn allocate(&mut self, force: bool) -> Register {
        if let Some(reg) = self.free_regs.pop_front() {
            self.used_regs.push(reg);
            reg
        } else if force && !self.used_regs.is_empty() {
            let reg = self.used_regs.pop().unwrap();
            self.free_regs.push_back(reg);
            let new_reg = self
                .free_regs
                .pop_front()
                .expect("No register available after forcing");
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

#[allow(non_camel_case_types,non_snake_case,unused)]
impl fmt::Display for Instruction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use Instruction::*;
            match self {
                MovF { dst, imm } => write!(f, "mov {:?}, {}", dst, imm),
                MovIntoVar { reg, var_name } => write!(f, "mov {:?}, {}", reg, var_name),
                MovFromVar { var_name, reg } => write!(f, "mov [{}], {:?}", var_name, reg),
                MovImm { dst, imm } => write!(f, "mov {:?}, {}", dst, imm),
                Mov { dst, src } => write!(f, "mov {:?}, {:?}", dst, src),
                Add { dst, src } => write!(f, "add {:?}, {:?}", dst, src),
                Sub { dst, src } => write!(f, "sub {:?}, {:?}", dst, src),
                Mul { dst, src } => write!(f, "mul {:?}, {:?}", dst, src),
                Div { src } => write!(f, "div {:?}", src),
                And { dst, src } => write!(f, "and {:?}, {:?}", dst, src),
                Or { dst, src } => write!(f, "or {:?}, {:?}", dst, src),
                Xor { dst, src } => write!(f, "xor {:?}, {:?}", dst, src),
                Not { reg } => write!(f, "not {:?}", reg),
                Shl { dst, src } => write!(f, "shl {:?}, {:?}", dst, src),
                Shr { dst, src } => write!(f, "shr {:?}, {:?}", dst, src),
                Push { reg } => write!(f, "push {:?}", reg),
                Pop { reg } => write!(f, "pop {:?}", reg),
                Call(func) => write!(f, "call {}", func),
                Ret => write!(f, "ret"),
                Jmp(label) => write!(f, "jmp {}", label),
                Label(label) => write!(f, "{}:", label),
                Cmp { op1, op2 } => write!(f, "cmp {:?}, {:?}", op1, op2),
                Je(label) => write!(f, "je {}", label),
                Jne(label) => write!(f, "jne {}", label),
                Jg(label) => write!(f, "jg {}", label),
                Jge(label) => write!(f, "jge {}", label),
                Jl(label) => write!(f, "jl {}", label),
                Jle(label) => write!(f, "jle {}", label),
                MovToMem { src, addr } => write!(f, "mov [{:?}], {:?}", addr, src),
                MovFromMem { addr, dst } => write!(f, "mov {:?}, [{:?}]", dst, addr),
                AddImm { dst, imm } => write!(f, "add {:?}, {}", dst, imm),
                AsIs(s) => write!(f, "{}", s),
                SYSCALL => write!(f, "syscall"),
                LeaIntoVar {reg,var_name} => write!(f, "lea {:?}, [{}]", reg, var_name),
                Rep_RSI_RDI => write!(f, "rep movsb"),
            }
        }
    }

#[derive(Debug, PartialEq, Clone, Copy)]
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
    mutable_variables: Vec<(&'static str, Variables)>,
    funcs : Vec<Funcs>,
}

#[derive(Debug)]
pub struct Funcs{
    pub name : &'static str,
    pub args : Vec<Register>,
    pub body : Vec<Instruction>,
}
impl Funcs{
    pub fn new(name : &'static str, args : Vec<Register>, body : Vec<Instruction>) -> Self{
        Self {
            name,
            args,
            body,
        }
    }
}

impl XasmCore {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
            reg_alloc: RegisterAllocator::new(),
            reg_stack: Vec::new(),
            variables: Vec::new(),
            mutable_variables: Vec::new(),
            funcs : Vec::new(),
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

    fn dump(
        &self,
    ) -> (
        &[Instruction],
        &[(&'static str, Variables)],
        &[(&'static str, Variables)],
        &[Funcs],
    ) {
        (&self.instructions, &self.variables, &self.mutable_variables, &self.funcs)
    }

    fn add_variable(&mut self, var: Variables, name: &'static str) {
        self.variables.push((name, var));
    }
}
#[derive(Debug)]

// Make the generic Xasm type internal by not marking it as public.
struct Xasm {
    core: XasmCore,
}

impl Xasm {
    fn new() -> Self {
        Self {
            core: XasmCore::new(),
        }
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

    fn dump(
        &self,
    ) -> (
        &[Instruction],
        &[(&'static str, Variables)],
        &[(&'static str, Variables)],
        &[Funcs],
    ) {
        self.core.dump()
    }

    fn add_variable(&mut self, var: Variables, name: &'static str) {
        self.core.add_variable(var, name)
    }
    fn add_mutable_variable(&mut self, var: Variables, var_name: &'static str) {
        self.core.mutable_variables.push((var_name, var));
        let tempname: &'static str = Box::leak(format!("temp_{}", rand::rng().random::<u32>()).into_boxed_str());
        self.add_variable(var, tempname);
        let free_reg = self.get_reg(Register::rcx, true);
        self.emit(Instruction::MovIntoVar { var_name: &tempname, reg: Register::rsi });
        self.emit(Instruction::MovIntoVar { var_name: var_name, reg: Register::rdi });
        match var {
            Variables::Str(ref txt) => {
                self.emit(Instruction::MovImm { dst: free_reg, imm: txt.len() as i64 });
            }
            Variables::I8(val) => {
                self.emit(Instruction::MovImm { dst: free_reg, imm: val as i64 });
                self.emit(Instruction::MovIntoVar { reg: free_reg, var_name });
            }
            Variables::I16(val) => {
                self.emit(Instruction::MovImm { dst: free_reg, imm: val as i64 });
                self.emit(Instruction::MovIntoVar { reg: free_reg, var_name });
            }
            Variables::I32(val) => {
                self.emit(Instruction::MovImm { dst: free_reg, imm: val as i64 });
                self.emit(Instruction::MovIntoVar { reg: free_reg, var_name });
            }
            Variables::I64(val) => {
                self.emit(Instruction::MovImm { dst: free_reg, imm: val as i64 });
                self.emit(Instruction::MovIntoVar { reg: free_reg, var_name });
            }
            Variables::U8(val) => {
                self.emit(Instruction::MovImm { dst: free_reg, imm: val as i64 });
                self.emit(Instruction::MovIntoVar { reg: free_reg, var_name });
            }
            Variables::U16(val) => {
                self.emit(Instruction::MovImm { dst: free_reg, imm: val as i64 });
                self.emit(Instruction::MovIntoVar { reg: free_reg, var_name });
            }
            Variables::U32(val) => {
                self.emit(Instruction::MovImm { dst: free_reg, imm: val as i64 });
                self.emit(Instruction::MovIntoVar { reg: free_reg, var_name });
            }
            Variables::U64(val) => {
                self.emit(Instruction::MovImm { dst: free_reg, imm: val as i64 });
                self.emit(Instruction::MovIntoVar { reg: free_reg, var_name });
            }
            Variables::Bool(val) => {
                self.emit(Instruction::MovImm { dst: free_reg, imm: val as i64 });
                self.emit(Instruction::MovIntoVar { reg: free_reg, var_name });
            }
            Variables::F32(val) => {
                self.emit(Instruction::MovF { dst: free_reg, imm: val as f64 });
                self.emit(Instruction::MovIntoVar { reg: free_reg, var_name });
            }
            Variables::F64(val) => {
                self.emit(Instruction::MovF { dst: free_reg, imm: val });
                self.emit(Instruction::MovIntoVar { reg: free_reg, var_name });
            }
            Variables::AsIs(ref txt) => {
                self.emit(Instruction::MovIntoVar { reg: free_reg, var_name: txt });
                self.emit(Instruction::MovIntoVar { reg: free_reg, var_name });
            }
        }
        self.emit(Instruction::RepRsiRdi);
        self.free_reg(free_reg);
    }
    
    
    fn add_func(&mut self, func : Funcs) {
        self.core.funcs.push(func);
    }
    fn direct_add_mut_var(&mut self, var: Variables, var_name: &'static str) {
        self.core.mutable_variables.push((var_name, var));
    }
}

#[derive(Debug)]
// Expose only the architecture-specific interface publicly.
pub struct LinuxX8664 {
    parent: Xasm,
}

impl LinuxX8664 {
    pub fn new() -> Self {
        Self {
            parent: Xasm::new(),
        }
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

    pub fn dump(
        &self,
    ) -> (
        &[Instruction],
        &[(&'static str, Variables)],
        &[(&'static str, Variables)],
        &[Funcs],
    ) {
        self.parent.dump()
    }
    pub fn direct_add_mut_var(&mut self, var: Variables, name: &'static str) {
        self.parent.direct_add_mut_var(var, name);
    }

    pub fn add_variable(&mut self, var: Variables, name: &'static str) {
        self.parent.add_variable(var, name)
    }
    pub fn add_mutable_variable(&mut self, var: Variables, name: &'static str) {
        self.parent.add_mutable_variable(var, name)
    }
    pub fn add_func(&mut self, func: Funcs) {
        self.parent.add_func(func);
    }
}
