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

const MOVE_INSTRUCTION: u8 = 0b10001000;
const IMMEDIATE_TO_REGISTER: u8 = 0b10110000;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let mut file = File::open(args.last().unwrap())?;
    let mut current_byte = [0u8];

    println!("; {}\n", args.last().unwrap());
    println!("bits 16\n");
    while let Ok(_) = file.read_exact(&mut current_byte) {
        let current_byte = current_byte[0];

        if IMMEDIATE_TO_REGISTER == current_byte & 0b11110000 {
            let w = ((0b1000 & current_byte) >> 3) as usize;
            let reg = 0b111 & current_byte as usize;
            if w == 0 {
                let mut data = [0u8];
                file.read_exact(&mut data).unwrap();
                println!("mov {}, {}", REGISTER_NAMES[w][reg], data[0] as i8);
            } else {
                let mut data = [0u8, 0u8];
                file.read_exact(&mut data).unwrap();
                println!(
                    "mov {}, {}",
                    REGISTER_NAMES[w][reg],
                    ((data[1] as i16) << 8) | data[0] as i16
                );
            }
        } else if MOVE_INSTRUCTION == current_byte & 0b11111100 {
            let mut next_byte = [0u8];
            file.read_exact(&mut next_byte).unwrap();
            let next_byte = next_byte[0];

            let mod_value = (0b11000000 & next_byte) >> 6;
            let reg = ((0b111000 & next_byte) >> 3) as usize;
            let rm = 0b111 & next_byte as usize;
            let w = 0b00000001 & current_byte as usize;
            let d = (0b00000010 & current_byte) >> 1;

            let reg_name = String::from(REGISTER_NAMES[w][reg]);
            let rm_name = String::from(REGISTER_NAMES[w][rm]);

            let mut source;
            let mut destination;

            if mod_value == 0b00 {
                // Memory mode no displacment
                if rm == 0b110 {
                    let mut displacment = [0u8, 0u8];
                    file.read_exact(&mut displacment).unwrap();
                    if d == 0 {
                        source = reg_name;
                        destination = format!(
                            "[{}]",
                            ((displacment[1] as i16) << 8) | displacment[0] as i16
                        );
                    } else {
                        source = format!(
                            "[{}]",
                            ((displacment[1] as i16) << 8) | displacment[0] as i16
                        );
                        destination = reg_name;
                    }
                } else {
                    if d == 0 {
                        source = reg_name;
                        destination = format!("[{}]", MEMORY_ADDRESS[rm]);
                    } else {
                        source = format!("[{}]", MEMORY_ADDRESS[rm]);
                        destination = reg_name;
                    }
                }
            } else if mod_value == 0b01 {
                // Memory mode, 8-bit displacment
                let mut displacment = [0u8];
                file.read_exact(&mut displacment).unwrap();
                if d == 0 {
                    source = reg_name;
                    destination = format!("[{} + {}]", MEMORY_ADDRESS[rm], displacment[0]);
                } else {
                    source = format!("[{} + {}]", MEMORY_ADDRESS[rm], displacment[0]);
                    destination = reg_name;
                }
            } else if mod_value == 0b10 {
                // Memory mode, 16-bit displacment
                let mut displacment = [0u8, 0u8];
                file.read_exact(&mut displacment).unwrap();
                if d == 0 {
                    source = reg_name;
                    destination = format!(
                        "[{} + {}]",
                        MEMORY_ADDRESS[rm],
                        ((displacment[1] as i16) << 8) | displacment[0] as i16
                    );
                } else {
                    source = format!(
                        "[{} + {}]",
                        MEMORY_ADDRESS[rm],
                        ((displacment[1] as i16) << 8) | displacment[0] as i16
                    );
                    destination = reg_name;
                }
            } else {
                // Register mode
                if d == 0 {
                    source = reg_name;
                    destination = rm_name;
                } else {
                    source = rm_name;
                    destination = reg_name;
                }
            }

            source = String::from(source.replace(" + 0", ""));
            destination = String::from(destination.replace(" + 0", ""));
            println!("mov {}, {}", destination, source);
        }
    }

    Ok(())
}
