//! Delays

use embedded_hal::delay::DelayNs;


/// Use RISCV machine-mode cycle counter (`mcycle`) as a delay provider.
///
/// This can be used for high resolution delays for device initialization,
/// bit-banging protocols, etc
#[derive(Copy, Clone)]
pub struct McycleDelay {
    core_frequency: u32,
}

impl McycleDelay {
    /// Constructs the delay provider based on core clock frequency `freq`
    /// `freq`: System clock frequency, used to convert clock cycles
    /// into real-world time values
    pub fn new(freq: u32) -> Self {
        Self {
            core_frequency: freq,
        }
    }

    /// Retrieves the cycle count for the current HART
    #[inline]
    pub fn get_cycle_count() -> u64 {
        riscv::register::mcycle::read64()
    }

    /// Returns the number of elapsed cycles since `previous_cycle_count`
    #[inline]
    pub fn cycles_since(previous_cycle_count: u64) -> u64 {
        riscv::register::mcycle::read64().wrapping_sub(previous_cycle_count)
    }

    /// Performs a busy-wait loop until the number of cycles `cycle_count` has elapsed
    #[inline]
    pub fn delay_cycles(cycle_count: u64) {
        let start_cycle_count = McycleDelay::get_cycle_count();

        while McycleDelay::cycles_since(start_cycle_count) <= cycle_count {}
    }
}

impl DelayNs for McycleDelay {

    #[inline]
    fn delay_ns(&mut self, ns: u32) {
        McycleDelay::delay_cycles(((ns as u64)  * (self.core_frequency as u64)) / 1_000_000_0000u64);
    }

    #[inline]
    fn delay_us(&mut self, us: u32) {
        McycleDelay::delay_cycles(((us as u64)  * (self.core_frequency as u64)) / 1_000_000u64);
    }

    #[inline]
    fn delay_ms(&mut self, ms: u32) {
        McycleDelay::delay_cycles(((ms as u64) * (self.core_frequency as u64)) / 1000u64);
    }
}
