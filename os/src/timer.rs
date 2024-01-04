use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer;
use core::ops::{AddAssign, Deref, DerefMut};
use core::time::Duration;
use riscv::register::time;

const TICKS_PER_SECOND: u64 = 1000;
const MICRO_PER_SECOND: u64 = 1000000;

pub fn set_next_trigger() {
    set_timer(time::read() + (CLOCK_FREQ / TICKS_PER_SECOND) as usize)
}

#[derive(Clone, Copy, Default)]
pub struct Time {
    start: Duration,
}

impl Time {
    pub fn up_time() -> Self {
        Self {
            start: Duration::from_micros(time::read64() * MICRO_PER_SECOND / CLOCK_FREQ),
        }
    }

    pub fn duration_since(&self, earlier: Self) -> Duration {
        self.start - earlier.start
    }

    pub fn elapsed(&self) -> Duration {
        Time::up_time().duration_since(*self)
    }

    pub fn elapsed_and_update(&mut self) -> Duration {
        let elasped = self.elapsed();
        self.start.add_assign(elasped);
        elasped
    }

    pub const fn zero() -> Self {
        Self {
            start: Duration::ZERO,
        }
    }
}

impl Deref for Time {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.start
    }
}

impl DerefMut for Time {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.start
    }
}
