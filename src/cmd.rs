use std::boxed::Box;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::core::code::{Code, CodeParseError};
use crate::core::instruction::Instruction;
use crate::core::machine::Machine;
use crate::core::state::State;

#[derive(Debug)]
pub enum CommandError {
    FileError,
    CodeError(CodeParseError),
    IOError(io::Error),
}

impl From<CodeParseError> for CommandError {
    fn from(value: CodeParseError) -> Self {
        Self::CodeError(value)
    }
}

impl From<io::Error> for CommandError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

pub fn run<P: AsRef<Path>>(
    program_path: P,
    trace: bool,
    output: Option<P>,
) -> Result<(), CommandError> {
    let mut outfile: Box<dyn Write> = match output {
        Some(t) => match File::create(t) {
            Ok(f) => Box::new(f) as Box<dyn Write>,
            Err(e) => return Err(e.into()),
        },
        None => Box::new(io::stdout()) as Box<dyn Write>,
    };

    let file_contents: Vec<u8> = match fs::read(program_path) {
        Ok(t) => t,
        Err(e) => return Err(e.into()),
    };
    let code: Code = match Code::try_from(file_contents) {
        Ok(t) => t,
        Err(e) => return Err(e.into()),
    };

    let mut machine: Machine = Machine::new(code);

    if trace {
        /* print initial machine state */
        println!("{:?}", machine.state.clone());

        match machine.run_callback(&clbk) {
            Ok(t) => write!(outfile, "{:?}", t)?,
            Err(e) => eprintln!("{:?}", e),
        };
    } else {
        match machine.run() {
            Ok(t) => write!(outfile, "{}", t)?,
            Err(e) => eprintln!("{:?}", e),
        };
    }

    Ok(())
}

fn clbk(state: State, instruction: Instruction) {
    println!("[{:?}] {:?}", instruction, state);
}
