# Xasm : assembly code generator for rust
> NOTE: Xasm only support linux 64/32 bit though we do have plans for windows and mac in future
---
# Installation
add the crate to your ``Cargo.toml`` file by running the following command:
```
cargo add xasm-rs
```
---
# Using Xasm
first create a mutable xasm class in folllowing way:
```rust
let mut xasm = init::Xasm::new();
```
and now start adding tokens,vars,functions to xasm
here is a sample stuff you can do
```rust
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
```
---
Check out the full documentation for xasm to know more on how to use it!

---

A big bear hug to ([neva](https://github.com/nevakrien)) for being an absolute star and guiding light in the journey of creating xasm!
