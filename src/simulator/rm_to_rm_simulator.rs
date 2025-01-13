use crate::rm::Rm;

use super::{SimulatorInput, SimulatorOutput};

pub trait RMToRmSimulator {
    fn simulate(&self, input: SimulatorInput) -> SimulatorOutput;
}

pub struct MovRmToRmSimulator;

impl RMToRmSimulator for MovRmToRmSimulator {
    fn simulate(&self, input: SimulatorInput) -> SimulatorOutput {
        let SimulatorInput {
            destination,
            simulation_registers,
            source,
            memory,
            flags,
            ..
        } = input;
        let mut output = SimulatorOutput::default();
        let source = source.unwrap();

        if let Rm::Reg {
            reg: destination, ..
        } = destination
        {
            output.old_value = simulation_registers[*destination as usize];
            if let Rm::Reg { reg: source, .. } = source {
                simulation_registers[*destination as usize] =
                    simulation_registers[*source as usize];
                output.number_of_cycles = 2;
            } else if let Rm::DirectMemory(index) = source {
                simulation_registers[*destination as usize] = memory[*index as usize] as i16;
                output.number_of_cycles = 8 + source.estimate_cycles();
            } else if let Rm::MemoryNoDisplacment(_) = source {
                let memory_index = source.calculate_memory_index(simulation_registers);
                simulation_registers[*destination as usize] = memory[memory_index as usize] as i16;
                output.number_of_cycles = 8 + source.estimate_cycles();
            } else if let Rm::MemoryWithDisplacment { rm: register_index, displacment } = source {
                let memory_index =
                    (simulation_registers[*register_index] + *displacment as i16) as usize;
                simulation_registers[*destination as usize] = memory[memory_index] as i16;
                output.number_of_cycles = 8 + source.estimate_cycles(); 
            }
            output.new_value = simulation_registers[*destination as usize];
            flags.update_from_number(simulation_registers[*destination as usize]);
        } else if let Rm::MemoryNoDisplacment(_) = destination {
            let memory_index = destination.calculate_memory_index(simulation_registers) as usize;
            output.old_value = memory[memory_index] as i16;
            if let Rm::Reg { reg: source, .. } = source {
                memory[memory_index] = simulation_registers[*source as usize] as u8;
            }
            output.number_of_cycles = 9 + destination.estimate_cycles(); 
            output.old_value = memory[memory_index] as i16;
        } else if let Rm::MemoryWithDisplacment { rm: register_index, displacment } = destination {
            let memory_index =
                (simulation_registers[*register_index] + *displacment as i16) as usize;
            output.old_value = memory[memory_index] as i16;

            if let Rm::Reg { reg: source, .. } = source {
                memory[memory_index] = simulation_registers[*source as usize] as u8;
                output.new_value = simulation_registers[*source as usize];
                output.number_of_cycles = 9 + destination.estimate_cycles();
            }
        }

        output
    }
}

pub struct AddRmToRmSimulator;

impl RMToRmSimulator for AddRmToRmSimulator {
    fn simulate(&self, input: SimulatorInput) -> SimulatorOutput {
        let SimulatorInput {
            destination,
            simulation_registers,
            source,
            flags,
            memory,
            ..
        } = input;
        let mut output = SimulatorOutput::default();
        let source = source.unwrap();

        if let Rm::Reg {
            reg: destination, ..
        } = destination
        {
            output.old_value = simulation_registers[*destination as usize];
            if let Rm::Reg { reg: source, .. } = source {
                simulation_registers[*destination as usize] +=
                    simulation_registers[*source as usize];
                output.number_of_cycles = 3;
            } else if let Rm::MemoryNoDisplacment(_) = source {
                let memory_index = source.calculate_memory_index(simulation_registers);
                simulation_registers[*destination as usize] += memory[memory_index as usize] as i16;
            }
            output.new_value = simulation_registers[*destination as usize];
        } else if let Rm::MemoryNoDisplacment(_) = destination {
            let memory_index = destination.calculate_memory_index(simulation_registers) as usize;
            output.old_value = memory[memory_index] as i16;
            if let Rm::Reg { reg: source, .. } = source {
                memory[memory_index] += simulation_registers[*source as usize] as u8;
            }
            output.old_value = memory[memory_index] as i16;
        } else if let Rm::MemoryWithDisplacment { rm: register_index, displacment } = destination {
            let memory_index =
                (simulation_registers[*register_index] + *displacment as i16) as usize;
            output.old_value = memory[memory_index] as i16;

            if let Rm::Reg { reg: source, .. } = source {
                memory[memory_index] += simulation_registers[*source as usize] as u8;
                output.new_value = simulation_registers[*source as usize];
                output.number_of_cycles = 16 + destination.estimate_cycles();
            }
        }

        flags.update_from_number(output.new_value);
        output
    }
}

pub struct SubRmToRmSimulator;

impl RMToRmSimulator for SubRmToRmSimulator {
    fn simulate(&self, input: SimulatorInput) -> SimulatorOutput {
        let SimulatorInput {
            destination,
            simulation_registers,
            source,
            flags,
            ..
        } = input;
        let mut output = SimulatorOutput::default();

        if let Rm::Reg {
            reg: destination, ..
        } = destination
        {
            output.old_value = simulation_registers[*destination as usize];
            if let Rm::Reg { reg: source, .. } = source.unwrap() {
                simulation_registers[*destination as usize] -=
                    simulation_registers[*source as usize];
            }
            output.new_value = simulation_registers[*destination as usize];
            flags.update_from_number(simulation_registers[*destination as usize]);
        }

        output
    }
}

pub struct CmpRmToRmSimulator;

impl RMToRmSimulator for CmpRmToRmSimulator {
    fn simulate(&self, input: SimulatorInput) -> SimulatorOutput {
        let SimulatorInput {
            destination,
            simulation_registers,
            source,
            flags,
            ..
        } = input;
        let mut output = SimulatorOutput::default();

        if let Rm::Reg {
            reg: destination, ..
        } = destination
        {
            output.old_value = simulation_registers[*destination as usize];
            if let Rm::Reg { reg: source, .. } = source.unwrap() {
                flags.update_from_number(
                    simulation_registers[*destination as usize]
                        - simulation_registers[*source as usize],
                );
            }
            output.new_value = simulation_registers[*destination as usize];
        }

        output
    }
}
