use crate::rm::Rm;

pub trait RMToRmSimulator {
    fn simulate(&self, simulation_registers: &mut [i16; 8], source: &Rm, destination: &Rm);
}

pub struct MovRmToRmSimulator;

impl RMToRmSimulator for MovRmToRmSimulator {
    fn simulate(&self, simulation_registers: &mut [i16; 8], source: &Rm, destination: &Rm) {
        if let Rm::Reg {
            reg: destination, ..
        } = destination
        {
            if let Rm::Reg { reg: source, .. } = source {
                simulation_registers[*destination as usize] =
                    simulation_registers[*source as usize];
            }
        }
    }
}

pub struct AddRmToRmSimulator;

impl RMToRmSimulator for AddRmToRmSimulator {
    fn simulate(&self, _simulation_registers: &mut [i16; 8], _source: &Rm, _destination: &Rm) {
        todo!()
    }
}
