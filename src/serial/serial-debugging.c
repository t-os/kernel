#include "serial-debugging.h"
#include "serial.h"
#include <stddef.h>

#include "display/vga.h"

void init_debug_port()
{
	init_serial_port(DEBUG_PORT);
}

void debug_print(const char* str)
{
	term_print("Debug Print: ");
	for (size_t i = 0; str[i] != '\0'; i++)
	{
		term_putc(str[i]);
		write_serial(DEBUG_PORT, str[i]);
	}
	write_serial(DEBUG_PORT, '\r');
	write_serial(DEBUG_PORT, '\n');
	term_putc('\n');
}
