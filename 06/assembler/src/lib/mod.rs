mod errors;
mod instructions;

use crate::instructions::Instruction;
use errors::AssemblerError;
use nom::{
    bytes::complete::{is_not, tag, take_until},
    combinator::value,
    error::ParseError,
    sequence::{delimited, pair},
    IResult,
};
use nom_locate::LocatedSpan;
use std::{fs::File, io::BufReader};

/// The span type for data.
type Span<'a> = LocatedSpan<&'a str>;

/// This struct is the main Assembler for the project. It takes in a buffered reader to a file, and outputs
/// the raw binary for the assembled code.
pub struct Assembler<'a> {
    /// The source file to read from.
    pub source: &'a str,
}

/// Public API
impl Assembler<'_> {
    /// Creates a new `Assembler` from a `BufReader<File>`.
    pub fn new(source: BufReader<File>) -> Self {
        Self {
            source: source.into_inner(),
        }
    }

    /// Assembles the source code into raw binary.
    pub fn assemble(&mut self) -> Result<Vec<u16>, AssemblerError> {
        todo!();

        Ok(vec![])
    }
}

/// Private API
impl Assembler<'_> {
    /// Parses the source code into a set of instructions.
    fn parse<'a>(&mut self) -> Result<Vec<Instruction<'a>>, AssemblerError> {
        let input: Span = Span::new(&self.source);

        println!("{:?}", input);

        todo!();

        Ok(vec![])
    }
}

pub fn multiline_comment<'a>(i: &'a str) -> IResult<&'a str, (), ParseError<Span<'a>>> {
    value(
        (), // Output is thrown away.
        delimited(tag("/*"), take_until("*/"), tag("*/")),
    )(i)
}

pub fn singleline_comment<'a>(i: &'a str) -> IResult<&'a str, (), ParseError<Span<'a>>> {
    value(
        (), // Output is thrown away.
        pair(tag(r"//"), is_not("\n\r")),
    )(i)
}
