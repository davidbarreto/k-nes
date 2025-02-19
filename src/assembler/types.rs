use crate::cpu::types::InstructionError;

#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    SyntaxError(String),
    SymbolAlreadyDefined(String),
    InvalidNumber(String),
    SymbolNotDefined(String),
    InstructionError(InstructionError),
    FatalError(String)
}

impl ParseError {
    pub fn cannot_define_directive_after_other_symbol() -> Self {
        ParseError::SyntaxError("Cannot define a DIRECTIVE after other types of symbols".to_string())
    }

    pub fn cannot_assign_value_to_non_constant(current_symbol_type: SymbolType) -> Self {
        ParseError::SyntaxError(format!("Cannot assign a value to a non-constant symbol. Current symbol type: {:?}", current_symbol_type))
    }

    pub fn constant_not_expected() -> Self {
        ParseError::FatalError("What? Should not have the symbol type CONSTANT here".to_string())
    }

    pub fn mnemonic_expected(symbol: String) -> Self {
        ParseError::SyntaxError(format!("Was expecting a mnemonic, but got {:?}", symbol))
    }
}

pub enum NumericType {
    BINARY,
    HEXADECIMAL,
    OCTAL,
    DECIMAL
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SymbolType {
    LABEL,
    MNEMONIC,
    DATA,
    DIRECTIVE,
    CONSTANT,
    VALUE,
    UNDEFINED
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType
}

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    pub symbol: Symbol,
    pub data: String
}

impl Symbol {
    pub fn new(sname: String, stype: SymbolType) -> Self {
        Self {
            name: sname,
            symbol_type: stype
        }
    }

    pub fn empty() -> Self {
        Self {
            name: String::from(""),
            symbol_type: SymbolType::UNDEFINED
        }
    }
}

impl Command {

    pub fn new(command_name: String, command_type: SymbolType, data: String) -> Self {
        Self {
            symbol: Symbol::new(command_name, command_type),
            data
        }
    }
    
    pub fn from_symbol(command: Symbol, data: String) -> Self {
        Self {
            symbol: command,
            data
        }
    }

    pub fn empty() -> Self {
        Self {
            symbol: Symbol::empty(),
            data: String::from(""),
        }
    }
}

impl NumericType {

    /// Return the radix of the specified numeric type
    pub fn to_radix(&self) -> u32 {
        match self {
            NumericType::BINARY => 2,
            NumericType::OCTAL => 8,
            NumericType::DECIMAL => 10,
            NumericType::HEXADECIMAL => 16
        }
    }

    /// Given a numeric string, check it's first one or two chars to
    /// detect what is its numeric base.
    pub fn detect_type_in_string(numeric_string: &str) -> (Self, &str) {

        if numeric_string.starts_with("$") {
            (NumericType::HEXADECIMAL, &numeric_string[1..])
        } else if numeric_string.starts_with("0x") {
            (NumericType::HEXADECIMAL, &numeric_string[2..])
        } else if numeric_string.starts_with("%") {
            (NumericType::BINARY, &numeric_string[1..])
        } else if numeric_string.starts_with("0b") {
            (NumericType::BINARY, &numeric_string[2..])
        } else if numeric_string.starts_with("@") {
            (NumericType::OCTAL, &numeric_string[1..])
        } else if numeric_string.starts_with("0o") {
            (NumericType::OCTAL, &numeric_string[2..])
        } else {
            (NumericType::DECIMAL, numeric_string)
        }
    }
}