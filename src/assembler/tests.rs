use std::fs;

use super::*;

#[test]
fn parse_line_comment_only() {

    // Given
    let line = ";This is a comment and should be ignored!";
    let mut symbol_table: HashMap<String, Command> = HashMap::new();
    let expected_command = Command::empty();

    // When
    let result = parser::parse_line(line, 0, &mut symbol_table);

    // Then
    assert_eq!(result, Ok(expected_command));
    assert!(symbol_table.is_empty());
}

#[test]
fn parse_line_mnemonic_without_data() {

    // Given
    let line = "BRK";
    let mut symbol_table: HashMap<String, Command> = HashMap::new();
    let expected_command = Command::new("BRK".to_string(), SymbolType::MNEMONIC, "".to_string());

    // When
    let result = parser::parse_line(line, 0, &mut symbol_table);

    // Then
    assert_eq!(result, Ok(expected_command));
    assert!(symbol_table.is_empty());
}

#[test]
fn parse_line_mnemonic_without_data_and_comments() {

    // Given
    let line = "CLC      ; CLEAR CARRY BIT";
    let mut symbol_table: HashMap<String, Command> = HashMap::new();
    let expected_command = Command::new("CLC".to_string(), SymbolType::MNEMONIC, "".to_string());

    // When
    let result = parser::parse_line(line, 0, &mut symbol_table);

    // Then
    assert_eq!(result, Ok(expected_command));
    assert!(symbol_table.is_empty());
}

#[test]
fn parse_line_mnemonic_with_data() {

    // Given
    let line = "LDA $8000";
    let mut symbol_table: HashMap<String, Command> = HashMap::new();
    let expected_command = Command::new("LDA".to_string(), SymbolType::MNEMONIC, "$8000".to_string());

    // When
    let result = parser::parse_line(line, 0, &mut symbol_table);

    // Then
    assert_eq!(result, Ok(expected_command));
    assert!(symbol_table.is_empty());
}

#[test]
fn parse_line_mnemonic_with_data_and_comments() {

    // Given
    let line = "JMP ADDRESS ; Jump to the address defined by label ADDRESS";
    let mut symbol_table: HashMap<String, Command> = HashMap::new();
    let expected_command = Command::new("JMP".to_string(), SymbolType::MNEMONIC, "ADDRESS".to_string());

    // When
    let result = parser::parse_line(line, 0, &mut symbol_table);

    // Then
    assert_eq!(result, Ok(expected_command));
    assert!(symbol_table.is_empty());
}

#[test]
fn parse_line_label_mnemonic_with_data_and_comments() {

    // Given
    let address: u16 = 0x00FF;
    let line = "LOOP: ADC RESULT ; Sum ACCUMULATOR with constant RESULT";
    let mut symbol_table: HashMap<String, Command> = HashMap::new();

    let expected_command = Command::new("ADC".to_string(), SymbolType::MNEMONIC, "RESULT".to_string());
    let expected_label = Command::new("LOOP".to_string(), SymbolType::LABEL, address.to_string());

    // When
    let result = parser::parse_line(line, address, &mut symbol_table);

    // Then
    assert_eq!(result, Ok(expected_command));
    assert_eq!(symbol_table, HashMap::from([(expected_label.symbol.name.clone(), expected_label)]));
}

#[test]
fn parse_line_label_mnemonic_with_indirection() {

    // Given
    let line = "JMP ($8000)";
    let mut symbol_table: HashMap<String, Command> = HashMap::new();

    let expected_command = Command::new("JMP".to_string(), SymbolType::MNEMONIC, "($8000)".to_string());

    // When
    let result = parser::parse_line(line, 0, &mut symbol_table);

    // Then
    assert_eq!(result, Ok(expected_command));
    assert!(symbol_table.is_empty());
}

#[test]
fn parse_line_constant() {

    // Given
    let line = "RESULT = $00FF";
    let mut symbol_table: HashMap<String, Command> = HashMap::new();

    let expected_command = Command::empty();
    let expected_label = Command::new("RESULT".to_string(), SymbolType::CONSTANT, "$00FF".to_string());

    // When
    let result = parser::parse_line(line, 0, &mut symbol_table);

    // Then
    assert_eq!(result, Ok(expected_command));
    assert_eq!(symbol_table, HashMap::from([(expected_label.symbol.name.clone(), expected_label)]));
}

#[test]
fn parse_line_directive() {

    // Given
    let line = ".org $8000";
    let mut symbol_table: HashMap<String, Command> = HashMap::new();

    let expected_command = Command::new(".org".to_string(), SymbolType::DIRECTIVE, "$8000".to_string());

    // When
    let result = parser::parse_line(line, 0, &mut symbol_table);

    // Then
    assert_eq!(result, Ok(expected_command));
    assert!(symbol_table.is_empty());
}

