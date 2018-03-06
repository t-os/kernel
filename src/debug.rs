use io::serial;
use core::fmt;
use spin::Mutex;

macro_rules! debug_println {
    ($fmt:expr) => (debug_print!(concat!($fmt, "\n\r")));
    ($fmt:expr, $($arg:tt)*) => (debug_print!(concat!($fmt, "\n\r"), $($arg)*));
}

macro_rules! debug_print {
    ($($arg:tt)*) => ({
        $crate::debug::print(format_args!($($arg)*));
    });
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    DEBUG.lock().write_fmt(args).unwrap();
}

pub fn init_debug()
{
    DEBUG.lock().initialize();
}

pub static DEBUG:Mutex<serial::SerialPort> = Mutex::new(serial::SerialPort{port: 0x3F8, baudrate: 38400});
