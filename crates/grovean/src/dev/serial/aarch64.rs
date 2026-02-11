use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::MmioSerialPort;

const UART0_MMIO_BASE: usize = 0x0900_0000;
const UART_LSR_OFFSET: usize = 5;
const UART_LSR_OUTPUT_EMPTY: u8 = 1 << 5;
const UART_WRITE_SPIN_LIMIT: usize = 100_000;

struct Aarch64SerialPort {
    base: usize,
    _mmio: MmioSerialPort,
}

impl Aarch64SerialPort {
    fn new(base: usize) -> Self {
        let mut mmio = unsafe { MmioSerialPort::new(base) };
        mmio.init();
        Self { base, _mmio: mmio }
    }

    fn write_byte_non_blocking(&mut self, byte: u8) -> Result<(), fmt::Error> {
        let line_sts = (self.base + UART_LSR_OFFSET) as *const u8;
        let data = self.base as *mut u8;

        for _ in 0..UART_WRITE_SPIN_LIMIT {
            let status = unsafe { core::ptr::read_volatile(line_sts) };
            if status & UART_LSR_OUTPUT_EMPTY != 0 {
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

lazy_static! {
    /// A static instance of the aarch64 memory-mapped serial port interface.
    static ref SERIAL1: Mutex<Aarch64SerialPort> = Mutex::new(Aarch64SerialPort::new(UART0_MMIO_BASE));
}

pub(super) fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;

    let _ = SERIAL1.lock().write_fmt(args);
}
