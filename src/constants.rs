pub const REGISTER_NAMES: [[&str; 8]; 2] = [
    // Used when W = 0.
    ["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"],
    // Used when W = 1.
    ["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"],
];

pub const EFFECTIVE_MEMOERY_ADDRESS: [&str; 8] = [
    "bx + si", "bx + di", "bp + si", "bp + di", "si", "di", "bp", "bx",
];

pub const ACCUMULATOR_NAMES: [&str; 2] = ["al", "ax"];

pub const MOVE_IMMEDIATE_TO_REGISTER_INSTRUCTION: u8 = 0b10110000;

pub const IMMEDIATE_TO_REGISTER_MEMORY_INSTRUCTION: u8 = 0b10000000;
pub const IMMEDIATE_TO_REGISTER_MEMORY_INSTRUCTION_MOV: u8 = 0b11000110;

pub const IMMEDIATE_TO_ACCUMULATOR_INSTRUCTIONS: [(u8, &str); 3] = [
    (0b00000100, "add"),
    (0b00101100, "sub"),
    (0b00111100, "cmp"),
];

pub const RETURN_INSTRUCTIONS: [(u8, &str); 20] = [
    (0b01110100, "je"),
    (0b01111100, "jl"),
    (0b01111110, "jle"),
    (0b01110010, "jb"),
    (0b01110110, "jbe"),
    (0b01111010, "jp"),
    (0b01110000, "jo"),
    (0b01111000, "js"),
    (0b01110101, "jne"),
    (0b01111101, "jnl"),
    (0b01111111, "jg"),
    (0b01110011, "jnb"),
    (0b01110111, "ja"),
    (0b01111011, "jnp"),
    (0b01110001, "jno"),
    (0b01111001, "jns"),
    (0b11100010, "loop"),
    (0b11100001, "loopz"),
    (0b11100000, "loopnz"),
    (0b11100011, "jcxz"),
];
