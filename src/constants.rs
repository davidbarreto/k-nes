use lazy_static::lazy_static;

pub const DEFAULT_OUTPUT_FILENAME: &str = "output.bin";

const HEX_8_BIT: &str = r"(\$|0x)[0-9A-Fa-f]{2}";
const BIN_8_BIT: &str = r"(%|0b)[01]{8}";
const OCTAL_8_BIT:&str = r"(@|0o)[0-7]{1,3}";
const DEC_8_BIT: &str = r"25[0-5]|2[0-4][0-9]|1?[0-9]{1,2}";

const HEX_16_BIT: &str = r"(\$|0x)[0-9A-Fa-f]{4}";
const BIN_16_BIT: &str = r"(%|0b)[01]{16}";
const OCTAL_16_BIT: &str = r"(@|0o)[0-7]{1,5}";
const DEC_16_BIT: &str = r"6553[0-5]|655[0-2][0-9]|65[0-4][0-9]{2}|[0-5]?[0-9]{1,4}";

lazy_static! {
    pub static ref NUM_8_BIT: String = format!("(?P<number>{}|{}|{}|{})", HEX_8_BIT, BIN_8_BIT, OCTAL_8_BIT, DEC_8_BIT);
    pub static ref NUM_16_BIT: String = format!("(?P<number>{}|{}|{}|{})", HEX_16_BIT, BIN_16_BIT, OCTAL_16_BIT, DEC_16_BIT);
    pub static ref NUM_UP_TO_16_BIT: String = format!("(?P<number>{}|{}|{}|{}|{}|{}|{}|{})", HEX_8_BIT, HEX_16_BIT, BIN_8_BIT, BIN_16_BIT, OCTAL_8_BIT, OCTAL_16_BIT, DEC_8_BIT, DEC_16_BIT);
}

