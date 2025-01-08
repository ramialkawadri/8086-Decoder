use crate::rm::Rm;

use super::{SimulatorInput, SimulatorOutput};

pub trait ImmediateToRMSimulator {
    fn simulate(&self, input: SimulatorInput) -> SimulatorOutput;
}

pub struct MovImmediateToRMSimulator;

impl ImmediateToRMSimulator for MovImmediateToRMSimulator {
    fn simulate(&self, input: SimulatorInput) -> SimulatorOutput {
        let SimulatorInput {
            destination,
            simulation_registers,
            memory,
            flags,
            immediate_value,
            ..
        } = input;
        let mut output = SimulatorOutput::default();

        match destination {
            Rm::Reg {
                reg: destination, ..
            } => {
                output.old_value = simulation_registers[*destination as usize];
                simulation_registers[*destination as usize] += immediate_value.unwrap();
            }
            Rm::MemoryWithDisplacment {
                rm: register_index,
                displacment,
            } => {
                let memory_index =
                    (simulation_registers[*register_index] + *displacment as i16) as usize;
                output.old_value = memory[memory_index] as i16;
                memory[memory_index] = (immediate_value.unwrap() & 0b11111111) as u8;
            }
            Rm::MemoryNoDisplacment(index) => {
                output.old_value = memory[*index as usize] as i16;
                memory[*index as usize] = (immediate_value.unwrap() & 0b11111111) as u8;
            }
            Rm::DirectMemory(index) => {
                output.old_value = memory[*index as usize] as i16;
                memory[*index as usize] = (immediate_value.unwrap() & 0b11111111) as u8;
            }
        }

        flags.update_from_number(immediate_value.unwrap());
        output.new_value = immediate_value.unwrap();
        output
    }
}

pub struct AddImmediateToRMSimulator;

impl ImmediateToRMSimulator for AddImmediateToRMSimulator {
    fn simulate(&self, input: SimulatorInput) -> SimulatorOutput {
        let SimulatorInput {
            destination,
            simulation_registers,
            flags,
            immediate_value,
            ..
        } = input;
        let mut output = SimulatorOutput::default();

        if let Rm::Reg {
            reg: destination, ..
        } = destination
        {
            output.old_value = simulation_registers[*destination as usize];
            simulation_registers[*destination as usize] += immediate_value.unwrap();
            output.new_value = simulation_registers[*destination as usize];
        }

        flags.update_from_number(immediate_value.unwrap());
        output
    }
}

pub struct SubImmediateToRMSimulator;

impl ImmediateToRMSimulator for SubImmediateToRMSimulator {
    fn simulate(&self, input: SimulatorInput) -> SimulatorOutput {
        let SimulatorInput {
            destination,
            simulation_registers,
            flags,
            immediate_value,
            ..
        } = input;
        let mut output = SimulatorOutput::default();

        if let Rm::Reg {
            reg: destination, ..
        } = destination
        {
            output.old_value = simulation_registers[*destination as usize];
            simulation_registers[*destination as usize] -= immediate_value.unwrap();
            output.new_value = simulation_registers[*destination as usize];
        }

        flags.update_from_number(immediate_value.unwrap());
        output
    }
}

pub struct CmpImmediateToRMSimulator;

impl ImmediateToRMSimulator for CmpImmediateToRMSimulator {
    fn simulate(&self, input: SimulatorInput) -> SimulatorOutput {
        let SimulatorInput {
            destination,
            simulation_registers,
            flags,
            immediate_value,
            ..
        } = input;
        let mut output = SimulatorOutput::default();

        if let Rm::Reg {
            reg: destination, ..
        } = destination
        {
            flags.update_from_number(
                simulation_registers[*destination as usize] - immediate_value.unwrap(),
            );
        }

        flags.update_from_number(immediate_value.unwrap());
        output.old_value = immediate_value.unwrap();
        output.new_value = immediate_value.unwrap();
        output
    }
}
