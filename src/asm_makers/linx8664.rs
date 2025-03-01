use crate::init::{LinuxX8664, Variables};
use std::fmt::Write;

pub static INDENT: &str = "    ";

pub fn mk_asm_linx8664(xasm: &LinuxX8664) -> String {
    let mut asm = String::with_capacity(2048);
    let mut txtsec = String::with_capacity(512);
    write!(txtsec, "section .text\n{}global _start\n_start:\n",INDENT).unwrap();
    let mut datasec = String::with_capacity(1024);
    write!(datasec,"section .data\n").unwrap();
    let mut bsssec = String::with_capacity(512);
    write!(bsssec,"section .bss\n").unwrap();
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
    for node in xasm.dump().0 {
        write!(txtsec, "{}{}{}\n",INDENT, INDENT, node).unwrap();
    }
    asm.push_str(&txtsec);
    asm.push_str(&datasec);
    asm.push_str(&bsssec);
    asm
}
