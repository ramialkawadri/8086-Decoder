use std::env;
use std::fs::File;
use std::io::prelude::*;

const REGISTER_NAMES: [[&str; 8]; 2] = [
    // Used when W = 0.
    ["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"],
    // Used when W = 1.
    ["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"],
];

const MOVE_INSTRUCTION: u8 = 0b10001000;

// TODO: Homework Move from register to memory and the otherway
// TODO: Homework Move from immediate to register

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let mut file = File::open(args.last().unwrap())?;
    let mut buf = vec![0u8; 2];

    println!("; {}\n", args.last().unwrap());
    println!("bits 16\n");
    while let Ok(_) = file.read_exact(&mut buf) {
        let opcode = buf[0] & 0b11111100;

        if MOVE_INSTRUCTION == opcode {
            let reg = (0b111000 & buf[1]) >> 3;
            let rm = 0b111 & buf[1];
            let w = 0b00000001 & buf[0] as usize;

            let reg_name = REGISTER_NAMES[w][reg as usize];
            let rm_name = REGISTER_NAMES[w][rm as usize];

            if 0b00000010 & buf[0] == 0 {
                println!("mov {}, {}", rm_name, reg_name);
            } else {
                println!("mov {}, {}", reg_name, rm_name);
            }
        }
    }

    Ok(())
}
