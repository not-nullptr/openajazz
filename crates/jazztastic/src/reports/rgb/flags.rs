// unused for now, purely because i think this is incorrect?

use bitflags::bitflags;

bitflags! {
    pub struct Flags: u8 {
        const RIGHT_TO_LEFT = 0b1;
        const UNKNOWN_1 = 0b1000;
        const UNKNOWN_2 = 0b100000;
    }
}

impl Default for Flags {
    fn default() -> Self {
        Flags::UNKNOWN_1 | Flags::UNKNOWN_2
    }
}
