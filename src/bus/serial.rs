use super::Memory;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum SerialClockSpeed {
    #[default]
    Normal = 0,
    Double = 1,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum SerialClockSelect {
    #[default]
    ExternalClock = 0,
    InternalClock = 1,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct SerialControl {
    pub transfer_enable: bool,
    pub clock_speed: SerialClockSpeed,
    pub clock_select: SerialClockSelect,
}

impl std::convert::From<SerialControl> for u8 {
    fn from(value: SerialControl) -> Self {
        let mut result = 0b1000_0000;
        if value.transfer_enable {
            result |= 0b1000_0000;
        }
        if value.clock_speed == SerialClockSpeed::Double {
            result |= 0b0000_0010;
        }
        if value.clock_select == SerialClockSelect::InternalClock {
            result |= 0b0000_0001;
        }
        result
    }
}

impl std::convert::From<u8> for SerialControl {
    fn from(value: u8) -> Self {
        let transfer_enable = value & 0b1000_0000 != 0;
        let clock_speed = match value & 0b0000_0010 {
            0b00 => SerialClockSpeed::Normal,
            0b10 => SerialClockSpeed::Double,
            _ => panic!("Invalid clock speed value: {:04X}", value),
        };
        let clock_select = match value & 0b0000_0001 {
            0b00 => SerialClockSelect::ExternalClock,
            0b01 => SerialClockSelect::InternalClock,
            _ => panic!("Invalid clock select value: {:04X}", value),
        };
        SerialControl {
            transfer_enable,
            clock_speed,
            clock_select,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct SerialRegister {
    pub data: u8,
    pub control: SerialControl,
}

impl Memory for SerialRegister {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF01 => self.data,
            0xFF02 => self.control.into(),
            _ => 0,
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF01 => self.data = value,
            0xFF02 => self.control = value.into(),
            _ => (),
        }
    }
}
