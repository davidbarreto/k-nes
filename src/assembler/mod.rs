pub mod types;
pub mod parser;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

use types::{Command, SymbolType};

use crate::constants;
use crate::cpu::opcode;
use crate::cpu::types::InstructionError;
use crate::memory::types::AddressingMode;

pub fn assemble(filename: &str, output_filename: Option<&str>) {

    let mut line_number: usize = 1;
    let mut address: u16 = 0;
    let mut program_binary: Vec<u8> = Vec::with_capacity(200);
    let mut symbol_table: HashMap<String, Command> = HashMap::with_capacity(50);

    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            if let Err(parse_error) = process_line(&line, &mut address, &mut program_binary, &mut symbol_table) {
                panic!("Error in line {}: {:?}", line_number, parse_error);
            }
            line_number += 1;
        }
    }

    let output = match output_filename {
        Some(name) => name,
        None => constants::DEFAULT_OUTPUT_FILENAME,
    };

    if let Err(error) = write_file(output, program_binary) {
        panic!("Error creating output binary file: {}", error);
    };
}

/// Process each line of the source code
/// Could return a ParseError
/// TODO Handle relative addresses (branch instructions)
/// TODO Handle more directives
pub fn process_line(line: &str, address: &mut u16, program_binary: &mut Vec<u8>, symbol_table: &mut HashMap<String, Command>) -> Result<(), types::ParseError> {
    let mut command = parser::parse_line(line, *address, symbol_table)?;
    if command.symbol.symbol_type != SymbolType::UNDEFINED {
        // Try to replace any symbols we already capture at the first step
        parser::replace_symbols(&mut command, &symbol_table);

        // Parse commands
        if command.symbol.symbol_type == SymbolType::DIRECTIVE {
            if command.symbol.name == ".org" {
                *address = parser::parse_org_data(command, &symbol_table)?;
            }
        } else if command.symbol.symbol_type == SymbolType::MNEMONIC {
            let result = assemble_instruction(&command.symbol.name, &command.data).clone();
            let Some(mut instruction_binary) = result.clone().ok() else {
                return Err(types::ParseError::InstructionError(result.unwrap_err()));
            };
            *address += instruction_binary.len() as u16;
            program_binary.append(&mut instruction_binary);
        }
    }
    Ok(())
}

/// Assemble a single instruction. Returns a Vec<u8> containing the machine code for that instruction
/// Returns Err<InstructionError> if any error found during translation
pub fn assemble_instruction(mnemonic: &str, data: &str) -> Result<Vec<u8>, InstructionError> {
    // Get all possible addressing modes for that mnemonic
    let addressing_modes = opcode::addressing_modes_from_mnemonic(mnemonic)?;
    // For each possible addressing mode, apply its regex to check which one is desired
    let addressing_mode = define_addressing_mode(data, &addressing_modes)?;
    // Given the mnemonic string and the addressing mode, get the opcode (8-bit integer)
    let op = opcode::translate_instruction_to_opcode(mnemonic, addressing_mode)?;

    // Parse data associated to the instruction
    Ok(parser::parse_instruction_data(op, addressing_mode, data))
}

// Given the instruction data and the list of possible addressing modes for the current instruction
/// returns the desired addressing mode. It's done by applying each addressing mode associated regex
/// and check if the data matches.
/// Return the addressing mode which associated regex matches the data
/// Return InstructionError::AddressingModeNotRecognized if none of the regex match the data
fn define_addressing_mode(data: &str, addressing_modes: &Vec<AddressingMode>) -> Result<AddressingMode, InstructionError> {
    for addressing_mode in addressing_modes {
        if addressing_mode.regex().is_match(data) {
            return Ok(*addressing_mode);
        }
    }
    Err(InstructionError::AddressingModeNotRecognized(data.to_string()))
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn write_file<P>(filename: P, binary_vec: Vec<u8>) -> io::Result<()>
where P: AsRef<Path>, {
    let mut file = File::create(filename)?;
    file.write_all(&binary_vec)?;
    Ok(())
}

#[cfg(test)]
mod tests;