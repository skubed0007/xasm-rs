#[derive(Debug,Clone)]
/// FileDescriptor represents a standard file stream in a system.
///
/// This enum is used to specify which standard file descriptor to use
/// for input and output operations. It is typically used in conjunction
/// with tokens that require a file descriptor to perform operations like
/// printing or reading data.
///
/// # Variants
///
/// * `STDIN` - Represents the standard input stream.
/// * `STDOUT` - Represents the standard output stream.
/// * `STDERR` - Represents the standard error stream.
pub enum FileDescriptor {
    STDIN = 0,
    STDOUT = 1,
    STDERR = 2,
}
#[derive(Debug,Clone)]
/// Xasm is the main struct used to generate assembly code.
///
/// Xasm holds all the information needed to generate assembly code.
/// It contains information about variables, functions, and tokens.
///
/// The information is stored in the following fields:
///
/// * `vars` - A vector of `Vars`, which represents variables declared in the source code.
/// * `mut_vars` - A vector of `MutVars`, which represents mutable variables declared in the source code.
/// * `funcs` - A vector of `Func`, which represents functions declared in the source code.
/// * `tokens` - A vector of `Tokens`, which represents tokens declared in the source code.
///
/// To generate assembly code, the `genasm` function should be called.
/// The `genasm` function takes a `Xasm` object and an `OsConfig` object as arguments,
/// and returns a string representing the assembly code.
pub struct Xasm {
    pub vars : Vec<Vars>,
    pub mut_vars : Vec<MutVars>,
    pub funcs : Vec<Func>,
    pub tokens : Vec<Tokens>,
}
#[allow(non_camel_case_types)]
#[derive(Debug,Clone)]
pub enum Tokens {
    /// Represents a token to print a string to a file descriptor.
    ///
    /// The `print` token is used to send a formatted string to a specified
    /// file descriptor, such as STDOUT, STDERR, or a custom file. The string
    /// is provided as a vector of characters.
    ///
    /// # Arguments
    ///
    /// * `file_descriptor` - The file descriptor to which the string will be printed.
    /// * `string` - A vector of characters representing the string to be printed.
    ///
    /// # Examples
    ///
    /// ```
    /// let token = Tokens::print(FileDescriptor::STDOUT, "Hello, world!\n".chars().collect());
    /// ```
    print(FileDescriptor, Vec<char>),
}
#[derive(Debug,Clone)]
pub enum Vars {
    /// Represents an immutable 8-bit signed integer variable.
    ///
    /// The first argument is the name of the variable, and the second argument is the value of the variable.
    I8(String, i8),
    /// Represents an immutable 16-bit signed integer variable.
    ///
    /// The first argument is the name of the variable, and the second argument is the value of the variable.
    I16(String, i16),
    /// Represents an immutable 32-bit signed integer variable.
    ///
    /// The first argument is the name of the variable, and the second argument is the value of the variable.
    I32(String, i32),
    /// Represents an immutable 64-bit signed integer variable.
    ///
    /// The first argument is the name of the variable, and the second argument is the value of the variable.
    I64(String, i64),
    /// Represents an immutable 32-bit floating-point number variable.
    ///
    /// The first argument is the name of the variable, and the second argument is the value of the variable.
    F32(String, f32),
    /// Represents an immutable 64-bit floating-point number variable.
    ///
    /// The first argument is the name of the variable, and the second argument is the value of the variable.
    F64(String, f64),
    /// Represents an immutable character variable.
    ///
    /// The first argument is the name of the variable, and the second argument is the value of the variable.
    Char(String, char),
    /// Represents an immutable string variable.
    ///
    /// The first argument is the name of the variable, and the second argument is the value of the variable.
    String(String, String),
}
#[derive(Debug,Clone)]
pub enum MutVars {
    /// Represents a mutable 8-bit signed integer variable.
    ///
    /// The first argument is the name of the variable, and the second argument is the value of the variable.
    I8(String,i8),
    /// Represents a mutable 16-bit signed integer variable.
    ///
    /// The first argument is the name of the variable, and the second argument is the value of the variable.
    I16(String,i16),
    /// Represents a mutable 32-bit signed integer variable.
    ///
    /// The first argument is the name of the variable, and the second argument is the value of the variable.
    I32(String,i32),
    /// Represents a mutable 64-bit signed integer variable.
    ///
    /// The first argument is the name of the variable, and the second argument is the value of the variable.
    I64(String,i64),
    /// Represents a mutable 32-bit floating-point number variable.
    ///
    /// The first argument is the name of the variable, and the second argument is the value of the variable.
    F32(String,f32),
    /// Represents a mutable 64-bit floating-point number variable.
    ///
    /// The first argument is the name of the variable, and the second argument is the value of the variable.
    F64(String,f64),
    /// Represents a mutable character variable.
    ///
    /// The first argument is the name of the variable, and the second argument is the value of the variable.
    Char(String,char),
    /// Represents a mutable string variable.
    ///
    /// The first argument is the name of the variable, and the second argument is the value of the variable.
    String(String,String),
}
#[derive(Debug,Clone)]
/// Represents a function in the Xasm assembly code.
///
/// The `Func` struct is used to define a function, including its name, arguments, body,
/// and return value. It provides the necessary structure to represent a function in the
/// assembly code generated by Xasm.
///
/// # Fields
///
/// * `name` - A `String` representing the name of the function.
/// * `args` - An `Option<Vec<Vars>>` representing the function's arguments as immutable variables.
/// * `mut_args` - An `Option<Vec<MutVars>>` representing the function's arguments as mutable variables.
/// * `body` - An `Xasm` struct representing the body of the function, which contains tokens, variables,
///   and other elements that make up the function's implementation.
/// * `ret` - An `Option<Vars>` representing the return value of the function, if any.
pub struct Func {
    pub name: String,
    pub args: Option<Vec<Vars>>,
    pub mut_args: Option<Vec<MutVars>>,
    pub body: Xasm,
    pub ret: Option<Vars>,
}
impl Xasm {
    /// Creates a new `Xasm` object with empty fields.
    ///
    /// Returns a new `Xasm` object with empty vectors for `vars`, `mut_vars`, `funcs`, and `tokens`.
    pub fn new() -> Xasm {
        Xasm {
            vars: Vec::new(),
            mut_vars: Vec::new(),
            funcs: Vec::new(),
            tokens: Vec::new(),
        }
    }
}