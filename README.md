# Xasm : assembly code generator for rust
> NOTE: Xasm only support nasm x86_64 on linux
This crate is used to generate and compile nasm assembly at runtime
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
//add a string type variable called "mystring"
xasm.vars.push(Vars::String("mystring".to_string(),"Hello world!".to_string())); 
// print the varible to STDOUT using print token
xasm.tokens.push(init::Tokens::print(init::FileDescriptor::STDOUT, "%mystring \\n".chars().collect()));
```
---
Check out the full documentation for xasm to know more on how to use it!
---
# License
**Xasm** is licensed under the **Joay-License** which in short says that:

***The Joay License allows non-commercial use, modification, and distribution of the software with proper attribution. Commercial use and converting the software to closed-source are prohibited***
