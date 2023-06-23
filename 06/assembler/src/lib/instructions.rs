use super::Span;

/// The main "AST" type for the assembler. This barely counts as an AST, buuuuut it's kinda required for a
/// single-pass assembler because it needs some stored context.
#[derive(Debug, Clone)]
pub struct Instruction<'a> {
    /// The kind of instruction.
    pub kind: InstructionKind,

    /// The span of text that the instruction was parsed from.
    pub span: Span<'a>,
    /// The raw text of the instruction.
    pub raw: String,
}

/// The type of instruction. It carries some data with it, which is used to determine how to assemble the
/// instruction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InstructionKind {
    // An A instruction with a literal. This just tells the assembler to load the literal value
    // into the A register. Note that the value is stored as a u16, but the assembler will only
    // use the lower 15 bits.
    ALiteral {
        /// The literal value to load into the A register.
        value: u16,
    },

    /// An A instruction with a variable or label. This tells the assembler to load the value of
    /// the variable or label into the A register, which is decided by the assembler.
    AVariable {
        /// The name of the variable or label to load into the A register.
        name: String,
    },

    /// A C instruction. This tells the assembler to perform some computation and store the result
    /// in the D register. All bools/bits stored here are the raw bits for the instruction, in the
    /// correct order they should be stored in raw binary format.
    C {
        /// Whether or not we should use the A register or memory for the computation.
        /// false: a = 0, don't use the A register, use memory
        /// true: a = 1, use the A register, don't use memory
        a: bool,

        /// The destination register for the result of the computation.
        /// dest[0] == true: memory
        /// dest[1] == true: D register
        /// dest[2] == true: A register
        dest: [bool; 3],

        /// The computation flag bits for the computation.
        comp: [bool; 6],

        /// The jump flag bits for the computation.
        jump: [bool; 3],
    },
}
