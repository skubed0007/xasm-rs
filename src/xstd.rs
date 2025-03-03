use rand::{rng, Rng};
use crate::{
    init::{LinuxX8664, Register, Variables},
    instructions::Instruction,
};

#[derive(Debug)]
pub enum PrintTokens {
    TEXT(&'static str),
    VAR(&'static str),
}

#[derive(Debug)]
pub struct Xstd<'a> {
    parent: &'a mut LinuxX8664,
}

impl<'a> Xstd<'a> {
    pub fn edit_mut_var(&mut self, var_name: &'static str, value: Variables) {
        let tempname: &'static str = Box::leak(format!("temp_{}", rand::rng().random::<u32>()).into_boxed_str());
        self.parent.add_variable(value, tempname);
        let free_reg = self.parent.get_reg(Register::rcx, true);
        self.parent.emit(Instruction::MovIntoVar { var_name: &tempname, reg: Register::rsi });
        self.parent.emit(Instruction::MovIntoVar { var_name: var_name, reg: Register::rdi });
        match value {
            Variables::Str(ref txt) => {
                self.parent.emit(Instruction::MovImm { dst: free_reg, imm: txt.len() as i64 });
            }
            Variables::I8(val) => {
                self.parent.emit(Instruction::MovImm { dst: free_reg, imm: val as i64 });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg, var_name });
            }
            Variables::I16(val) => {
                self.parent.emit(Instruction::MovImm { dst: free_reg, imm: val as i64 });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg, var_name });
            }
            Variables::I32(val) => {
                self.parent.emit(Instruction::MovImm { dst: free_reg, imm: val as i64 });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg, var_name });
            }
            Variables::I64(val) => {
                self.parent.emit(Instruction::MovImm { dst: free_reg, imm: val as i64 });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg, var_name });
            }
            Variables::U8(val) => {
                self.parent.emit(Instruction::MovImm { dst: free_reg, imm: val as i64 });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg, var_name });
            }
            Variables::U16(val) => {
                self.parent.emit(Instruction::MovImm { dst: free_reg, imm: val as i64 });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg, var_name });
            }
            Variables::U32(val) => {
                self.parent.emit(Instruction::MovImm { dst: free_reg, imm: val as i64 });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg, var_name });
            }
            Variables::U64(val) => {
                self.parent.emit(Instruction::MovImm { dst: free_reg, imm: val as i64 });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg, var_name });
            }
            Variables::Bool(val) => {
                self.parent.emit(Instruction::MovImm { dst: free_reg, imm: val as i64 });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg, var_name });
            }
            Variables::F32(val) => {
                self.parent.emit(Instruction::MovF { dst: free_reg, imm: val as f64 });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg, var_name });
            }
            Variables::F64(val) => {
                self.parent.emit(Instruction::MovF { dst: free_reg, imm: val });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg, var_name });
            }
            Variables::AsIs(ref txt) => {
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg, var_name: txt });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg, var_name });
            }
        }
        self.parent.emit(Instruction::RepRsiRdi);
        self.parent.free_reg(free_reg);
    }

    pub fn new(parent: &'a mut LinuxX8664) -> Self {
        Self { parent }
    }

    pub fn setup(&mut self) {
        self.parent.add_variable(Variables::AsIs("_newline_ : db 0x0a"), "");
        self.parent.add_variable(Variables::AsIs("_space_ : db 0x20"), "");
        //self.parent.add_mutable_variable(Variables::AsIs("BUFFERADDR : resb 128"), "");
        //self.parent.emit(Instruction::AsIs("find_length:\ncmp byte [rsi + rcx], 0\nje length_found\ninc rcx\njmp find_length\nlength_found:\n"));
    }

    #[allow(unused)]
    pub fn xprint(&mut self, tokens: Vec<PrintTokens>) {
        let mut print_tok_counter = rng().random_range(0..10000);
        for (index, token) in tokens.iter().enumerate() {
            match token {
                PrintTokens::TEXT(text) => {
                    let mut word = String::new();
                    let mut escapemode = false;
                    for char in text.chars() {
                        print_tok_counter = rng().random_range(0..10000);
                        match char {
                            '\\' if !escapemode => escapemode = !escapemode,
                            ' ' => {
                                if !word.is_empty() {
                                    let rax_reg = self.parent.get_reg(Register::rax, true);
                                    let rdi_reg = self.parent.get_reg(Register::rdi, true);
                                    let rsi_reg = self.parent.get_reg(Register::rsi, true);
                                    let rdx_reg = self.parent.get_reg(Register::rdx, true);
                                    self.parent.emit(Instruction::MovImm { dst: rax_reg, imm: 1 });
                                    self.parent.emit(Instruction::MovImm { dst: rdi_reg, imm: 1 });
                                    let label_str = format!("print_label_{}_{}", print_tok_counter, index);
                                    let label: &'static str = Box::leak(label_str.into_boxed_str());
                                    let to_p_word: &'static str = Box::leak(Box::new(word.clone()));
                                    word.clear();
                                    self.parent.add_variable(Variables::Str(to_p_word), label);
                                    self.parent.emit(Instruction::MovIntoVar { reg: rsi_reg, var_name: label });
                                    self.parent.emit(Instruction::MovImm { dst: rdx_reg, imm: to_p_word.len() as i64 });
                                    self.parent.emit(Instruction::SYSCALL);
                                    print_tok_counter = rng().random_range(0..10000);
                                    self.parent.free_reg(rax_reg);
                                    self.parent.free_reg(rdi_reg);
                                    self.parent.free_reg(rsi_reg);
                                    self.parent.free_reg(rdx_reg);
                                }
                                let rax_reg = self.parent.get_reg(Register::rax, true);
                                let rdi_reg = self.parent.get_reg(Register::rdi, true);
                                let rsi_reg = self.parent.get_reg(Register::rsi, true);
                                let rdx_reg = self.parent.get_reg(Register::rdx, true);
                                self.parent.emit(Instruction::MovImm { dst: rax_reg, imm: 1 });
                                self.parent.emit(Instruction::MovImm { dst: rdi_reg, imm: 1 });
                                self.parent.emit(Instruction::MovIntoVar { reg: rsi_reg, var_name: "_space_" });
                                self.parent.emit(Instruction::MovImm { dst: rdx_reg, imm: 1 });
                                self.parent.emit(Instruction::SYSCALL);
                                print_tok_counter = rng().random_range(0..10000);
                                self.parent.free_reg(rax_reg);
                                self.parent.free_reg(rdi_reg);
                                self.parent.free_reg(rsi_reg);
                                self.parent.free_reg(rdx_reg);
                            }
                            _ if escapemode => {
                                if !word.is_empty() {
                                    let rax_reg = self.parent.get_reg(Register::rax, true);
                                    let rdi_reg = self.parent.get_reg(Register::rdi, true);
                                    let rsi_reg = self.parent.get_reg(Register::rsi, true);
                                    let rdx_reg = self.parent.get_reg(Register::rdx, true);
                                    self.parent.emit(Instruction::MovImm { dst: rax_reg, imm: 1 });
                                    self.parent.emit(Instruction::MovImm { dst: rdi_reg, imm: 1 });
                                    let label_str = format!("print_label_{}_{}", print_tok_counter, index);
                                    let label: &'static str = Box::leak(label_str.into_boxed_str());
                                    let to_p_word: &'static str = Box::leak(Box::new(word.clone()));
                                    word.clear();
                                    self.parent.add_variable(Variables::Str(to_p_word), label);
                                    self.parent.emit(Instruction::MovIntoVar { reg: rsi_reg, var_name: label });
                                    self.parent.emit(Instruction::MovImm { dst: rdx_reg, imm: to_p_word.len() as i64 });
                                    self.parent.emit(Instruction::SYSCALL);
                                    print_tok_counter = rng().random_range(0..10000);
                                    self.parent.free_reg(rax_reg);
                                    self.parent.free_reg(rdi_reg);
                                    self.parent.free_reg(rsi_reg);
                                    self.parent.free_reg(rdx_reg);
                                }
                                if char == 'n' {
                                    let rax_reg = self.parent.get_reg(Register::rax, true);
                                    let rdi_reg = self.parent.get_reg(Register::rdi, true);
                                    let rsi_reg = self.parent.get_reg(Register::rsi, true);
                                    let rdx_reg = self.parent.get_reg(Register::rdx, true);
                                    self.parent.emit(Instruction::MovImm { dst: rax_reg, imm: 1 });
                                    self.parent.emit(Instruction::MovImm { dst: rdi_reg, imm: 1 });
                                    self.parent.emit(Instruction::MovIntoVar { reg: rsi_reg, var_name: "_newline_" });
                                    self.parent.emit(Instruction::MovImm { dst: rdx_reg, imm: 1 });
                                    self.parent.emit(Instruction::SYSCALL);
                                    print_tok_counter = rng().random_range(0..10000);
                                    self.parent.free_reg(rax_reg);
                                    self.parent.free_reg(rdi_reg);
                                    self.parent.free_reg(rsi_reg);
                                    self.parent.free_reg(rdx_reg);
                                }
                                escapemode = false;
                            }
                            _ => {
                                word.push(char);
                                print_tok_counter = rng().random_range(0..10000);
                            }
                        }
                    }
                }
                PrintTokens::VAR(var) => {
                    let rax_reg = self.parent.get_reg(Register::rax, true);
                    let rdi_reg = self.parent.get_reg(Register::rdi, true);
                    let rsi_reg = self.parent.get_reg(Register::rsi, true);
                    let rdx_reg = self.parent.get_reg(Register::rdx, true);
                    let rcx_reg = self.parent.get_reg(Register::rcx, true);
                    self.parent.emit(Instruction::MovImm { dst: rax_reg, imm: 1 });
                    self.parent.emit(Instruction::MovImm { dst: rdi_reg, imm: 1 });
                    self.parent.emit(Instruction::MovIntoVar { reg: rsi_reg, var_name: var });
                    self.parent.emit(Instruction::Mov { dst: rcx_reg, src: rcx_reg });
                    self.parent.emit(Instruction::Xor { dst: rcx_reg, src: rcx_reg });
                    let label = format!("find_length_{}", rand::rng().random::<u32>());
                    let asm_str = format!("{}:\n        cmp byte [rsi + rcx], 0\n        je {}_found\n        inc rcx\n        jmp {}\n        {}_found:\n", label, label, label, label);
                    let asm_static: &'static str = Box::leak(asm_str.into_boxed_str());
                    self.parent.emit(Instruction::AsIs(asm_static));
                    self.parent.emit(Instruction::Mov { dst: rdx_reg, src: rcx_reg });
                    self.parent.emit(Instruction::SYSCALL);
                    self.parent.free_reg(rax_reg);
                    self.parent.free_reg(rdi_reg);
                    self.parent.free_reg(rsi_reg);
                    self.parent.free_reg(rdx_reg);
                    self.parent.free_reg(rcx_reg);
                }
                
            }
        }
    }

    pub fn xexit(&mut self, code: i64) {
        self.parent.emit(Instruction::MovImm { dst: Register::rax, imm: 60 });
        self.parent.emit(Instruction::MovImm { dst: Register::rdi, imm: code });
        self.parent.emit(Instruction::SYSCALL);
    }
}
