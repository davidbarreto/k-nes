mod types;

use crate::assembler::types::NumericType;
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

/// Parse source code line by line, saving each 'logical line' in a certain position
/// of the vector.
/// It aggregates labels definitions with instructions.
/// Creates a symbol table, so labels can be resolved into addresses later
fn parse_source_code(source_code: &str) -> Vec<&str> {
    Vec::new()
}

/// Resolve the labels used in the source code.
/// Return
fn resolve_labels(source: &str) -> Vec<&str> {
    Vec::new()
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
    Ok(parse_data(op, addressing_mode, data))
}

/// Parse the data associated with instruction (operand) and return the values associated with them
/// in a Vec<u8>. First position of vector is the opcode already translated (op)
fn parse_data(op: u8, addressing_mode: AddressingMode, data: &str) -> Vec<u8> {
    
    // Get the number capture of the regex. As the regex already matched at this point, we can
    // safely do the unwrap.
    let numeric_string = &addressing_mode.regex().captures(data).unwrap()["number"];
    // Check what is the numeric type (binary, octal, decimal or hexadecimal)
    let (numeric_type, value) = NumericType::detect_type_in_string(numeric_string);
    
    let mut result: Vec<u8> = Vec::with_capacity(3);
    result.push(op);

    let number = u16::from_str_radix(value, numeric_type.to_radix()).unwrap();

    // If number fits one byte, then save it as 8-bit number
    if number & 0xFF00 == 0 {
        result.push(number as u8);

    // If not, save 16-bit number separated in two parrs of 8-bit.
    // As we use little endian, we store last byte first
    } else {
        
        result.push((number & 0x00FF) as u8);
        result.push((number >> 8) as u8);
    }
    result
}

/// Given the instruction data and the list of possible addressing modes for the current instruction
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

