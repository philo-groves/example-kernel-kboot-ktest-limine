#[cfg(target_arch = "x86_64")]
use lazy_static::lazy_static;
#[cfg(target_arch = "x86_64")]
use spin::Mutex;
#[cfg(target_arch = "x86_64")]
use uart_16550::SerialPort;

#[cfg(target_arch = "x86_64")]
lazy_static! {
  /// A static instance of the serial port interface.
  pub static ref SERIAL1: Mutex<SerialPort> = {
    let mut serial_port = unsafe { SerialPort::new(0x3F8) };
    serial_port.init();
    Mutex::new(serial_port)
  };
}

#[doc(hidden)]
#[cfg(target_arch = "x86_64")]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        SERIAL1
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
    });
}

#[doc(hidden)]
#[cfg(target_arch = "aarch64")]
pub fn _print(_args: ::core::fmt::Arguments) {}

/// Print to the serial port.
#[macro_export]
macro_rules! serial_print {
  ($($arg:tt)*) => {
    $crate::dev::serial::_print(format_args!($($arg)*));
  };
}

/// Print INFO to the serial port followed by a newline.
#[macro_export]
macro_rules! serial_info_ln {
  () => ($crate::serial_print!("\n"));
  ($fmt:expr) => ($crate::serial_print!(concat!("INFO: ", $fmt, "\n")));
  ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}

/// Print DEBUG to the serial port followed by a newline.
#[macro_export]
macro_rules! serial_debug_ln {
  () => ($crate::serial_print!("\n"));
  ($fmt:expr) => ($crate::serial_print!(concat!("DEBUG: ", $fmt, "\n")));
  ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}

/// Print WARN to the serial port followed by a newline.
#[macro_export]
macro_rules! serial_warn_ln {
  () => ($crate::serial_print!("\n"));
  ($fmt:expr) => ($crate::serial_print!(concat!("WARN: ", $fmt, "\n")));
  ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}

/// Print DANGER to the serial port followed by a newline.
#[macro_export]
macro_rules! serial_danger_ln {
  () => ($crate::serial_print!("\n"));
  ($fmt:expr) => ($crate::serial_print!(concat!("DANGER: ", $fmt, "\n")));
  ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}
