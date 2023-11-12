use super::{InteruptFlags, Memory};

#[derive(Default, Debug, Clone, Copy)]
pub enum ClockSelect {
    #[default]
    Hz4096 = 0,
    Hz262144 = 1,
    Hz65536 = 2,
    Hz16384 = 3,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct TimerControl {
    pub timer_enable: bool,
    pub clock_select: ClockSelect,
}

impl std::convert::From<TimerControl> for u8 {
    fn from(value: TimerControl) -> Self {
        let mut result = 0b0000_0000;
        if value.timer_enable {
            result |= 0b0000_0100;
        }
        result |= value.clock_select as u8;
        result
    }
}

impl std::convert::From<u8> for TimerControl {
    fn from(value: u8) -> Self {
        let timer_enable = value & 0b0000_0100 != 0;
        let clock_select = match value & 0b0000_0011 {
            0b00 => ClockSelect::Hz4096,
            0b01 => ClockSelect::Hz262144,
            0b10 => ClockSelect::Hz65536,
            0b11 => ClockSelect::Hz16384,
            _ => panic!("Invalid clock select value: {:04X}", value),
        };
        TimerControl {
            timer_enable,
            clock_select,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct TimeDividerRegister {
    internal_counter: u64,
    pub div: u8,
    pub tima: u8,
    pub tma: u8,
    pub tac: TimerControl,
}

impl TimeDividerRegister {
    fn incr_div(&mut self) {
        self.div = self.div.wrapping_add(1);
    }

    fn incr_tima(&mut self, interupt_flags: &mut InteruptFlags) {
        let (tima, overflow) = self.tima.overflowing_add(1);
        if overflow {
            self.tima = self.tma;
            interupt_flags.timer = true;
        } else {
            self.tima = tima;
        }
    }

    fn tick_tima(&mut self, interupt_flags: &mut InteruptFlags, cpu_freq_hz: u64) {
        let clock_select = match self.tac.clock_select {
            ClockSelect::Hz4096 => 1024,
            ClockSelect::Hz262144 => 16,
            ClockSelect::Hz65536 => 64,
            ClockSelect::Hz16384 => 256,
        };

        let cycles_per_tick = cpu_freq_hz / clock_select;
        if self.internal_counter >= cycles_per_tick {
            self.incr_tima(interupt_flags);
            self.internal_counter = 0;
        }
    }

    fn tick_div(&mut self, cpu_freq_hz: u64) {
        let cycles_per_tick = cpu_freq_hz / 16384;
        if self.internal_counter >= cycles_per_tick {
            self.incr_div();
            self.internal_counter = 0;
        }
    }

    pub fn tick(&mut self, interupt_flags: &mut InteruptFlags, cpu_freq_hz: u64) {
        self.internal_counter += 1;
        self.tick_tima(interupt_flags, cpu_freq_hz);
        self.tick_div(cpu_freq_hz);
    }
}

impl Memory for TimeDividerRegister {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF04 => self.div,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac.into(),
            _ => 0,
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.div = 0,
            // 0xFF05 => self.tima = value,
            0xFF06 => self.tma = value,
            0xFF07 => self.tac = value.into(),
            _ => panic!(
                "Invalid write to TimeDividerRegister address: {:04X}",
                address
            ),
        }
    }
}
