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
pub fn tmp_test() {

    let mut lines: Vec<&str> = Vec::new();

    lines.push("  .org $8000");
    lines.push("       RESULT = 0x0F");
    lines.push("START: LDA #RESULT ; Load a number");
    lines.push("   JMP START ; Test Jump");
    lines.push("; Here is a comment without any command on it");
    lines.push("BRK");

    let mut address: u16 = 0;
    let mut symbol_table: HashMap<String, Command> = HashMap::new();
    let mut program_binary: Vec<u8> = Vec::with_capacity(500);
    let mut line_number: usize = 1;

    for line in lines {
        if let Err(parse_error) = process_line(line, &mut address, &mut program_binary, &mut symbol_table) {
            panic!("Error in line {}: {:?}", line_number, parse_error);
        }
        line_number += 1;
    }

    assert_eq!(program_binary, vec![0xA9, 0x0F, 0x4C, 0x00, 0x80, 0x00]);
}