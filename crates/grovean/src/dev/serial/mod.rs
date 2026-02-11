#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "x86_64")]
mod x86_64;

#[doc(hidden)]
#[cfg(target_arch = "aarch64")]
pub fn _print(args: ::core::fmt::Arguments) {
    aarch64::_print(args);
}

#[doc(hidden)]
#[cfg(target_arch = "x86_64")]
pub fn _print(args: ::core::fmt::Arguments) {
    x86_64::_print(args);
}

/// Print to the serial port.
#[macro_export]
macro_rules! serial_print {
  ($($arg:tt)*) => {
    $crate::dev::serial::_print(format_args!($($arg)*));
  };
}

/// Print INFO to the serial port.
#[macro_export]
macro_rules! serial_info {
  ($fmt:expr) => ($crate::serial_print!(concat!("INFO: ", $fmt)));
  ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!($fmt, $($arg)*));
}

/// Print INFO to the serial port followed by a newline.
#[macro_export]
macro_rules! serial_info_ln {
  () => ($crate::serial_print!("\n"));
  ($fmt:expr) => ($crate::serial_print!(concat!("INFO: ", $fmt, "\n")));
  ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}

/// Print DEBUG to the serial port.
#[macro_export]
macro_rules! serial_debug {
  ($fmt:expr) => ($crate::serial_print!(concat!("DEBUG: ", $fmt)));
  ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!($fmt, $($arg)*));
}

/// Print DEBUG to the serial port followed by a newline.
#[macro_export]
macro_rules! serial_debug_ln {
  () => ($crate::serial_print!("\n"));
  ($fmt:expr) => ($crate::serial_print!(concat!("DEBUG: ", $fmt, "\n")));
  ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}

/// Print WARN to the serial port.
#[macro_export]
macro_rules! serial_warn {
  ($fmt:expr) => ($crate::serial_print!(concat!("WARN: ", $fmt)));
  ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!($fmt, $($arg)*));
}

/// Print WARN to the serial port followed by a newline.
#[macro_export]
macro_rules! serial_warn_ln {
  () => ($crate::serial_print!("\n"));
  ($fmt:expr) => ($crate::serial_print!(concat!("WARN: ", $fmt, "\n")));
  ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}

/// Print DANGER to the serial port.
#[macro_export]
macro_rules! serial_danger {
  ($fmt:expr) => ($crate::serial_print!(concat!("DANGER: ", $fmt)));
  ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!($fmt, $($arg)*));
}

/// Print DANGER to the serial port followed by a newline.
#[macro_export]
macro_rules! serial_danger_ln {
  () => ($crate::serial_print!("\n"));
  ($fmt:expr) => ($crate::serial_print!(concat!("DANGER: ", $fmt, "\n")));
  ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}
