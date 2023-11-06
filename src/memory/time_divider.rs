use super::InteruptFlags;

pub struct TimeDivider {
    internal_counter: u64,
    pub div: u8,
    pub tima: u8,
    pub tma: u8,
    pub tac: TimerControl,
}

pub enum ClockSelect {
    Hz4096 = 0,
    Hz262144 = 1,
    Hz65536 = 2,
    Hz16384 = 3,
}

pub struct TimerControl {
    pub timer_enable: bool,
    pub clock_select: ClockSelect,
}

impl TimeDivider {
    pub fn write_div(&mut self, value: u8) {
        self.div = 0;
    }

    pub fn incr_div(&mut self) {
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

    pub fn tick_tima(&mut self, interupt_flags: &mut InteruptFlags, cpu_freq_hz: u64) {
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

    pub fn tick_div(&mut self, cpu_freq_hz: u64) {
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
