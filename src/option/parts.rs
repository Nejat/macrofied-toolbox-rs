bitflags! {
    pub struct Parts: u8 {
        const NOTHING = 0b00000;
        const SOME    = 0b00001;
        const DEBUG   = 0b00010;
        const NONE    = 0b00100;
        const SOME_DEBUG      = Self::SOME.bits  | Self::DEBUG.bits;
        const SOME_NONE       = Self::SOME.bits  | Self::NONE.bits;
        const SOME_DEBUG_NONE = Self::SOME.bits  | Self::DEBUG.bits     | Self::NONE.bits;
        const DEBUG_NONE      = Self::DEBUG.bits | Self::NONE.bits;
    }
}
