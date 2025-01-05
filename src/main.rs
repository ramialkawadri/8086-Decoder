mod constants;
mod flag;
mod rm;
mod simulator;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use constants::{
    ACCUMULATOR_NAMES, IMMEDIATE_TO_ACCUMULATOR_INSTRUCTIONS, IMMEDIATE_TO_REGISTER_MEMORY_INSTRUCTION, MOVE_IMMEDIATE_TO_REGISTER_INSTRUCTION, REGISTER_NAMES, RETURN_INSTRUCTIONS
};
use flag::Flags;
use rm::Rm;
use simulator::{
    immediate_to_rm_simulator::{
        AddImmediateToRMSimulator, CmpImmediateToRMSimulator, ImmediateToRMSimulator,
        SubImmediateToRMSimulator,
    },
    rm_to_rm_simulator::{
        AddRmToRmSimulator, CmpRmToRmSimulator, MovRmToRmSimulator, RMToRmSimulator,
        SubRmToRmSimulator,
    },
};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2);

    let simulation_mode = args[1] == "--exec";
    let mut file = File::open(args.last().unwrap())?;
    let mut current_byte = [0u8];

    let rm_to_rm_instructions: [(u8, &str, Box<dyn RMToRmSimulator>); 4] = [
        (0b10001000, "mov", Box::new(MovRmToRmSimulator)),
        (0b00000000, "add", Box::new(AddRmToRmSimulator)),
        (0b00101000, "sub", Box::new(SubRmToRmSimulator)),
        (0b00111000, "cmp", Box::new(CmpRmToRmSimulator)),
    ];

    let immediate_to_register_instructions: [(&str, Box<dyn ImmediateToRMSimulator>); 8] = [
        ("add", Box::new(AddImmediateToRMSimulator)),
        ("1", Box::new(AddImmediateToRMSimulator)),
        ("2", Box::new(AddImmediateToRMSimulator)),
        ("3", Box::new(AddImmediateToRMSimulator)),
        ("4", Box::new(AddImmediateToRMSimulator)),
        ("sub", Box::new(SubImmediateToRMSimulator)),
        ("6", Box::new(AddImmediateToRMSimulator)),
        ("cmp", Box::new(CmpImmediateToRMSimulator)),
    ];

    println!("; {}\n", args.last().unwrap());
    println!("bits 16\n");

    if args[1] == "--print-binary" {
        while let Ok(_) = file.read_exact(&mut current_byte) {
            print!("{:#010b} ", current_byte[0]);
        }
    }

    let mut simulation_registers = [0, 0, 0, 0, 0, 0, 0, 0];
    let mut flags = Flags {
        zf: false,
        sf: false,
    };

    while let Ok(_) = file.read_exact(&mut current_byte) {
        let current_byte = current_byte[0];

        if MOVE_IMMEDIATE_TO_REGISTER_INSTRUCTION == current_byte & 0b11110000 {
            let w = ((0b1000 & current_byte) >> 3) as usize;
            let reg = 0b111 & current_byte as usize;
            let data = read_date(&mut file, w == 0);

            if w == 1 && simulation_mode {
                let old_value = simulation_registers[reg];
                simulation_registers[reg] = data;
                println!(
                    "mov {}, {} ; {}:{:#06x}->{:#06x}",
                    REGISTER_NAMES[w][reg],
                    data,
                    REGISTER_NAMES[w][reg],
                    old_value,
                    simulation_registers[reg]
                );
            } else {
                println!("mov {}, {}", REGISTER_NAMES[w][reg], data);
            }
        } else if IMMEDIATE_TO_REGISTER_MEMORY_INSTRUCTION == current_byte & 0b11111100 {
            let mut next_byte = [0u8];
            file.read_exact(&mut next_byte).unwrap();
            let next_byte = next_byte[0];

            let mod_value = (0b11000000 & next_byte) >> 6;
            let one_byte = (current_byte & 0b11) != 0b01;
            let rm = Rm::new(
                &mut file,
                mod_value,
                (0b1 & current_byte) as usize,
                (0b111 & next_byte) as usize,
            );
            let data = read_date(&mut file, one_byte);
            let prefix = if mod_value == 0b11 {
                ""
            } else {
                if one_byte { "byte " } else { "word " }
            };

            let operation_index = ((next_byte & 0b111000) >> 3) as usize;

            if simulation_mode {
                let old_flags = flags.clone();
                if let Rm::Reg { reg, .. } = rm {
                    let old_value = simulation_registers[reg];
                    immediate_to_register_instructions[operation_index]
                        .1
                        .simulate(&mut simulation_registers, &mut flags, &rm, data);
                    println!(
                        "{} {}, {} ; {}:{:#06x}->{:#06x} ; flags:{}->{}",
                        immediate_to_register_instructions[operation_index].0,
                        rm,
                        data,
                        rm,
                        old_value,
                        simulation_registers[reg],
                        old_flags,
                        flags
                    );
                }
            } else {
                println!(
                    "{} {}{}, {}",
                    immediate_to_register_instructions[operation_index].0, prefix, rm, data
                );
            }
        } else if let Some(instruction) = rm_to_rm_instructions
            .iter()
            .find(|i| i.0 == current_byte & 0b11111100)
        {
            let mut next_byte = [0u8];
            file.read_exact(&mut next_byte).unwrap();
            let next_byte = next_byte[0];

            let w = 0b00000001 & current_byte as usize;
            let d = (0b00000010 & current_byte) >> 1;

            let mod_value = (0b11000000 & next_byte) >> 6;
            let reg = Rm::Reg {
                w,
                reg: ((0b111000 & next_byte) >> 3) as usize,
            };
            let rm = Rm::new(&mut file, mod_value, w, 0b111 & next_byte as usize);

            let mut old_value = 0;
            let old_flags = flags.clone();

            let source;
            let destination;
            if d == 0 {
                source = reg;
                destination = rm;
            } else {
                source = rm;
                destination = reg;
            }

            if simulation_mode {
                if let Rm::Reg { reg, .. } = destination {
                    old_value = simulation_registers[reg];
                }
                instruction.2.simulate(
                    &mut simulation_registers,
                    &mut flags,
                    &source,
                    &destination,
                );
            }

            if w == 1 && simulation_mode {
                if let Rm::Reg { reg, w } = destination {
                    println!(
                        "{} {}, {} ; {}:{:#06x}->{:#06x} ; flags:{}->{}",
                        instruction.1,
                        destination,
                        source,
                        REGISTER_NAMES[w][reg],
                        old_value,
                        simulation_registers[reg],
                        old_flags,
                        flags
                    );
                }
            } else {
                println!("{} {}, {}", instruction.1, destination, source);
            }
        } else if let Some(instruction) = IMMEDIATE_TO_ACCUMULATOR_INSTRUCTIONS
            .iter()
            .find(|i| i.0 == current_byte & 0b11111110)
        {
            let w = 0b1 & current_byte;
            let data = read_date(&mut file, w == 0);
            println!(
                "{} {}, {}",
                instruction.1, ACCUMULATOR_NAMES[w as usize], data
            );
        } else if let Some(instruction) = RETURN_INSTRUCTIONS
            .iter()
            .find(|i| i.0 == current_byte & 0b11111111)
        {
            let data = read_date(&mut file, true);
            println!("{} ; {}", instruction.1, data);
        }
    }

    if simulation_mode {
        println!("\nFinal registers:");
        for i in 0..(simulation_registers.len()) {
            println!(
                "\t{}: {:#06x} ({})",
                REGISTER_NAMES[1][i], simulation_registers[i], simulation_registers[i],
            );
        }
        println!("\tflags: {}", flags);
    }

    Ok(())
}

fn read_date(file: &mut File, one_byte: bool) -> i16 {
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
