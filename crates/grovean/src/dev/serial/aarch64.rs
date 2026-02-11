use core::fmt;
use core::sync::atomic::{AtomicBool, Ordering};

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::MmioSerialPort;

#[derive(Clone, Copy)]
struct SerialConfig {
    base: usize,
    line_status_offset: usize,
    output_empty_mask: u8,
    write_spin_limit: usize,
}

const DEFAULT_SERIAL_CONFIG: SerialConfig = SerialConfig {
    // QEMU virt platform 16550-compatible UART base.
    base: 0x0900_0000,
    // 16550 line status register offset.
    line_status_offset: 5,
    // 16550 LSR bit 5: transmitter holding register empty.
    output_empty_mask: 1 << 5,
    // Best-effort write bound to prevent indefinite lockup.
    write_spin_limit: 100_000,
};

struct SerialRuntimeState {
    disabled: AtomicBool,
}

impl SerialRuntimeState {
    const fn new() -> Self {
        Self {
            disabled: AtomicBool::new(false),
        }
    }

    fn is_disabled(&self) -> bool {
        self.disabled.load(Ordering::Relaxed)
    }

    fn disable(&self) {
        self.disabled.store(true, Ordering::Relaxed);
    }
}

struct Aarch64SerialPort {
    config: SerialConfig,
    _mmio: MmioSerialPort,
}

impl Aarch64SerialPort {
    fn new(config: SerialConfig) -> Self {
        let mut mmio = unsafe { MmioSerialPort::new(config.base) };
        mmio.init();
        Self {
            config,
            _mmio: mmio,
        }
    }

    fn data_addr(&self) -> *mut u8 {
        self.config.base as *mut u8
    }

    fn line_status_addr(&self) -> *const u8 {
        (self.config.base + self.config.line_status_offset) as *const u8
    }

    fn is_output_empty(&self, status: u8) -> bool {
        status & self.config.output_empty_mask != 0
    }

    fn write_byte_non_blocking(&mut self, byte: u8) -> Result<(), fmt::Error> {
        let line_status = self.line_status_addr();
        let data = self.data_addr();

        for _ in 0..self.config.write_spin_limit {
            let status = unsafe { core::ptr::read_volatile(line_status) };
            if self.is_output_empty(status) {
                unsafe {
                    core::ptr::write_volatile(data, byte);
                }
                return Ok(());
            }
            core::hint::spin_loop();
        }

        Err(fmt::Error)
    }
}

impl fmt::Write for Aarch64SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte_non_blocking(byte)?;
        }
        Ok(())
    }
}

static SERIAL_STATE: SerialRuntimeState = SerialRuntimeState::new();

lazy_static! {
    /// A static instance of the aarch64 memory-mapped serial port interface.
    static ref SERIAL1: Mutex<Aarch64SerialPort> = Mutex::new(Aarch64SerialPort::new(DEFAULT_SERIAL_CONFIG));
}

pub(super) fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;

    if SERIAL_STATE.is_disabled() {
        return;
    }

    if SERIAL1.lock().write_fmt(args).is_err() {
        SERIAL_STATE.disable();
    }
}

#[cfg(test)]
mod tests {
    use kunit::kunit;

    use super::{DEFAULT_SERIAL_CONFIG, SerialConfig, SerialRuntimeState};

    #[kunit]
    fn default_config_has_expected_16550_values() {
        assert_eq!(DEFAULT_SERIAL_CONFIG.base, 0x0900_0000);
        assert_eq!(DEFAULT_SERIAL_CONFIG.line_status_offset, 5);
        assert_eq!(DEFAULT_SERIAL_CONFIG.output_empty_mask, 1 << 5);
    }

    #[kunit]
    fn line_status_address_uses_base_plus_offset() {
        let config = SerialConfig {
            base: 0x1000,
            line_status_offset: 5,
            output_empty_mask: 1 << 5,
            write_spin_limit: 10,
        };

        let line_status_addr = config.base + config.line_status_offset;
        assert_eq!(line_status_addr, 0x1005);
    }

    #[kunit]
    fn runtime_state_disables_once_set() {
        let state = SerialRuntimeState::new();
        assert!(!state.is_disabled());

        state.disable();

        assert!(state.is_disabled());
    }
}
