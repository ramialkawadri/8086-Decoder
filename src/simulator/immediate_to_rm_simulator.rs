use crate::{flag::Flags, rm::Rm};

pub trait ImmediateToRMSimulator {
    fn simulate(
        &self,
        simulation_registers: &mut [i16; 8],
        flags: &mut Flags,
        destination: &Rm,
        value: i16,
    );
}

pub struct AddImmediateToRMSimulator;

impl ImmediateToRMSimulator for AddImmediateToRMSimulator {
    fn simulate(
        &self,
        simulation_registers: &mut [i16; 8],
        flags: &mut Flags,
        destination: &Rm,
        value: i16,
    ) {
        if let Rm::Reg {
            reg: destination, ..
        } = destination
        {
            simulation_registers[*destination as usize] += value;
            flags.update_from_number(simulation_registers[*destination as usize]);
        }
    }
}

pub struct SubImmediateToRMSimulator;

impl ImmediateToRMSimulator for SubImmediateToRMSimulator {
    fn simulate(
        &self,
        simulation_registers: &mut [i16; 8],
        flags: &mut Flags,
        destination: &Rm,
        value: i16,
    ) {
        if let Rm::Reg {
            reg: destination, ..
        } = destination
        {
            simulation_registers[*destination as usize] -= value;
            flags.update_from_number(simulation_registers[*destination as usize]);
        }
    }
}

pub struct CmpImmediateToRMSimulator;

impl ImmediateToRMSimulator for CmpImmediateToRMSimulator {
    fn simulate(
        &self,
        simulation_registers: &mut [i16; 8],
        flags: &mut Flags,
        destination: &Rm,
        value: i16,
    ) {
        if let Rm::Reg {
            reg: destination, ..
        } = destination
        {
            flags.update_from_number(simulation_registers[*destination as usize] - value);
        }
    }
}
