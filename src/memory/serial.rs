pub struct SerialRegister {
    pub data: u8,
    pub control: SerialControl,
}

pub enum SerialClockSpeed {
    Normal = 0,
    Double = 1,
}

pub enum SerialClockSelect {
    ExternalClock = 0,
    InternalClock = 1,
}

pub struct SerialControl {
    pub transfer_enable: bool,
    pub clock_speed: SerialClockSpeed,
    pub clock_select: SerialClockSelect,
}
