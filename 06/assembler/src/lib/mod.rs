mod errors;
mod instructions;

use errors::AssemblerError;

use crate::instructions::Instruction;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/**
 * This struct is the main Assembler for the project. It takes in a buffered reader to a file, and outputs
 * the raw binary for the assembled code.
 */
pub struct Assembler {
    pub source: BufReader<File>,
}

/// Public API
impl Assembler {
    /**
     * Creates a new `Assembler` from a `BufReader<File>`.
     */
    pub fn new(source: BufReader<File>) -> Self {
        Self { source }
    }

    /**
     * Assembles the source code into raw binary.
     */
    pub fn assemble(&mut self) -> Result<Vec<u16>, ()> {
        todo!();

        Ok(vec![])
    }
}

/// Private API
impl Assembler {
    /**
     * Parses the source code into a set of instructions.
     * TODO: Can I do this without taking ownership of self?
     */
    fn parse(self) -> Result<Vec<Instruction>, AssemblerError> {
        let instructions = vec![];

        for line in self.source.lines().enumerate() {
            // Get the line and line number.
            let (line_number, line) = line;
            let line = line?; // Fail on error (thanks to the ? operator!).

            // Remove comments.
            let line = line.split("//").next().unwrap().trim();

            // If the line is empty or only whitespace, skip it.
            if line.is_empty() || line.chars().all(|c| c.is_whitespace()) {
                continue;
            }
        }

        todo!();

        Ok(instructions)
    }
}
