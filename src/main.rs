use k_nes::assembler;

fn main() {
    println!("{:X?}", assembler::assemble_instruction("ADC", "$E0").unwrap());
    println!("{:X?}", assembler::assemble_instruction("SBC", "$01,X").unwrap());
    println!("{:X?}", assembler::assemble_instruction("ADC", "($FD,X)").unwrap());
    println!("{:X?}", assembler::assemble_instruction("CMP", "($FF),Y").unwrap());
    println!("{:X?}", assembler::assemble_instruction("ADC", "#$0A").unwrap());
    println!("{:X?}", assembler::assemble_instruction("ADC", "#0x0A").unwrap());
    println!("{:X?}", assembler::assemble_instruction("ADC", "#0x0a").unwrap());
    println!("{:X?}", assembler::assemble_instruction("ADC", "#10").unwrap());
    println!("{:X?}", assembler::assemble_instruction("ADC", "#%00001010").unwrap());
    println!("{:X?}", assembler::assemble_instruction("ADC", "#0b00001010").unwrap());
    println!("{:X?}", assembler::assemble_instruction("ADC", "#@12").unwrap());
    println!("{:X?}", assembler::assemble_instruction("ADC", "#0o12").unwrap());
    println!("{:X?}", assembler::assemble_instruction("JMP", "$0AFD").unwrap());
    println!("{:X?}", assembler::assemble_instruction("LDA", "$0AFD, X").unwrap());
    println!("{:X?}", assembler::assemble_instruction("ADC", "$1B02, Y").unwrap());
    println!("{:X?}", assembler::assemble_instruction("JMP", "($1B02)").unwrap());
}
