mod constants;
mod flag;
mod rm;
mod simulator;

use std::fs::File;
use std::io::prelude::*;
use std::{env, io::SeekFrom};

use constants::{
    ACCUMULATOR_NAMES, IMMEDIATE_TO_ACCUMULATOR_INSTRUCTIONS,
    IMMEDIATE_TO_REGISTER_MEMORY_INSTRUCTION, IMMEDIATE_TO_REGISTER_MEMORY_INSTRUCTION_MOV,
    MOVE_IMMEDIATE_TO_REGISTER_INSTRUCTION, REGISTER_NAMES, RETURN_INSTRUCTIONS,
};
use flag::Flags;
use rm::Rm;
use simulator::immediate_to_rm_simulator::MovImmediateToRMSimulator;
use simulator::{SimulatorInput, SimulatorOutput};
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

    let mut simulation_registers = [0; 8];
    let mut memory = [0u8; 65536];
    let mut flags = Flags {
        zf: false,
        sf: false,
    };
    let mut current_clock = 0i16;

    let mut old_ip = 0;
    while let Ok(_) = file.read_exact(&mut current_byte) {
        let current_byte = current_byte[0];
        let old_flags = flags.clone();
        let mut old_value = 0;
        let mut new_value = 0;
        let mut prefix = "";
        let mut instruction_name = "";
        let mut destination: Option<Rm> = None;
        let mut source_rm: Option<Rm> = None;
        let mut immediate_value: Option<i16> = None;
        let mut default_print = true;
        let mut number_of_cycles = 0i16;

        if MOVE_IMMEDIATE_TO_REGISTER_INSTRUCTION == current_byte & 0b11110000 {
            let w = ((0b1000 & current_byte) >> 3) as usize;
            let reg = 0b111 & current_byte as usize;
            let data = read_date(&mut file, w == 0);
            number_of_cycles = 4;

            instruction_name = "mov";
            destination = Some(Rm::Reg { w, reg });

            if w == 1 && simulation_mode {
                old_value = simulation_registers[reg];
                simulation_registers[reg] = data;
                new_value = simulation_registers[reg];
                immediate_value = Some(data);
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
            prefix = if mod_value == 0b11 {
                ""
            } else {
                if one_byte { "byte " } else { "word " }
            };

            let operation_index = ((next_byte & 0b111000) >> 3) as usize;

            instruction_name = immediate_to_register_instructions[operation_index].0;
            destination = Some(rm.clone());
            immediate_value = Some(data);

            if simulation_mode {
                SimulatorOutput {
                    old_value,
                    new_value,
                    number_of_cycles,
                } = immediate_to_register_instructions[operation_index]
                    .1
                    .simulate(SimulatorInput {
                        simulation_registers: &mut simulation_registers,
                        memory: &mut memory,
                        flags: &mut flags,
                        destination: &rm,
                        immediate_value: Some(data),
                        source: None,
                    });
            }
        } else if IMMEDIATE_TO_REGISTER_MEMORY_INSTRUCTION_MOV == current_byte & 0b11111110 {
            let mut next_byte = [0u8];
            file.read_exact(&mut next_byte).unwrap();
            let next_byte = next_byte[0];

            let mod_value = (0b11000000 & next_byte) >> 6;
            let w = (current_byte & 0b1) as usize;
            let rm = Rm::new(&mut file, mod_value, w, (0b111 & next_byte) as usize);
            let data = read_date(&mut file, w != 1);
            prefix = if mod_value == 0b11 {
                ""
            } else {
                if w != 1 { "byte " } else { "word " }
            };
            instruction_name = "mov";
            destination = Some(rm.clone());
            immediate_value = Some(data);

            if simulation_mode {
                SimulatorOutput {
                    old_value,
                    new_value,
                    number_of_cycles,
                } = MovImmediateToRMSimulator.simulate(SimulatorInput {
                    simulation_registers: &mut simulation_registers,
                    memory: &mut memory,
                    flags: &mut flags,
                    destination: &rm,
                    immediate_value: Some(data),
                    source: None,
                });
            } else {
                print!("{} {}{}, {}", "mov", prefix, rm, data);
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

            let source;
            let dst;
            if d == 0 {
                source = reg;
                dst = rm;
            } else {
                source = rm;
                dst = reg;
            }

            instruction_name = instruction.1;
            destination = Some(dst.clone());
            source_rm = Some(source.clone());

            if simulation_mode {
                SimulatorOutput {
                    old_value,
                    new_value,
                    number_of_cycles,
                } = instruction.2.simulate(SimulatorInput {
                    simulation_registers: &mut simulation_registers,
                    memory: &mut memory,
                    flags: &mut flags,
                    source: Some(&source),
                    destination: &dst,
                    immediate_value: None,
                });
            }
        } else if let Some(instruction) = IMMEDIATE_TO_ACCUMULATOR_INSTRUCTIONS
            .iter()
            .find(|i| i.0 == current_byte & 0b11111110)
        {
            let w = 0b1 & current_byte;
            let data = read_date(&mut file, w == 0);
            print!(
                "{} {}, {}",
                instruction.1, ACCUMULATOR_NAMES[w as usize], data
            );
        } else if let Some(instruction) = RETURN_INSTRUCTIONS
            .iter()
            .find(|i| i.0 == current_byte & 0b11111111)
        {
            let data = read_date(&mut file, true);

            if simulation_mode {
                if instruction.1 == "jne" && !flags.zf {
                    file.seek(SeekFrom::Current(data as i64))
                        .expect("Seek error");
                }
            }
            print!("{} ; {}", instruction.1, data);
            default_print = false;
        }

        let new_ip = file.stream_position().unwrap();

        if default_print {
            let source = if let Some(immediate_value) = immediate_value {
                immediate_value.to_string()
            } else {
                source_rm.unwrap().to_string()
            };
            if simulation_mode {
                current_clock += number_of_cycles;
                print!(
                    "{} {}{}, {} ; {}:{:#06x}->{:#06x} ; flags:{}->{}; Clocks: +{} = {}",
                    instruction_name,
                    prefix,
                    destination.clone().expect("No destination filled"),
                    source,
                    destination.unwrap(),
                    old_value,
                    new_value,
                    old_flags,
                    flags,
                    number_of_cycles,
                    current_clock
                );
            } else {
                print!(
                    "{} {}{}, {}",
                    instruction_name,
                    prefix,
                    destination.clone().expect("No destination filled"),
                    source
                );
            }
        }
        println!("; ip:{:#04x}->{:#04x}", old_ip, new_ip);
        old_ip = new_ip;
    }

    if simulation_mode {
        println!("\nFinal registers:");
        for i in 0..(simulation_registers.len()) {
            if simulation_registers[i] == 0 {
                continue;
            }
            println!(
                "\t{}: {:#06x} ({})",
                REGISTER_NAMES[1][i], simulation_registers[i], simulation_registers[i],
            );
        }
        println!(
            "\tip: {:#06x} ({})",
            file.stream_position().unwrap(),
            file.stream_position().unwrap()
        );
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
