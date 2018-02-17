
// Unsafe function does no checking to see if 2 writes is ocurring at the same time

pub fn unsafe init_port(port: u16, baudrate: i32) {
    let divisor = (115200  / baudrate) as u16);
    let hibyte = (divisor >> 8) as u8;
    let lobyte = (divisor & 0x00FF) as u8;
    outb(port + 1, 0x00);    // Disable all interrupts
    outb(port + 3, 0x80);    // Enable DLAB (set baud rate divisor)
    outb(port + 0, lobyte);    // Set divisor to 3 (lo byte) 38400 baud
    outb(port + 1, hibyte);    //                  (hi byte)
    outb(port + 3, 0x03);    // 8 bits, no parity, one stop bit
    outb(port + 2, 0xC7);    // Enable FIFO, clear them, with 14-byte threshold
    outb(port + 4, 0x0B);
}


pub fn unsafe write_byte(port: u16, byte: u8)
{
    while ::port_io::inb(port + 5) & 0x20 == 0 {}

    ::port_io::out(port, byte);
}

pub fn unsafe read_byte(port: u16) -> u8 {
    while ::port_io::inb(port + 5) & 1 == 0 {}
    return inb(port);
}
