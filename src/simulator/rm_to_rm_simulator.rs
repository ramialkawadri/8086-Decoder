use crate::{flag::Flags, rm::Rm};

pub trait RMToRmSimulator {
    fn simulate(
        &self,
        simulation_registers: &mut [i16; 8],
        flags: &mut Flags,
        source: &Rm,
        destination: &Rm,
    );
}

pub struct MovRmToRmSimulator;

impl RMToRmSimulator for MovRmToRmSimulator {
    fn simulate(
        &self,
        simulation_registers: &mut [i16; 8],
        flags: &mut Flags,
        source: &Rm,
        destination: &Rm,
    ) {
        if let Rm::Reg {
            reg: destination, ..
        } = destination
        {
            if let Rm::Reg { reg: source, .. } = source {
                simulation_registers[*destination as usize] =
                    simulation_registers[*source as usize];
                flags.update_from_number(simulation_registers[*destination as usize]);
            }
        }
    }
}

pub struct AddRmToRmSimulator;

impl RMToRmSimulator for AddRmToRmSimulator {
    fn simulate(
        &self,
        simulation_registers: &mut [i16; 8],
        flags: &mut Flags,
        source: &Rm,
        destination: &Rm,
    ) {
        if let Rm::Reg {
            reg: destination, ..
        } = destination
        {
            if let Rm::Reg { reg: source, .. } = source {
                simulation_registers[*destination as usize] +=
                    simulation_registers[*source as usize];
                flags.update_from_number(simulation_registers[*destination as usize]);
            }
        }
    }
}

pub struct SubRmToRmSimulator;

impl RMToRmSimulator for SubRmToRmSimulator {
    fn simulate(
        &self,
        simulation_registers: &mut [i16; 8],
        flags: &mut Flags,
        source: &Rm,
        destination: &Rm,
    ) {
        if let Rm::Reg {
            reg: destination, ..
        } = destination
        {
            if let Rm::Reg { reg: source, .. } = source {
                simulation_registers[*destination as usize] -=
                    simulation_registers[*source as usize];
                flags.update_from_number(simulation_registers[*destination as usize]);
            }
        }
    }
}

pub struct CmpRmToRmSimulator;

impl RMToRmSimulator for CmpRmToRmSimulator {
    fn simulate(
        &self,
        simulation_registers: &mut [i16; 8],
        flags: &mut Flags,
        source: &Rm,
        destination: &Rm,
    ) {
        if let Rm::Reg {
            reg: destination, ..
        } = destination
        {
            if let Rm::Reg { reg: source, .. } = source {
                flags.update_from_number(
                    simulation_registers[*destination as usize]
                        - simulation_registers[*source as usize],
                );
            }
        }
    }
}
