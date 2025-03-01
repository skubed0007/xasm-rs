use rand::{rng, Rng};

use crate::{
    init::{LinuxX8664, Register, Variables},
    instructions::Instruction,
};

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
    /// Create a new instance of Xstd from an existing mutable LinuxX8664 instance.
    pub fn new(parent: &'a mut LinuxX8664) -> Self {
        Self { parent }
    }
    ///sets up common variables and other things
    pub fn setup(&mut self) {
        self.parent
            .add_variable(Variables::AsIs("_newline_ : db 0x0a"), "");
        self.parent
            .add_variable(Variables::AsIs("_space_ : db 0x20"), "");
    }
    #[allow(unused)]
    /// The xprint method processes print tokens and emits the corresponding instructions using the parent LinuxX8664 instance.
    pub fn xprint(&mut self, tokens: Vec<PrintTokens>) {
        let mut print_tok_counter = rng().random_range(0..10000);
        for (index, token) in tokens.iter().enumerate() {
            match token {
                PrintTokens::TEXT(text) => {
                    // Collect chars until a space or new line char
                    let mut word = String::new();
                    let mut escapemode = false;
                    for char in text.chars() {
                        print_tok_counter = rng().random_range(0..10000);
                        match char {
                            '\\' if !escapemode => escapemode = !escapemode,
                            ' ' => {
                                if !word.is_empty() {
                                    let rax = self.parent.get_reg(Register::RAX, true);
                                    let rdi = self.parent.get_reg(Register::RDI, true);
                                    let rsi = self.parent.get_reg(Register::RSI, true);
                                    let rdx = self.parent.get_reg(Register::RDX, true);
                                    self.parent.emit(Instruction::MovImm { dst: rax, imm: 1 });
                                    self.parent.emit(Instruction::MovImm { dst: rdi, imm: 1 });

                                    // Generate a unique label for the text variable.
                                    let label_str =
                                        format!("print_label_{}_{}", print_tok_counter, index);
                                    let label: &'static str = Box::leak(label_str.into_boxed_str());

                                    // Add the text as a variable and emit instructions to move it into a register.
                                    let to_p_word: &'static str = Box::leak(Box::new(word.clone()));
                                    word.clear();
                                    self.parent.add_variable(Variables::Str(to_p_word), label);
                                    self.parent.emit(Instruction::MovVar {
                                        reg: rsi,
                                        var_name: label,
                                    });
                                    self.parent.emit(Instruction::MovImm {
                                        dst: rdx,
                                        imm: to_p_word.len() as i64,
                                    });
                                    self.parent.emit(Instruction::SYSCALL);
                                    print_tok_counter = rng().random_range(0..10000);
                                }
                                let rax = self.parent.get_reg(Register::RAX, true);
                                let rdi = self.parent.get_reg(Register::RDI, true);
                                let rsi = self.parent.get_reg(Register::RSI, true);
                                let rdx = self.parent.get_reg(Register::RDX, true);
                                self.parent.emit(Instruction::MovImm { dst: rax, imm: 1 });
                                self.parent.emit(Instruction::MovImm { dst: rdi, imm: 1 });
                                self.parent.emit(Instruction::MovVar {
                                    reg: rsi,
                                    var_name: "_space_",
                                });
                                self.parent.emit(Instruction::MovImm {
                                    dst: rdx,
                                    imm: 1,
                                });
                                self.parent.emit(Instruction::SYSCALL);
                                print_tok_counter = rng().random_range(0..10000);
                            }
                            _ if escapemode => {
                                if !word.is_empty() {
                                    let rax = self.parent.get_reg(Register::RAX, true);
                                    let rdi = self.parent.get_reg(Register::RDI, true);
                                    let rsi = self.parent.get_reg(Register::RSI, true);
                                    let rdx = self.parent.get_reg(Register::RDX, true);
                                    self.parent.emit(Instruction::MovImm { dst: rax, imm: 1 });
                                    self.parent.emit(Instruction::MovImm { dst: rdi, imm: 1 });

                                    // Generate a unique label for the text variable.
                                    let label_str =
                                        format!("print_label_{}_{}", print_tok_counter, index);
                                    let label: &'static str = Box::leak(label_str.into_boxed_str());

                                    // Add the text as a variable and emit instructions to move it into a register.
                                    let to_p_word: &'static str = Box::leak(Box::new(word.clone()));
                                    word.clear();
                                    self.parent.add_variable(Variables::Str(to_p_word), label);
                                    self.parent.emit(Instruction::MovVar {
                                        reg: rsi,
                                        var_name: label,
                                    });
                                    self.parent.emit(Instruction::MovImm {
                                        dst: rdx,
                                        imm: to_p_word.len() as i64,
                                    });
                                    self.parent.emit(Instruction::SYSCALL);
                                    print_tok_counter = rng().random_range(0..10000);
                                }
                                if char == 'n' {
                                    let rax = self.parent.get_reg(Register::RAX, true);
                                    let rdi = self.parent.get_reg(Register::RDI, true);
                                    let rsi = self.parent.get_reg(Register::RSI, true);
                                    let rdx = self.parent.get_reg(Register::RDX, true);
                                    self.parent.emit(Instruction::MovImm { dst: rax, imm: 1 });
                                    self.parent.emit(Instruction::MovImm { dst: rdi, imm: 1 });
                                    self.parent.emit(Instruction::MovVar {
                                        reg: rsi,
                                        var_name: "_newline_",
                                    });
                                    self.parent.emit(Instruction::MovImm {
                                        dst: rdx,
                                        imm: 1 as i64,
                                    });
                                    self.parent.emit(Instruction::SYSCALL);
                                    print_tok_counter = rng().random_range(0..10000);
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
                    // For variable tokens, simply print the value.
                    print!("{}", var);
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
        self.parent.emit(Instruction::MovImm { dst: Register::RAX, imm: 60 }); // syscall number for exit
        self.parent.emit(Instruction::MovImm { dst: Register::RDI, imm: code }); // exit status code
        self.parent.emit(Instruction::SYSCALL); // invoke the syscall
    }
    
}