#[test]
fn parse_line_should_fail_when_directive_after_anything() {

    // Given
    let line = "LABEL: .org $8000";
    let mut symbol_table: HashMap<String, Command> = HashMap::new();

    // When
    let result = parser::parse_line(line, 0, &mut symbol_table);

    // Then
    assert_eq!(result, Err(types::ParseError::cannot_define_directive_after_other_symbol()));
}

#[test]
fn parse_line_should_fail_when_assign_value_to_label() {

    // Given
    let line = "LABEL: = 255";
    let mut symbol_table: HashMap<String, Command> = HashMap::new();

    // When
    let result = parser::parse_line(line, 0, &mut symbol_table);

    // Then
    assert_eq!(result, Err(types::ParseError::cannot_assign_value_to_non_constant(SymbolType::LABEL)));
}

#[test]
fn parse_line_should_fail_when_assign_value_to_mnemonic() {

    // Given
    let line = "LDA = 255";
    let mut symbol_table: HashMap<String, Command> = HashMap::new();

    // When
    let result = parser::parse_line(line, 0, &mut symbol_table);

    // Then
    assert_eq!(result, Err(types::ParseError::cannot_assign_value_to_non_constant(SymbolType::MNEMONIC)));
}

#[test]
fn parse_line_should_fail_when_constant_is_defined_after_label() {

    // Given
    let line = "LABEL: CONSTANT = 2";
    let mut symbol_table: HashMap<String, Command> = HashMap::new();

    // When
    let result = parser::parse_line(line, 0, &mut symbol_table);

    // Then
    assert_eq!(result, Err(types::ParseError::mnemonic_expected("CONSTANT".to_string())));
}

#[test]
fn process_line_should_not_add_any_binary_when_process_label_only() {

    // Given
    let mut address: u16 = 10;
    let line = "LABEL:";
    let mut symbol_table: HashMap<String, Command> = HashMap::new();
    let mut program_binary: Vec<u8> = Vec::new();
    let expected_label = Command::new("LABEL".to_string(), SymbolType::LABEL, "10".to_string());

    // When
    if let Err(error) = process_line(line, &mut address, &mut program_binary, &mut symbol_table) {
        panic!("Error: {:?}", error);
    }

    // Then
    assert!(program_binary.is_empty());
    assert_eq!(symbol_table, HashMap::from([(expected_label.symbol.name.clone(), expected_label)]));
}

#[test]
fn process_line_should_not_add_any_binary_when_process_constant() {

    // Given
    let mut address: u16 = 10;
    let line = "RESULT = $000A";
    let mut symbol_table: HashMap<String, Command> = HashMap::new();
    let mut program_binary: Vec<u8> = Vec::new();
    let expected_label = Command::new("RESULT".to_string(), SymbolType::CONSTANT, "$000A".to_string());

    // When
    if let Err(error) = process_line(line, &mut address, &mut program_binary, &mut symbol_table) {
        panic!("Error: {:?}", error);
    }

    // Then
    assert!(program_binary.is_empty());
    assert_eq!(symbol_table, HashMap::from([(expected_label.symbol.name.clone(), expected_label)]));
}

#[test]
fn process_line_address_should_be_updated_when_process_org() {

    // Given
    let mut address: u16 = 10;
    let line = ".org $000B";
    let mut symbol_table: HashMap<String, Command> = HashMap::new();
    let mut program_binary: Vec<u8> = Vec::new();

    // When
    if let Err(error) = process_line(line, &mut address, &mut program_binary, &mut symbol_table) {
        panic!("Error: {:?}", error);
    }

    // Then
    assert!(program_binary.is_empty());
    assert!(symbol_table.is_empty());
    assert_eq!(0x000B, address);
}

#[test]
fn process_line_should_append_binary_to_current_vector() {

    // Given
    let mut address: u16 = 10;
    let line = "ADC #0x0F";
    let mut symbol_table: HashMap<String, Command> = HashMap::new();
    let mut program_binary: Vec<u8> = vec![42, 42, 42];

    // When
    if let Err(error) = process_line(line, &mut address, &mut program_binary, &mut symbol_table) {
        panic!("Error: {:?}", error);
    }

    // Then
    assert_eq!(program_binary, vec![42, 42, 42, 0x69, 0x0F]);
    assert!(symbol_table.is_empty());
    assert_eq!(0x000C, address);
}

#[test]
pub fn test_assembly_sum_1_and_2_program() {

    // Given
    let root_dir = env!("CARGO_MANIFEST_DIR");
    let expected_binary: Vec<u8> = vec![
        0x18, 0xD8, 0xA9, 0x01, 0x8D, 0x00, 0x61, 0xA9, 0x02, 0x8D, 0x01,
        0x61, 0xAD, 0x00, 0x61, 0x6D, 0x01, 0x61, 0x8D, 0x02, 0x61, 0x00];
    let input_filename = format!("{}/resources/test/add.asm", root_dir);
    let output_filename = format!("{}/target/tmp/output.bin", root_dir);

    // When
    assemble(input_filename.as_str(), Some(output_filename.as_str()));

    //Then
    assert_eq!(expected_binary, fs::read(output_filename).unwrap());
}