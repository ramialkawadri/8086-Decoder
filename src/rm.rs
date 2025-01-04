use std::{fmt::Display, fs::File, io::Read};

use crate::{REGISTER_NAMES, constants::EFFECTIVE_MEMOERY_ADDRESS};

#[derive(Debug)]
pub enum Rm {
    Reg { w: usize, reg: usize },
    DirectMemory(u16),
    MemoryWithDisplacment { rm: usize, displacment: u16 },
    MemoryNoDisplacment(usize),
}

impl Rm {
    pub fn new(file: &mut File, mod_value: u8, w: usize, rm: usize) -> Rm {
        if mod_value == 0b00 {
            // Memory mode no displacment
            if rm == 0b110 {
                // Direct memory
                let mut displacment = [0u8, 0u8];
                file.read_exact(&mut displacment).unwrap();
                return Rm::DirectMemory(((displacment[1] as u16) << 8) | displacment[0] as u16);
            } else {
                return Rm::MemoryNoDisplacment(rm);
            }
        } else if mod_value == 0b01 {
            // Memory mode, 8-bit displacment
            let mut displacment = [0u8];
            file.read_exact(&mut displacment).unwrap();
            return Rm::MemoryWithDisplacment {
                rm,
                displacment: displacment[0] as u16,
            };
        } else if mod_value == 0b10 {
            // Memory mode, 16-bit displacment
            let mut displacment = [0u8, 0u8];
            file.read_exact(&mut displacment).unwrap();
            return Rm::MemoryWithDisplacment {
                rm,
                displacment: ((displacment[1] as u16) << 8) | displacment[0] as u16,
            };
        } else {
            return Rm::Reg { w, reg: rm };
        }
    }
}

impl Display for Rm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Rm::Reg { w, reg } => String::from(REGISTER_NAMES[*w][*reg]),
            Rm::DirectMemory(displacment) => format!("[{}]", displacment),
            Rm::MemoryWithDisplacment { rm, displacment } => {
                format!("[{} + {}]", EFFECTIVE_MEMOERY_ADDRESS[*rm], displacment)
            }
            Rm::MemoryNoDisplacment(rm) => format!("[{}]", EFFECTIVE_MEMOERY_ADDRESS[*rm]),
        };
        write!(f, "{}", string)
    }
}
