use rand::{rng, Rng};

use crate::{
    init::{LinuxX8664, Register, Variables},
    instructions::Instruction,
};
#[derive(Debug)]
/// Tokens used for printing.
pub enum PrintTokens {
    TEXT(&'static str),
    VAR(&'static str),
}

/// The Xstd type wraps a mutable reference to a LinuxX8664 instance and provides extended functionality.
pub struct Xstd<'a> {
    parent: &'a mut LinuxX8664,
}

impl<'a> Xstd<'a> {
    pub fn edit_mut_var(&mut self, var_name: &'static str, value: Variables) {
        // Renamed variable: free_reg -> free_reg_reg
        let free_reg_reg = self.parent.get_reg(Register::r15, false);
        match value {
            Variables::Str(txt) => {
                // For a string, store its length as a placeholder.
                self.parent.emit(Instruction::MovImm { dst: free_reg_reg, imm: txt.len() as i64 });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg_reg, var_name });
                let formatted_str = format!("mov {:?} , \"{}\"", free_reg_reg, txt);
                self.parent.emit(Instruction::AsIs(Box::leak(formatted_str.into_boxed_str())));
            }
            Variables::I8(val) => {
                self.parent.emit(Instruction::MovImm { dst: free_reg_reg, imm: val as i64 });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg_reg, var_name });
            }
            Variables::I16(val) => {
                self.parent.emit(Instruction::MovImm { dst: free_reg_reg, imm: val as i64 });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg_reg, var_name });
            }
            Variables::I32(val) => {
                self.parent.emit(Instruction::MovImm { dst: free_reg_reg, imm: val as i64 });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg_reg, var_name });
            }
            Variables::I64(val) => {
                self.parent.emit(Instruction::MovImm { dst: free_reg_reg, imm: val as i64 });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg_reg, var_name });
            }
            Variables::U8(val) => {
                self.parent.emit(Instruction::MovImm { dst: free_reg_reg, imm: val as i64 });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg_reg, var_name });
            }
            Variables::U16(val) => {
                self.parent.emit(Instruction::MovImm { dst: free_reg_reg, imm: val as i64 });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg_reg, var_name });
            }
            Variables::U32(val) => {
                self.parent.emit(Instruction::MovImm { dst: free_reg_reg, imm: val as i64 });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg_reg, var_name });
            }
            Variables::U64(val) => {
                self.parent.emit(Instruction::MovImm { dst: free_reg_reg, imm: val as i64 });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg_reg, var_name });
            }
            Variables::Bool(val) => {
                self.parent.emit(Instruction::MovImm { dst: free_reg_reg, imm: val as i64 });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg_reg, var_name });
            }
            Variables::F32(val) => {
                self.parent.emit(Instruction::MovF { dst: free_reg_reg, imm: val as f64 });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg_reg, var_name });
            }
            Variables::F64(val) => {
                self.parent.emit(Instruction::MovF { dst: free_reg_reg, imm: val });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg_reg, var_name });
            }
            Variables::AsIs(txt) => {
                // For an "as-is" variable, assume the provided text already represents the value.
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg_reg, var_name: txt });
                self.parent.emit(Instruction::MovIntoVar { reg: free_reg_reg, var_name });
            }
        }
        self.parent.emit(Instruction::RepRsiRdi);
    }
            
    /// Create a new instance of Xstd from an existing mutable LinuxX8664 instance.
    pub fn new(parent: &'a mut LinuxX8664) -> Self {
        Self { parent }
    }
    
    /// Sets up common variables and other things.
    pub fn setup(&mut self) {
        self.parent
            .add_variable(Variables::AsIs("_newline_ : db 0x0a"), "");
        self.parent
            .add_variable(Variables::AsIs("_space_ : db 0x20"), "");
        self.parent.add_mutable_variable(Variables::AsIs("BUFFERADDR : resb 128"), "");
    }
    
    #[allow(unused)]
    /// The xprint method processes print tokens and emits the corresponding instructions using the parent LinuxX8664 instance.
    pub fn xprint(&mut self, tokens: Vec<PrintTokens>) {
        println!("xprint called");
        println!("print toks {:?}", tokens);
        let mut print_tok_counter = rng().random_range(0..10000);
        for (index, token) in tokens.iter().enumerate() {
            match token {
                PrintTokens::TEXT(text) => {
                    // Collect characters until a space or newline.
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

                                    // Generate a unique label for the text variable.
                                    let label_str = format!("print_label_{}_{}", print_tok_counter, index);
                                    let label: &'static str = Box::leak(label_str.into_boxed_str());

                                    // Add the text as a variable and emit instructions to move it into a register.
                                    let to_p_word: &'static str = Box::leak(Box::new(word.clone()));
                                    word.clear();
                                    self.parent.add_variable(Variables::Str(to_p_word), label);
                                    self.parent.emit(Instruction::MovIntoVar {
                                        reg: rsi_reg,
                                        var_name: label,
                                    });
                                    self.parent.emit(Instruction::MovImm {
                                        dst: rdx_reg,
                                        imm: to_p_word.len() as i64,
                                    });
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
                                self.parent.emit(Instruction::MovIntoVar {
                                    reg: rsi_reg,
                                    var_name: "_space_",
                                });
                                self.parent.emit(Instruction::MovImm {
                                    dst: rdx_reg,
                                    imm: 1,
                                });
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

                                    // Generate a unique label for the text variable.
                                    let label_str = format!("print_label_{}_{}", print_tok_counter, index);
                                    let label: &'static str = Box::leak(label_str.into_boxed_str());

                                    // Add the text as a variable and emit instructions to move it into a register.
                                    let to_p_word: &'static str = Box::leak(Box::new(word.clone()));
                                    word.clear();
                                    self.parent.add_variable(Variables::Str(to_p_word), label);
                                    self.parent.emit(Instruction::MovIntoVar {
                                        reg: rsi_reg,
                                        var_name: label,
                                    });
                                    self.parent.emit(Instruction::MovImm {
                                        dst: rdx_reg,
                                        imm: to_p_word.len() as i64,
                                    });
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
                                    self.parent.emit(Instruction::MovIntoVar {
                                        reg: rsi_reg,
                                        var_name: "_newline_",
                                    });
                                    self.parent.emit(Instruction::MovImm {
                                        dst: rdx_reg,
                                        imm: 1,
                                    });
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
                    self.parent.emit(Instruction::MovImm { dst: rax_reg, imm: 1 });
                    self.parent.emit(Instruction::MovImm { dst: rdi_reg, imm: 1 });
                    self.parent.emit(Instruction::MovIntoVar {
                        reg: rsi_reg,
                        var_name: var,
                    });
                    self.parent.emit(Instruction::MovImm {
                        dst: rdx_reg,
                        imm: var.len() as i64,
                    });
                    self.parent.emit(Instruction::SYSCALL);
                    self.parent.free_reg(rax_reg);
                    self.parent.free_reg(rdi_reg);
                    self.parent.free_reg(rsi_reg);
                    self.parent.free_reg(rdx_reg);
                }
            }
        }
    }
    
    /// Exits the program with the specified exit status code.
    ///
    /// This function emits the necessary instructions to exit the program with the given exit status code.
    /// It sets the system call number to 60 (the exit syscall) in the `RAX` register, the exit status code
    /// in the `RDI` register, and then invokes the system call.
    pub fn xexit(&mut self, code: i64) {
        self.parent.emit(Instruction::MovImm {
            dst: Register::rax,
            imm: 60,
        });
        self.parent.emit(Instruction::MovImm {
            dst: Register::rdi,
            imm: code,
        });
        self.parent.emit(Instruction::SYSCALL);
    }
}
