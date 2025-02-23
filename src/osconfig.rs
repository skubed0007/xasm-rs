use std::collections::HashMap;

#[allow(non_camel_case_types)]
#[derive(Debug,Clone)]
/// Represents the operating system configuration.
///
/// The `OsConfig` enum is used to specify the target operating system for which
/// the assembly code should be generated. This affects the selection of registers
/// and system calls used in the assembly code.
///
/// # Variants
///
/// * `Linux_X86_64` - Represents a 64-bit Linux operating system configuration.
/// * `Linux_X86_32` - Represents a 32-bit Linux operating system configuration.
#[allow(non_camel_case_types)]
pub enum OsConfig {
    Linux_X86_64,
    Linux_X86_32,
}

impl OsConfig {
    /// Returns a hashmap of registers and their corresponding assembly code representations.
    ///
    /// This function returns a hashmap of registers and their corresponding assembly code
    /// representations. The hashmap contains the name of the register as the key and the
    /// assembly code representation of the register as the value.
    /// The names (as key) are the register names used in Linux x86_64 Bit assembly
    pub fn get_registers(&self) -> HashMap<String, String> {
        let mut regs = HashMap::new();
        match self {
            OsConfig::Linux_X86_64 => {
                regs.insert("rax".to_string(), "rax".to_string());
                regs.insert("rbx".to_string(), "rbx".to_string());
                regs.insert("rcx".to_string(), "rcx".to_string());
                regs.insert("rdx".to_string(), "rdx".to_string());
                regs.insert("rsi".to_string(), "rsi".to_string());
                regs.insert("rdi".to_string(), "rdi".to_string());
                regs.insert("rbp".to_string(), "rbp".to_string());
                regs.insert("rsp".to_string(), "rsp".to_string());
                regs.insert("r8".to_string(), "r8".to_string());
                regs.insert("r9".to_string(), "r9".to_string());
                regs.insert("r10".to_string(), "r10".to_string());
                regs.insert("r11".to_string(), "r11".to_string());
                regs.insert("r12".to_string(), "r12".to_string());
                regs.insert("r13".to_string(), "r13".to_string());
                regs.insert("r14".to_string(), "r14".to_string());
                regs.insert("r15".to_string(), "r15".to_string());
                regs.insert("xmm0".to_string(), "xmm0".to_string());
                regs.insert("xmm1".to_string(), "xmm1".to_string());
                regs.insert("exit_num_ok".to_string(), "60".to_string());
                regs.insert("syscall_inst".to_string(), "syscall".to_string());
                regs.insert("mov_inst".to_string(), "mov".to_string());
            }
            OsConfig::Linux_X86_32 => {
                regs.insert("rax".to_string(), "eax".to_string());
                regs.insert("rbx".to_string(), "ebx".to_string());
                regs.insert("rcx".to_string(), "ecx".to_string());
                regs.insert("rdx".to_string(), "edx".to_string());
                regs.insert("rsi".to_string(), "esi".to_string());
                regs.insert("rdi".to_string(), "edi".to_string());
                regs.insert("rbp".to_string(), "ebp".to_string());
                regs.insert("rsp".to_string(), "esp".to_string());
                regs.insert("xmm0".to_string(), "xmm0".to_string());
                regs.insert("xmm1".to_string(), "xmm1".to_string());
                regs.insert("exit_num_ok".to_string(), "1".to_string());
                regs.insert("syscall_inst".to_string(), "syscall".to_string());
                regs.insert("mov_inst".to_string(), "mov".to_string());
            }
        }
        regs
    }
}
