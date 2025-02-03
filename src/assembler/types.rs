
pub enum NumericType {
    BINARY,
    HEXADECIMAL,
    OCTAL,
    DECIMAL
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