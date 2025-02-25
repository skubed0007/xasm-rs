pub mod genasm;
pub mod init;
pub mod mkwnasm;
pub mod osconfig;

#[cfg(test)]
mod tests {
    use crate::mkwnasm::compile_with_nasm;

    use super::*;

    #[test]
    pub fn testxasm() {
        let mut xasm = init::Xasm::new();
        xasm.tokens.push(init::Tokens::print(
            init::FileDescriptor::STDOUT,
            "\"hello world\"".chars().collect(),
        ));
        xasm.tokens.push(init::Tokens::print(
            init::FileDescriptor::STDERR,
            "\"Hello Universe!\"".chars().collect(),
        ));
        let asm = genasm::genasm(&xasm, osconfig::OsConfig::Linux_X86_64);
        println!("{}", asm);
        match compile_with_nasm(&asm,osconfig::OsConfig::Linux_X86_64) {
            Ok(()) => (),
            Err(e) => panic!("compilation failed: {}", e),
        }
        // Optionally, add assertions here
    }
}
