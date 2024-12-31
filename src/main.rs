use std::env;
use std::fs::File;
use std::io::prelude::*;

const REGISTER_NAMES: [[&str; 8]; 2] = [
    // Used when W = 0.
    ["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"],
    // Used when W = 1.
    ["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"],
];

const MEMORY_ADDRESS: [&str; 8] = [
    "bx + si", "bx + di", "bp + si", "bp + di", "si", "di", "bp", "bx",
];

const ACCUMULATOR_NAMES: [&str; 2] = ["al", "ax"];

const MOVE_INSTRUCTION: u8 = 0b10001000;
const MOVE_IMMEDIATE_TO_REGISTER_INSTRUCTION: u8 = 0b10110000;

const ADD_INSTRUCTION: u8 = 0b00000000;
const SUB_INSTRUCTION: u8 = 0b00101000;
const CMP_INSTRUCTION: u8 = 0b00111000;

const IMMEDIATE_TO_REGISTER_MEMORY_INSTRUCTION: u8 = 0b10000000;

const IMMEDIATE_TO_ACCUMULATOR_OPERATIONS: [(u8, &str); 3] =
    [(0b00000100, "add"), (0b00101100, "sub"), (0b00111100, "cmp")];

// TODO: JNZ: jump if not zero [01110101][displacment] (page 168)

const IMMEDIATE_TO_REGISTER_OPERATIONS: [&str; 8] = ["add", "1", "2", "3", "4", "sub", "6", "cmp"];

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let mut file = File::open(args.last().unwrap())?;
    let mut current_byte = [0u8];

    println!("; {}\n", args.last().unwrap());
    println!("bits 16\n");

    // while let Ok(_) = file.read_exact(&mut current_byte) {
    //     print!("{:#010b} ", current_byte[0]);
    // }

    while let Ok(_) = file.read_exact(&mut current_byte) {
        let current_byte = current_byte[0];

        if MOVE_IMMEDIATE_TO_REGISTER_INSTRUCTION == current_byte & 0b11110000 {
            let w = ((0b1000 & current_byte) >> 3) as usize;
            let reg = 0b111 & current_byte as usize;
            let data = read_date_based_on_w(&mut file, w == 0);
            println!("mov {}, {}", REGISTER_NAMES[w][reg], data);
        } else if MOVE_INSTRUCTION == current_byte & 0b11111100 {
            let (source, destination) = decode_rm_to_rm(&mut file, current_byte);
            println!("mov {}, {}", destination, source);
        } else if ADD_INSTRUCTION == current_byte & 0b11111100 {
            let (source, destination) = decode_rm_to_rm(&mut file, current_byte);
            println!("add {}, {}", destination, source);
        } else if SUB_INSTRUCTION == current_byte & 0b11111100 {
            let (source, destination) = decode_rm_to_rm(&mut file, current_byte);
            println!("sub {}, {}", destination, source);
        } else if CMP_INSTRUCTION == current_byte & 0b11111100 {
            let (source, destination) = decode_rm_to_rm(&mut file, current_byte);
            println!("cmp {}, {}", destination, source);
        } else if IMMEDIATE_TO_REGISTER_MEMORY_INSTRUCTION == current_byte & 0b11111100 {
            let (rm_name, prefix, next_byte, data) = decode_immediate_to_register(
                &mut file,
                current_byte,
                (current_byte & 0b11) != 0b01,
            );
            let operation_index = ((next_byte & 0b111000) >> 3) as usize;
            println!(
                "{} {}{}, {}",
                IMMEDIATE_TO_REGISTER_OPERATIONS[operation_index], prefix, rm_name, data
            );
        } else {
            // Immediate to accumulator.
            let operation = IMMEDIATE_TO_ACCUMULATOR_OPERATIONS
                .iter()
                .find(|i| i.0 == current_byte & 0b11111110);
            let operation = match operation {
                Some(operation) => operation,
                None => continue,
            };
            let w = 0b1 & current_byte;
            let data = read_date_based_on_w(&mut file, w == 0);
            println!("{} {}, {}", operation.1, ACCUMULATOR_NAMES[w as usize], data);
        }
    }

    Ok(())
}

/// Works on data where the next byte is of format **mod reg r/m**
fn decode_rm_to_rm(file: &mut File, current_byte: u8) -> (String, String) {
    let mut next_byte = [0u8];
    file.read_exact(&mut next_byte).unwrap();
    let next_byte = next_byte[0];

    let mod_value = (0b11000000 & next_byte) >> 6;
    let reg = ((0b111000 & next_byte) >> 3) as usize;
    let rm = 0b111 & next_byte as usize;

    let w = 0b00000001 & current_byte as usize;
    let d = (0b00000010 & current_byte) >> 1;

    let reg_name = String::from(REGISTER_NAMES[w][reg]);
    let rm_name = read_rm(file, mod_value, w, rm);

    if d == 0 {
        return (reg_name, rm_name);
    } else {
        return (rm_name, reg_name);
    }
}

/// Decode instruction for the type of immediate to register/memory, and it
/// returns the rm_name, the prefix (empty string, byte or word), the next byte and the data.
fn decode_immediate_to_register(
    file: &mut File,
    current_byte: u8,
    one_byte: bool,
) -> (String, &str, u8, i16) {
    let mut next_byte = [0u8];
    file.read_exact(&mut next_byte).unwrap();
    let next_byte = next_byte[0];
    let mod_value = (0b11000000 & next_byte) >> 6;
    let rm = (0b111 & next_byte) as usize;
    let rm_name = read_rm(file, mod_value, (0b1 & current_byte) as usize, rm);
    let data = read_date_based_on_w(file, one_byte);
    return (
        rm_name,
        if mod_value == 0b11 {
            ""
        } else {
            if one_byte { "byte " } else { "word " }
        },
        next_byte,
        data,
    );
}

fn read_rm(file: &mut File, mod_value: u8, w: usize, rm: usize) -> String {
    if mod_value == 0b00 {
        // Memory mode no displacment
        if rm == 0b110 {
            let mut displacment = [0u8, 0u8];
            file.read_exact(&mut displacment).unwrap();
            return format!(
                "[{}]",
                ((displacment[1] as u16) << 8) | displacment[0] as u16
            );
        } else {
            return format!("[{}]", MEMORY_ADDRESS[rm]);
        }
    } else if mod_value == 0b01 {
        // Memory mode, 8-bit displacment
        let mut displacment = [0u8];
        file.read_exact(&mut displacment).unwrap();
        return format!("[{} + {}]", MEMORY_ADDRESS[rm], displacment[0]);
    } else if mod_value == 0b10 {
        // Memory mode, 16-bit displacment
        let mut displacment = [0u8, 0u8];
        file.read_exact(&mut displacment).unwrap();
        return format!(
            "[{} + {}]",
            MEMORY_ADDRESS[rm],
            ((displacment[1] as u16) << 8) | displacment[0] as u16
        );
    } else {
        return String::from(REGISTER_NAMES[w][rm]);
    }
}

fn read_date_based_on_w(file: &mut File, one_byte: bool) -> i16 {
    if one_byte {
        let mut data = [0u8];
        file.read_exact(&mut data).unwrap();
        return (data[0] as i8) as i16;
    } else {
        let mut data = [0u8, 0u8];
        file.read_exact(&mut data).unwrap();
        return ((data[1] as i16) << 8) | data[0] as i16;
    }
}
