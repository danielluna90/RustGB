enum GBType {
    Original,
    ColorInOriginal,
    Color,
}

enum Flags {
    C = 0b00010000,
    H = 0b00100000,
    N = 0b01000000,
    Z = 0b10000000,
}

pub struct Registers {
    pc: u16,
    sp: u16,
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            pc: 0,
            sp: 0,
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

    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }

    pub fn set_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f = (val & 0xF0) as u8;
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu;

    use super::Registers;

    #[test]
    fn registers() {
        let mut reg = Registers::new();

        reg.a = 0x10;
        reg.f = 0x20;
        assert_eq!(reg.get_af(), 0x1020);

        reg.b = 0x30;
        reg.c = 0x40;

        reg.d = 0x50;
        reg.e = 0x60;

        reg.h = 0x70;
        reg.l = 0x80;
    }
}
