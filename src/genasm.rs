use super::{
    init::{FileDescriptor, MutVars, Tokens, Vars, Xasm},
    osconfig::OsConfig,
};

pub static INDENT: &str = "    ";

/// Generates assembly code from the given xasm object
///
/// This function takes an xasm object (which contains information about variables,
/// functions, and tokens) and an operating system configuration. It produces a string
/// containing assembly code with a data section, a bss section, a text section, and a function section.
pub fn genasm(xasm: &Xasm, osconf: &OsConfig) -> String {
    let mut data_section = String::from("section .data\n");
    data_section.push_str(format!("{}{}_jnl_: db 0x0a,0\n", INDENT, "").as_str());
    data_section.push_str(format!("{}{}_spc_: db 0x20,0\n", INDENT, "").as_str());

    let mut bss_section = String::from("section .bss\n");
    let mut text_section = String::new();
    let mut funcs = String::new();
    let mut assembly = String::new();
    let mut collectvars: Vec<(usize, String)> = Vec::new();
    let regs = osconf.get_registers();
    let mut haserror = false;

    // Process immutable variables (data section)
    for var in xasm.vars.iter() {
        match var {
            Vars::I8(name, value) => {
                data_section.push_str(&format!("{}{}: db {}\n", INDENT, name, value));
                collectvars.push((1, name.clone()));
            }
            Vars::I16(name, value) => {
                data_section.push_str(&format!("{}{}: dw {}\n", INDENT, name, value));
                collectvars.push((2, name.clone()));
            }
            Vars::I32(name, value) => {
                data_section.push_str(&format!("{}{}: dd {}\n", INDENT, name, value));
                collectvars.push((4, name.clone()));
            }
            Vars::I64(name, value) => {
                data_section.push_str(&format!("{}{}: dq {}\n", INDENT, name, value));
                collectvars.push((8, name.clone()));
            }
            Vars::F32(name, value) => {
                data_section.push_str(&format!("{}{}: dd {}\n", INDENT, name, value));
                collectvars.push((4, name.clone()));
            }
            Vars::F64(name, value) => {
                data_section.push_str(&format!("{}{}: dq {}\n", INDENT, name, value));
                collectvars.push((8, name.clone()));
            }
            Vars::Char(name, value) => {
                data_section.push_str(&format!("{}{}: db '{}'\n", INDENT, name, value));
                collectvars.push((1, name.clone()));
            }
            Vars::String(name, value) => {
                data_section.push_str(&format!("{}{}: db \"{}\", 0\n", INDENT, name, value));
                collectvars.push((value.len() + 1, name.clone()));
            }
        }
    }

    // Process mutable variables (bss section)
    for var in xasm.mut_vars.iter() {
        match var {
            MutVars::I8(name, _) => {
                bss_section.push_str(&format!("{}{}: resb 1\n", INDENT, name));
                collectvars.push((1, name.clone()));
            }
            MutVars::I16(name, _) => {
                bss_section.push_str(&format!("{}{}: resw 1\n", INDENT, name));
                collectvars.push((2, name.clone()));
            }
            MutVars::I32(name, _) => {
                bss_section.push_str(&format!("{}{}: resd 1\n", INDENT, name));
                collectvars.push((4, name.clone()));
            }
            MutVars::I64(name, _) => {
                bss_section.push_str(&format!("{}{}: resq 1\n", INDENT, name));
                collectvars.push((8, name.clone()));
            }
            MutVars::F32(name, _) => {
                bss_section.push_str(&format!("{}{}: resd 1\n", INDENT, name));
                collectvars.push((4, name.clone()));
            }
            MutVars::F64(name, _) => {
                bss_section.push_str(&format!("{}{}: resq 1\n", INDENT, name));
                collectvars.push((8, name.clone()));
            }
            MutVars::Char(name, _) => {
                bss_section.push_str(&format!("{}{}: resb 1\n", INDENT, name));
                collectvars.push((1, name.clone()));
            }
            MutVars::String(name, val) => {
                bss_section.push_str(&format!("{}{}: resb 256\n", INDENT, name));
                collectvars.push((val.len(), name.clone()));
            }
        }
    }

    // Start the text section with _start label
    text_section.push_str("section .text\n");
    let mut ptok_counter = 0;
    text_section.push_str("global _start\n");
    text_section.push_str("_start:\n");

    // Process tokens (for now, handling only the 'print' token)
    for tok in xasm.tokens.iter() {
        match tok {
            Tokens::print(fd, chars) => {
                let fd_num = match fd {
                    FileDescriptor::STDOUT => 1,
                    FileDescriptor::STDERR => 2,
                    FileDescriptor::STDIN => 0,
                };
                let mut literal = String::new();
                let mut iter = chars.into_iter().peekable();
                while let Some(c) = iter.next() {
                    if *c == '%' {
                        // Flush the current literal as a token if any
                        data_section.push_str(&format!(
                            "{}ptok{}: db \"{}\", 0\n",
                            INDENT,
                            ptok_counter,
                            literal.trim_matches('\"')
                        ));
                        if !literal.is_empty() {
                            let len = literal.len();
                            text_section.push_str(&format!("{}mov {}, 1\n", INDENT, regs.get("rax").unwrap()));
                            text_section.push_str(&format!("{}mov {}, {}\n", INDENT, regs.get("rdi").unwrap(), fd_num));
                            text_section.push_str(&format!("{}mov {}, ptok{}\n", INDENT, regs.get("rsi").unwrap(), ptok_counter));
                            text_section.push_str(&format!("{}mov {}, {}\n", INDENT, regs.get("rdx").unwrap(), len));
                            text_section.push_str(&format!("{}{}\n", INDENT, regs.get("syscall").unwrap()));
                            ptok_counter += 1;
                            literal.clear();
                        }
                        let mut var_name = String::new();
                        while let Some(&ch) = iter.peek() {
                            if ch.is_whitespace() || ch == &'\\' {
                                break;
                            }
                            var_name.push(*ch);
                            iter.next();
                        }
                        if collectvars.iter().find(|(_, name)| name == &var_name).is_none() {
                            eprintln!("Variable {} not found", var_name);
                            var_name.clear();
                            haserror = true;
                        }
                        if !var_name.is_empty() {
                            text_section.push_str(&format!("{}mov {}, 1\n", INDENT, regs.get("rax").unwrap()));
                            text_section.push_str(&format!("{}mov {}, {}\n", INDENT, regs.get("rdi").unwrap(), fd_num));
                            text_section.push_str(&format!("{}mov {}, {}\n", INDENT, regs.get("rsi").unwrap(), var_name));
                            let var_size = collectvars.iter()
                                .find(|(_, name)| name == &var_name)
                                .map(|(size, _)| *size)
                                .unwrap_or(0);
                            text_section.push_str(&format!("{}mov {}, {}\n", INDENT, regs.get("rdx").unwrap(), var_size));
                            text_section.push_str(&format!("{}{}\n", INDENT, regs.get("syscall").unwrap()));
                        }
                    } else if *c == '\\' {
                        if let Some(next_c) = iter.next() {
                            // Flush the current literal if any
                            data_section.push_str(&format!("{}ptok{}: db \"{}\", 0\n", INDENT, ptok_counter, literal.trim_matches('\"')));
                            if !literal.is_empty() && literal != " " {
                                let len = literal.len();
                                text_section.push_str(&format!("{}mov {}, 1\n", INDENT, regs.get("rax").unwrap()));
                                text_section.push_str(&format!("{}mov {}, {}\n", INDENT, regs.get("rdi").unwrap(), fd_num));
                                text_section.push_str(&format!("{}mov {}, ptok{}\n", INDENT, regs.get("rsi").unwrap(), ptok_counter));
                                text_section.push_str(&format!("{}mov {}, {}\n", INDENT, regs.get("rdx").unwrap(), len));
                                text_section.push_str(&format!("{}{}\n", INDENT, regs.get("syscall").unwrap()));
                                ptok_counter += 1;
                                literal.clear();
                            } else {
                                text_section.push_str(&format!("{}mov {}, 1\n", INDENT, regs.get("rax").unwrap()));
                                text_section.push_str(&format!("{}mov {}, {}\n", INDENT, regs.get("rdi").unwrap(), fd_num));
                                text_section.push_str(&format!("{}mov {}, _spc_\n", INDENT, regs.get("rsi").unwrap()));
                                text_section.push_str(&format!("{}mov {}, 1\n", INDENT, regs.get("rdx").unwrap()));
                                text_section.push_str(&format!("{}{}\n", INDENT, regs.get("syscall").unwrap()));
                                ptok_counter += 1;
                                literal.clear();
                            }
                            match *next_c {
                                '\\' => {
                                    text_section.push_str(&format!("{}mov {}, 1\n", INDENT, regs.get("rax").unwrap()));
                                    text_section.push_str(&format!("{}mov {}, {}\n", INDENT, regs.get("rdi").unwrap(), fd_num));
                                    text_section.push_str(&format!("{}mov {}, '{}'\n", INDENT, regs.get("rsi").unwrap(), '\\'));
                                    text_section.push_str(&format!("{}mov {}, 1\n", INDENT, regs.get("rdx").unwrap()));
                                    text_section.push_str(&format!("{}{}\n", INDENT, regs.get("syscall").unwrap()));
                                }
                                't' => {
                                    text_section.push_str(&format!("{}mov {}, 1\n", INDENT, regs.get("rax").unwrap()));
                                    text_section.push_str(&format!("{}mov {}, {}\n", INDENT, regs.get("rdi").unwrap(), fd_num));
                                    text_section.push_str(&format!("{}mov {}, '{}'\n", INDENT, regs.get("rsi").unwrap(), "0x09,0"));
                                    text_section.push_str(&format!("{}mov {}, 1\n", INDENT, regs.get("rdx").unwrap()));
                                    text_section.push_str(&format!("{}{}\n", INDENT, regs.get("syscall").unwrap()));
                                }
                                'n' => {
                                    text_section.push_str(&format!("{}mov {}, 1\n", INDENT, regs.get("rax").unwrap()));
                                    text_section.push_str(&format!("{}mov {}, {}\n", INDENT, regs.get("rdi").unwrap(), fd_num));
                                    text_section.push_str(&format!("{}mov {}, _jnl_\n", INDENT, regs.get("rsi").unwrap()));
                                    text_section.push_str(&format!("{}mov {}, 1\n", INDENT, regs.get("rdx").unwrap()));
                                    text_section.push_str(&format!("{}{}\n", INDENT, regs.get("syscall").unwrap()));
                                }
                                other => {
                                    text_section.push_str(&format!("{}mov {}, 1\n", INDENT, regs.get("rax").unwrap()));
                                    text_section.push_str(&format!("{}mov {}, {}\n", INDENT, regs.get("rdi").unwrap(), fd_num));
                                    text_section.push_str(&format!("{}mov {}, '{}'\n", INDENT, regs.get("rsi").unwrap(), other));
                                    text_section.push_str(&format!("{}mov {}, 1\n", INDENT, regs.get("rdx").unwrap()));
                                    text_section.push_str(&format!("{}{}\n", INDENT, regs.get("syscall").unwrap()));
                                }
                            }
                        }
                    } else {
                        literal.push(*c);
                    }
                }
                // Flush any remaining literal
                data_section.push_str(&format!("{}ptok{}: db \"{}\", 0\n", INDENT, ptok_counter, literal.trim_matches('\"')));
                if !literal.is_empty() {
                    let len = literal.len();
                    text_section.push_str(&format!("{}mov {}, 1\n", INDENT, regs.get("rax").unwrap()));
                    text_section.push_str(&format!("{}mov {}, {}\n", INDENT, regs.get("rdi").unwrap(), fd_num));
                    text_section.push_str(&format!("{}mov {}, ptok{}\n", INDENT, regs.get("rsi").unwrap(), ptok_counter));
                    text_section.push_str(&format!("{}mov {}, {}\n", INDENT, regs.get("rdx").unwrap(), len));
                    text_section.push_str(&format!("{}{}\n", INDENT, regs.get("syscall").unwrap()));
                    ptok_counter += 1;
                }
            }
            // You can add other token types here as needed.
        }
    }

    // Append function definitions (if any)
    for func in xasm.funcs.iter() {
        funcs.push_str(&format!("{}:\n", func.name));
    }

    assembly.push_str(&data_section);
    assembly.push_str("\n");
    assembly.push_str(&bss_section);
    assembly.push_str("\n");
    assembly.push_str(&text_section);

    // Append the exit sequence using the appropriate syscall number
    assembly.push_str(&format!("{}mov {}, {}\n", INDENT, regs.get("rax").unwrap(), regs.get("exit_num_ok").unwrap()));
    assembly.push_str(&format!("{}xor {},{} {}\n", INDENT, regs.get("rdi").unwrap(), regs.get("rdi").unwrap(),INDENT));
    assembly.push_str(&format!("{}{}\n", INDENT, regs.get("syscall").unwrap()));

    assembly.push_str("\n");
    assembly.push_str(&funcs);

    if haserror {
        eprintln!("Fix all the errors in order to compile the assembly");
        std::process::exit(1);
    }

    assembly
}
