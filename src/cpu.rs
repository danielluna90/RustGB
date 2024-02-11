use super::registers::Registers;

pub struct CPU {
    pub registers: Registers,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
        }
    }

    pub fn step() -> u8 {
        12
    }
}
