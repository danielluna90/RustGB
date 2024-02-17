use std::fmt;

pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
        }
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }
    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }
    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A: {:#04x}, F: {:#04x} (C: , H: , N: , Z: )\nB: {:#04x}, C: {:#04x}\nD: {:#04x}, E: {:#04x}\nH: {:#04x}, L: {:#04x}", self.a, self.f, self.b, self.c, self.d, self.e, self.h, self.l)
    }
}

#[cfg(test)]
mod tests {
    use crate::registers::Registers;

    #[test]
    fn get_bc() {
        let mut regs: Registers = Registers::new();

        regs.b = 0xAB;
        regs.c = 0xCD;

        assert_eq!(regs.get_bc(), 0xABCD);
    }

    #[test]
    fn get_de() {
        let mut regs: Registers = Registers::new();

        regs.d = 0xAB;
        regs.e = 0xCD;

        assert_eq!(regs.get_de(), 0xABCD);
    }

    #[test]
    fn get_hl() {
        let mut regs: Registers = Registers::new();

        regs.h = 0xAB;
        regs.l = 0xCD;

        assert_eq!(regs.get_hl(), 0xABCD);
    }
}