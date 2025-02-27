pub mod init;

#[cfg(test)]
mod tests {
    use crate::init::{self, Instruction, Variables, Xasm};
    #[test]
   fn test_xasm(){
     let mut xasm = Xasm::new();
     let rax = xasm.get_reg(init::Register::RAX, true);
     let rdi = xasm.get_reg(init::Register::RDI, true);
     let rsi = xasm.get_reg(init::Register::RSI, true);
     let rdx = xasm.get_reg(init::Register::RDX, true);
 
     xasm.emit(Instruction::MovImm {
         dst: rax,
         imm: 1,
     });
     xasm.emit(Instruction::MovImm {
         dst: rdi,
         imm: 1,
     });
     xasm.add_variable(Variables::Str("hello world"), "v1");
     xasm.emit(Instruction::MovVar {
         reg: rsi,
         var_name: "v1",
     });
     xasm.emit(Instruction::MovImm {
         dst: rdx,
         imm: 12,
     });
     xasm.emit(Instruction::SYSCALL);
     for inst in xasm.dump(){
         println!("{}",inst);
     }
     println!("xasm:\n{:#?}", xasm);

   }
}
