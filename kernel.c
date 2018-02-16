#include <stddef.h>
#include <stdint.h>

// First, let's do some basic checks to make sure we are using our x86-elf cross-compiler correctly
#if defined(__linux__)
	#error "This code must be compiled with a cross-compiler"
#elif !defined(__i386__)
	#error "This code must be compiled with an x86-elf compiler"
#endif

#include "display/vga.h"
#include "serial/serial-debugging.h"
void kernel_main()
{
	term_init();
	term_print("Hello World's\n");
	term_print("Welcome to kernel land\n");
	init_debug_port();
	debug_print("Hello Serial");
	debug_print("Hello Again");
}


