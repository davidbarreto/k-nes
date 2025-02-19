pub mod types;
pub mod parser;

use std::collections::HashMap;

use types::{Command, SymbolType};

use crate::cpu::opcode;
use crate::cpu::types::InstructionError;
use crate::memory::types::AddressingMode;

pub fn assemble() {
    // 1. Read source file into a list of lines
    // 2. Strip comments and normalize whitespace
    // 3. First pass: Resolve labels and calculate addresses
    // 4. Second pass: Parse mnemonics and operands, generate machine code
    // 5. Handle relative addresses (branch instructions)
    // 6. Handle control structures (directives like .org, .byte, .word)
    // 7. Output assembled bytes
}

/// Process each line of the source code
/// Could return a ParseError
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

#[cfg(test)]
mod tests;