use crate::{flag::Flags, rm::Rm};

pub mod immediate_to_rm_simulator;
pub mod rm_to_rm_simulator;

pub struct SimulatorInput<'a> {
    pub simulation_registers: &'a mut [i16; 8],
    pub memory: &'a mut [u8; 65536],
    pub flags: &'a mut Flags,
    pub source: Option<&'a Rm>,
    pub destination: &'a Rm,
    pub immediate_value: Option<i16>,
}

#[derive(Default)]
pub struct SimulatorOutput {
    pub old_value: i16,
    pub new_value: i16,
    pub number_of_cycles: i16,
}
