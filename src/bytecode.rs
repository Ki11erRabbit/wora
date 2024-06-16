
pub enum Size {
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
}

pub enum Comparison {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

pub enum Bytecode {
    /// Halt the program
    Halt,
    // Load a constant value into a register
    Load8(u8, u8),
    Load16(u8, u16),
    Load32(u8, u32),
    Load64(u8, u64),
    /// Load a value from a register
    LoadReg(u8, u8),
    /// Load a value from memory
    LoadMem(u8, usize, Size),
    /// Load a value from the stack
    LoadStack(u8, usize, Size),
    /// Add two values
    Add(u8, u8, Size),
    /// Subtract two values
    Sub(u8, u8, Size),
    /// Multiply two values
    Mult(u8, u8, Size),
    /// Divide two values
    Div(u8, u8, Size),
    /// Modulo two values
    Mod(u8, u8, Size),
    /// Logical Shift Left
    LShift(u8, u8, Size),
    /// Logical Shift Right
    RShift(u8, u8, Size),
    /// Arithmetic Shift Left
    ALShift(u8, u8, Size),
    /// Arithmetic Shift Right
    ARShift(u8, u8, Size),
    /// Bitwise AND
    And(u8, u8, Size),
    /// Bitwise OR
    Or(u8, u8, Size),
    /// Bitwise XOR
    Xor(u8, u8, Size),
    /// Bitwise NOT
    Not(u8, Size),
    /// Compare two values
    Cmp(u8, u8, Comparison, Size),
    /// Jump to a specific instruction
    Jmp(usize),
    /// Jump to a specific instruction if a condition is met
    JmpIf(Comparison, usize),
    /// Call a function
    Call(usize),
    /// Return from a function
    Ret,
}
