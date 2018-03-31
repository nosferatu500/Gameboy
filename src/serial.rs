const EXTURNAL_CLOCK: u32 = 500 * 1024;
const INTERNAL_CLOCK: u32 = 8192;

pub struct Serial {
    pub data: u8,
    pub control: u8,

    pub transfer_flag: bool, // 0 - non-transfer; 1 - transfer;

    pub clock: bool, // 0 - external clock; 1 - internal clock;
}

impl Serial {
    pub fn new() -> Serial {
        Serial {
            data: 0,
            control: 0,

            transfer_flag: false,

            clock: false,
        }
    }
}
