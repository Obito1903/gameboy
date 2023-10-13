struct Instruction {
    opcode: u8,
    name: &'static str,
    size: u8,
    cycles: u8,
    operand1: Option<&'static str>,
    operand2: Option<&'static str>,
}

static INSTRUCTION: &'static [(
    u8,
    &'static str,
    u8,
    u8,
    Option<&'static str>,
    Option<&'static str>,
)] = &[
    (0x00, "NOP", 1, 4, None, None),
    (0x01, "LD", 3, 12, Some("BC"), Some("d16")),
    (0x02, "LD", 1, 8, Some("(BC)"), Some("A")),
    (0x03, "INC", 1, 8, None, None),
];
