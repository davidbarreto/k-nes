use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;

use crate::assembler::types::{SymbolType, Command, NumericType, ParseError};
use crate::cpu::opcode;
use crate::memory::types::AddressingMode;
use crate::constants;

lazy_static! {
    static ref NUM_16_BIT_REGEX: Regex = Regex::new(&format!(r"^{}$", constants::NUM_8_BIT.as_str())).unwrap();
    static ref NUM_8_BIT_REGEX: Regex = Regex::new(&format!(r"^{}$", constants::NUM_8_BIT.as_str())).unwrap();
    static ref NUM_UP_TO_16_BIT_REGEX: Regex = Regex::new(&format!(r"^{}$", constants::NUM_UP_TO_16_BIT.as_str())).unwrap();
}

/// Parse 1 line of source code 
/// It aggregates labels definitions with instructions.
/// Creates a symbol table, so labels can be resolved into addresses later
pub fn parse_line(line: &str, address_number: u16, symbol_table: &mut HashMap<String, Command>) -> Result<Command, ParseError> {

    let mut current_symbol_type = SymbolType::UNDEFINED;
    let mut expected_symbol_type = SymbolType::UNDEFINED;
    let mut current_symbol: String = String::new();
    let mut data_tokens: Vec<&str> = Vec::with_capacity(50);

    for token in line.trim().split(' ') {

        // Ignore comments
        if token.starts_with(';') {
            break;
        }
        
        // Handle directive
        if token.starts_with('.') {
            
            // We can't have a directive after any other type of symbol
            if current_symbol_type != SymbolType::UNDEFINED {
                return Err(ParseError::cannot_define_directive_after_other_symbol());
            }

            current_symbol = token.to_string();
            current_symbol_type = SymbolType::DIRECTIVE;
            expected_symbol_type = SymbolType::DATA;

        // Handle label
        } else if token.ends_with(':') {

            current_symbol = token[0..token.len()-1].to_string();
            current_symbol_type = SymbolType::LABEL;
            expected_symbol_type = SymbolType::MNEMONIC;
            symbol_table.insert(current_symbol.clone(), 
                Command::new(current_symbol.clone(), current_symbol_type, address_number.to_string()));

        // Handle constants (left part)
        } else if token == "=" {

            // Equal sign can't follow some symbol types
            match current_symbol_type {
                SymbolType::LABEL | SymbolType::DIRECTIVE | SymbolType::MNEMONIC | SymbolType::DATA | SymbolType::VALUE => {
                    return Err(ParseError::cannot_assign_value_to_non_constant(current_symbol_type));
                },
                SymbolType::CONSTANT => {
                    return Err(ParseError::constant_not_expected());
                },
                SymbolType::UNDEFINED => () //Do nothing
            }
            current_symbol_type = SymbolType::CONSTANT;
            expected_symbol_type = SymbolType::VALUE;

        // Handle right part
        } else {
            
            // Right part is a VALUE
            if expected_symbol_type == SymbolType::VALUE {
                
                symbol_table.insert(current_symbol.clone(),
                    Command::new(current_symbol.clone(), current_symbol_type, token.to_string()));
            }
            // Right part is a DATA
            else if expected_symbol_type == SymbolType::DATA {
    
                // Accumulate tokens
                data_tokens.push(token);
                
            // Let's try to figure out what current symbol is.
            } else {
    
                current_symbol = token.to_string();

                // Handle MNEMONIC
                if (expected_symbol_type == SymbolType::MNEMONIC || expected_symbol_type == SymbolType::UNDEFINED) && opcode::is_valid_mnemonic(token) {    
                    current_symbol_type = SymbolType::MNEMONIC;
                    expected_symbol_type = SymbolType::DATA;
                } else if expected_symbol_type == SymbolType::MNEMONIC {
                    return Err(ParseError::mnemonic_expected(current_symbol));
                }
            }
        }
    }

    if expected_symbol_type == SymbolType::DATA {
        Ok(Command::new(current_symbol.to_string(), current_symbol_type, data_tokens.join(" ")))
    } else {
        Ok(Command::empty())
    }
}

pub fn replace_symbols(command: &mut Command, symbol_table: &HashMap<String, Command>) {
    
    for (key, value) in symbol_table {

        // TODO Handle * as current address (PC)

        // Change occurrences of symbols present in our symbol table
        command.data = command.data.replace(key, &value.data);
    }
}

/// Parse the data associated with instruction (operand) and return the values associated with them
/// in a Vec<u8>. First position of vector is the opcode already translated (op)
pub fn parse_instruction_data(op: u8, addressing_mode: AddressingMode, data: &str) -> Vec<u8> {

    let mut result: Vec<u8> = Vec::with_capacity(3);
    result.push(op);

    if data.is_empty() {
        return result;
    }

    let number = parse_number(&addressing_mode.regex(), &data.to_string()).unwrap();    
    
    // If addressing mode accepts only 1-byte number, then save it as 8-bit number
    if addressing_mode.byte_size() < 3 {
        result.push(number as u8);

    // If not, save 16-bit number separated in two parrs of 8-bit.
    // As we use little endian, we store last byte first
    } else {
        result.push((number & 0x00FF) as u8);
        result.push((number >> 8) as u8);
    }
    result
}

pub fn parse_org_data(org: Command, symbol_table: &HashMap<String, Command>) -> Result<u16, ParseError> {

    if let Ok(number) = parse_number(&NUM_UP_TO_16_BIT_REGEX, &org.data.clone()) {
        return Ok(number);
    } else {
        if symbol_table.contains_key(&org.data) {
            return Ok(parse_number(&NUM_UP_TO_16_BIT_REGEX, &symbol_table.get(&org.data).unwrap().data)?);
        // If not a symbol, try to parse it to a immediate number
        } else {
            return Err(ParseError::SymbolNotDefined(format!("Symbol [{}] not defined", org.data)));
        }
    }
}

fn parse_number(number_regex: &Regex, data: &String) -> Result<u16, ParseError> {

    // Get the number capture of the regex
    if let Some(capture) = &number_regex.captures(&data) {
        let numeric_string = &capture["number"];

        // Check what is the numeric type (binary, octal, decimal or hexadecimal)
        let (numeric_type, value) = NumericType::detect_type_in_string(numeric_string);
        if let Ok(result) = u16::from_str_radix(value, numeric_type.to_radix()) {
            return Ok(result);
        } else {
            return Err(ParseError::InvalidNumber(format!("Invalid integer: {}", numeric_string)));
        }
    } else {
        return Err(ParseError::FatalError("Error detecting number value".to_string()));
    }
}