use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct KeyboardFeatures: u64 {
        const RGB = 0b1;
        const TIME_SYNC = 0b10;
    }
}
