#include "serial.h"
#include "display/vga.h"
void init_serial_port(uint16_t port)
{
	outb(port + 1, 0x00);    // Disable all interrupts
	outb(port + 3, 0x80);    // Enable DLAB (set baud rate divisor)
	outb(port + 0, 0x03);    // Set divisor to 3 (lo byte) 38400 baud
	outb(port + 1, 0x00);    //                  (hi byte)
	outb(port + 3, 0x03);    // 8 bits, no parity, one stop bit
	outb(port + 2, 0xC7);    // Enable FIFO, clear them, with 14-byte threshold
	outb(port + 4, 0x0B);    // IRQs enabled, RTS/DSR set
}

int serial_received(uint16_t port)
{
	return inb(port + 5) & 1;
}

char read_serial(uint16_t port)
{
	while (serial_received(port) == 0);
	return inb(port);
}

int is_transmit_empty(uint16_t port)
{
	uint8_t ret =inb(port + 5) & 0x20;
	return ret;
}

void write_serial(uint16_t port, char a)
{
	while (is_transmit_empty(port) == 0);
	outb(port, a);
}
