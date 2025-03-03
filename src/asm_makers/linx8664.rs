use crate::init::{LinuxX8664, Variables};
use std::fmt::Write;

pub static INDENT: &str = "    ";

/// Generates the assembly code for a Linux x86-64 executable.
///
/// This function takes a `LinuxX8664` struct, which contains the necessary information to generate the assembly code, and returns a `String` containing the complete assembly code.
/// The assembly code is divided into three sections: `.text`, `.data`, and `.bss`. The `.text` section contains the executable instructions, the `.data` section contains initialized data, and the `.bss` section contains uninitialized data.
/// The function iterates over the variables in the `LinuxX8664` struct and generates the appropriate assembly instructions for each variable type, such as `db`, `dw`, `dd`, `dq`, and `db` for strings.
/// The function also generates the necessary labels and global symbols for the assembly code.
pub fn mk_asm_linx8664(xasm: &LinuxX8664) -> String {
    let mut asm = String::with_capacity(2048);
    let mut txtsec = String::with_capacity(512);
    write!(txtsec, "section .text\n{}global _start\n_start:\n",INDENT).unwrap();
    let mut datasec = String::with_capacity(1024);
    write!(datasec,"section .data\n").unwrap();
    let mut bsssec = String::with_capacity(512);
    write!(bsssec,"section .bss\n").unwrap();
    let mut funcs = String::with_capacity(512);
    for funs in xasm.dump().3{
        funcs.push_str(format!("{}:\n",funs.name).as_str());
        for inst in &funs.body{
            funcs.push_str(format!("{}{}\n",INDENT,inst).as_str());
        }
    }
    for vars in xasm.dump().1 {
        match vars.1 {
            Variables::I8(val) => write!(datasec, "{}{}: db {}\n", INDENT, vars.0, val).unwrap(),
            Variables::I16(val) => write!(datasec, "{}{}: dw {}\n", INDENT, vars.0, val).unwrap(),
            Variables::I32(val) => write!(datasec, "{}{}: dd {}\n", INDENT, vars.0, val).unwrap(),
            Variables::I64(val) => write!(datasec, "{}{}: dq {}\n", INDENT, vars.0, val).unwrap(),
            Variables::U8(val) => write!(datasec, "{}{}: db {}\n", INDENT, vars.0, val).unwrap(),
            Variables::U16(val) => write!(datasec, "{}{}: dw {}\n", INDENT, vars.0, val).unwrap(),
            Variables::U32(val) => write!(datasec, "{}{}: dd {}\n", INDENT, vars.0, val).unwrap(),
            Variables::U64(val) => write!(datasec, "{}{}: dq {}\n", INDENT, vars.0, val).unwrap(),
            Variables::F32(val) => write!(datasec, "{}{}: dd {}\n", INDENT, vars.0, val).unwrap(),
            Variables::F64(val) => write!(datasec, "{}{}: dq {}\n", INDENT, vars.0, val).unwrap(),
            Variables::Str(ref val) => write!(datasec, "{}{}: db \"{}\", 0\n", INDENT, vars.0, val).unwrap(),
            Variables::Bool(val) => write!(datasec, "{}{}: db {}\n", INDENT, vars.0, if val { 1 } else { 0 }).unwrap(),
            Variables::AsIs(code) => write!(datasec, "{}{}\n", INDENT, code).unwrap(),
        }
    }
    for vars in xasm.dump().2{
        match vars.1 {
            Variables::I8(val) => write!(bsssec, "{}{}: resb {}\n", INDENT, vars.0, val).unwrap(),
            Variables::I16(val) => write!(bsssec, "{}{}: resw {}\n", INDENT, vars.0, val).unwrap(),
            Variables::I32(val) => write!(bsssec, "{}{}: resd {}\n", INDENT, vars.0, val).unwrap(),
            Variables::I64(val) => write!(bsssec, "{}{}: resq {}\n", INDENT, vars.0, val).unwrap(),
            Variables::U8(val) => write!(bsssec, "{}{}: resb {}\n", INDENT, vars.0, val).unwrap(),
            Variables::U16(val) => write!(bsssec, "{}{}: resw {}\n", INDENT, vars.0, val).unwrap(),
            Variables::U32(val) => write!(bsssec, "{}{}: resd {}\n", INDENT, vars.0, val).unwrap(),
            Variables::U64(val) => write!(bsssec, "{}{}: resq {}\n", INDENT, vars.0, val).unwrap(),
            Variables::F32(val) => write!(bsssec, "{}{}: resd {}\n", INDENT, vars.0, val).unwrap(),
            Variables::F64(val) => write!(bsssec, "{}{}: resq {}\n", INDENT, vars.0, val).unwrap(),
            Variables::Str(ref val) => write!(bsssec, "{}{}: resb {}\n", INDENT, vars.0, val.len()).unwrap(),
            Variables::Bool(val) => write!(bsssec, "{}{}: resb {}\n", INDENT, vars.0, if val { 1 } else { 0 }).unwrap(),
            Variables::AsIs(code) => write!(bsssec, "{}{}\n", INDENT, code).unwrap(),
        }
    }
    for node in xasm.dump().0 {
        write!(txtsec, "{}{}{}\n",INDENT, INDENT, node).unwrap();
    }
    asm.push_str(&txtsec);
    asm.push_str(&funcs);
    asm.push_str(&datasec);
    asm.push_str(&bsssec);
    asm
}
