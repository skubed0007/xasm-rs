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
//add a string type variable called "mystring"
xasm.vars.push(Vars::String("mystring".to_string(),"Hello world!".to_string())); 
// print the varible to STDOUT using print token
xasm.tokens.push(init::Tokens::print(init::FileDescriptor::STDOUT, "%mystring \\n".chars().collect()));
```
---
Check out the full documentation for xasm to know more on how to use it!

---

A big bear hug to ([neva](https://github.com/nevakrien)) for being an absolute star and guiding light in the journey of creating xasm!

---
# License
**Xasm** is licensed under the **Joay-License** which in short says that:

***The Joay License allows non-commercial use, modification, and distribution of the software with proper attribution. Commercial use and converting the software to closed-source are prohibited***
