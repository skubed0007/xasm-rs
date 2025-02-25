use std::{fs, io::Write, path::Path};

use crate::osconfig::OsConfig;

/// Compiles the given NASM assembly code into an executable using NASM and LD.
///
/// # Arguments
///
/// * `code` - A string containing the NASM assembly source code.
///
/// # Returns
///
/// Returns `Ok(())` if the compilation and linking succeed, or an `Err(String)` if any step fails.
///
/// # Examples
///
/// ```
/// let asm_code = String::from("section .text\nglobal _start\n_start:\n  mov rax, 60\n  xor rdi, rdi\n  syscall\n");
/// compile_with_nasm(&asm_code).expect("Compilation failed");
/// // The final executable is produced as `out`.
/// ```
///
/// # Notes
///
/// This function quickly removes any pre-existing temporary files ("_.asm" and "_.o"),
/// creates a new assembly file, compiles it with NASM into an object file, links the object
/// file into an executable named `out`, and then removes the temporary files.
pub fn compile_with_nasm(code: &String,osconf : OsConfig) -> Result<(), String> {
    if osconf == OsConfig::Linux_X86_64 {
        let asm_path = Path::new("_.asm");
        if asm_path.exists() {
            fs::remove_file(asm_path).map_err(|e| e.to_string())?;
        }
    
        // Write assembly code to file
        {
            let mut asm_file = fs::File::create(asm_path).map_err(|e| e.to_string())?;
            asm_file.write_all(code.as_bytes()).map_err(|e| e.to_string())?;
        }
    
        // Assemble for 64-bit
        let nasm_status = std::process::Command::new("nasm")
            .args(["-f", "elf64", "-o", "_.o", "_.asm"])
            .status()
            .map_err(|e| e.to_string())?;
    
        if !nasm_status.success() {
            return Err("Error running nasm".to_string());
        }
    
        // Link for 64-bit
        let ld_status = std::process::Command::new("ld")
            .args(["-o", "out", "_.o"])
            .status()
            .map_err(|e| e.to_string())?;
    
        if !ld_status.success() {
            return Err("Error running ld".to_string());
        }
    
        fs::remove_file(asm_path).map_err(|e| e.to_string())?;
        fs::remove_file("_.o").map_err(|e| e.to_string())?;
        Ok(())
    } else if osconf == OsConfig::Linux_X86_32 {
        let asm_path = Path::new("_.asm");
        if asm_path.exists() {
            fs::remove_file(asm_path).map_err(|e| e.to_string())?;
        }
    
        // Write assembly code to file
        {
            let mut asm_file = fs::File::create(asm_path).map_err(|e| e.to_string())?;
            asm_file.write_all(code.as_bytes()).map_err(|e| e.to_string())?;
        }
    
        // Assemble for 32-bit
        let nasm_status = std::process::Command::new("nasm")
            .args(["-f", "elf32", "-o", "_.o", "_.asm"])
            .status()
            .map_err(|e| e.to_string())?;
    
        if !nasm_status.success() {
            return Err("Error running nasm for 32-bit".to_string());
        }
    
        // Link for 32-bit
        let ld_status = std::process::Command::new("ld")
            .args(["-m", "elf_i386", "-o", "out", "_.o"])
            .status()
            .map_err(|e| e.to_string())?;
    
        if !ld_status.success() {
            return Err("Error running ld for 32-bit".to_string());
        }
    
        fs::remove_file(asm_path).map_err(|e| e.to_string())?;
        fs::remove_file("_.o").map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Unsupported OS configuration".to_string())
    }
    
}
