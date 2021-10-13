bitflags! {
    pub(crate) struct Parts: u8 {
        const NONE     = 0b00000;
        const OK       = 0b00001;
        const DEBUG    = 0b00010;
        const ERROR    = 0b00100;
        const ERR_EXPR = 0b01000;
        const ERROR_ERR_EXPR          = Self::ERROR.bits | Self::ERR_EXPR.bits;
        const OK_DEBUG                = Self::OK.bits    | Self::DEBUG.bits;
        const OK_ERROR                = Self::OK.bits    | Self::ERROR.bits;
        const OK_ERR_EXPR             = Self::OK.bits    | Self::ERR_EXPR.bits;
        const OK_ERROR_ERR_EXPR       = Self::OK.bits    | Self::ERROR.bits     | Self::ERR_EXPR.bits;
        const OK_DEBUG_ERROR          = Self::OK.bits    | Self::DEBUG.bits     | Self::ERROR.bits;
        const OK_DEBUG_ERR_EXPR       = Self::OK.bits    | Self::DEBUG.bits     | Self::ERR_EXPR.bits;
        const OK_DEBUG_ERROR_ERR_EXPR = Self::OK.bits    | Self::DEBUG.bits     | Self::ERROR.bits     | Self::ERR_EXPR.bits;
        const DEBUG_ERROR             = Self::DEBUG.bits | Self::ERROR.bits;
        const DEBUG_ERR_EXPR          = Self::DEBUG.bits | Self::ERR_EXPR.bits;
        const DEBUG_ERROR_ERR_EXPR    = Self::DEBUG.bits | Self::ERROR.bits     | Self::ERR_EXPR.bits;
    }
}
