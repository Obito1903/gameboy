use std::default;

#[derive(Default, Debug, Clone, Copy)]
pub enum JoypadRead {
    Buttons,
    Directions,
    #[default]
    None,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct JoypadRegister {
    pub read: JoypadRead,
    pub buttons: JoypadButtons,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct JoypadButtons {
    pub a: bool,
    pub b: bool,
    pub select: bool,
    pub start: bool,
    pub right: bool,
    pub left: bool,
    pub up: bool,
    pub down: bool,
}

impl JoypadRegister {
    pub fn write_byte(&mut self, value: u8) {
        match value & 0b0011_0000 {
            0b0010_0000 => self.read = JoypadRead::Buttons,
            0b0001_0000 => self.read = JoypadRead::Directions,
            _ => self.read = JoypadRead::None,
        }
    }
}

impl std::convert::From<JoypadRegister> for u8 {
    fn from(value: JoypadRegister) -> Self {
        match value.read {
            JoypadRead::None => 0b0011_1111,
            JoypadRead::Buttons => {
                let mut result = 0b0010_0000;
                if value.buttons.a {
                    result |= 0b0000_0001;
                }
                if value.buttons.b {
                    result |= 0b0000_0010;
                }
                if value.buttons.select {
                    result |= 0b0000_0100;
                }
                if value.buttons.start {
                    result |= 0b0000_1000;
                }
                result
            }
            JoypadRead::Directions => {
                let mut result = 0b0001_0000;
                if value.buttons.right {
                    result |= 0b0000_0001;
                }
                if value.buttons.left {
                    result |= 0b0000_0010;
                }
                if value.buttons.up {
                    result |= 0b0000_0100;
                }
                if value.buttons.down {
                    result |= 0b0000_1000;
                }
                result
            }
        }
    }
}
