
// Unsafe function does no checking to see if 2 writes is ocurring at the same time

use io::port_io::{outb, inb};
use core::fmt;

pub struct SerialPort {
    pub port: u16,
    pub baudrate: i32,
}

impl SerialPort {
    pub fn initialize(&mut self) {
        unsafe {
            let divisor = (115200  / self.baudrate) as u16;
            let hibyte = (divisor >> 8) as u8;
            let lobyte = (divisor & 0x00FF) as u8;
            outb(self.port + 1, 0x00);    // Disable all interrupts
            outb(self.port + 3, 0x80);    // Enable DLAB (set baud rate divisor)
            outb(self.port + 0, lobyte);    // Set divisor to 3 (lo byte) 38400 baud
            outb(self.port + 1, hibyte);    //                  (hi byte)
            outb(self.port + 3, 0x03);    // 8 bits, no parity, one stop bit
            outb(self.port + 2, 0xC7);    // Enable FIFO, clear them, with 14-byte threshold
            outb(self.port + 4, 0x0B);
        }
    }

    pub fn write_byte(&mut self, byte: u8)
    {
        unsafe {
            while inb(self.port + 5) & 0x20 == 0 {}
            outb(self.port, byte);
        }
    }

    pub unsafe fn read_byte(&mut self) -> u8 {
        while inb(self.port + 5) & 1 == 0 {}
        return inb(self.port);
    }

}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
          self.write_byte(byte)
        }
        Ok(())
    }
}
