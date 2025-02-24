use std::{fs, io::Write, path::Path};

/// Assembles and links the given NASM assembly code into an executable using NASM and LD.
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
pub fn compile_with_nasm(code: &String) -> Result<(), String> {
    let asm_path = Path::new("_.asm");
    if asm_path.exists() {
        fs::remove_file(asm_path).map_err(|e| e.to_string())?;
    }
    {
        let mut asm_file = fs::File::create(asm_path).map_err(|e| e.to_string())?;
        asm_file.write_all(code.as_bytes()).map_err(|e| e.to_string())?;
    }
    let nasm_status = std::process::Command::new("nasm")
        .arg("-felf64")
        .arg("-o")
        .arg("_.o")
        .arg("_.asm")
        .status()
        .map_err(|e| e.to_string())?;
    if !nasm_status.success() {
        return Err("Error running nasm".to_string());
    }
    let ld_status = std::process::Command::new("ld")
        .arg("-o")
        .arg("out")
        .arg("_.o")
        .status()
        .map_err(|e| e.to_string())?;
    if !ld_status.success() {
        return Err("Error running ld".to_string());
    }
    fs::remove_file(asm_path).map_err(|e| e.to_string())?;
    fs::remove_file("_.o").map_err(|e| e.to_string())?;
    Ok(())
}
