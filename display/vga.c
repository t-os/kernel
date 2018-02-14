#include "vga.h"

#include <stddef.h>
#include <stdint.h>

//Pointer to the vga textmode buffer.
volatile uint16_t* vga_buffer = (uint16_t*)0xB8000;

//Width and height
const int VGA_COLS = 80;
const int VGA_ROWS = 25;

int term_col = 0;
int term_row = 0;

uint8_t term_color = 0x0F; //Black bg white fg.

// This function initiates the terminal by clearing it
void term_init()
{
	// Clear the textmode buffer
	for (int col = 0; col < VGA_COLS; col++)
	{
		for (int row = 0; row < VGA_ROWS; row++)
		{
			// The VGA textmode buffer has size (VGA_COLS * VGA_ROWS).
			// Given this, we find an index into the buffer for our character
			const size_t index = (VGA_COLS * row) + col;
			// Entries in the VGA buffer take the binary form BBBBFFFFCCCCCCCC, where:
			// - B is the background color
			// - F is the foreground color
			// - C is the ASCII character
			vga_buffer[index] = ((uint16_t)term_color << 8) | ' '; // Set the character to blank (a space character)
		}
	}
}

void term_putc(char c)
{
	switch (c)
	{
	case '\n':
		term_col = 0;
		term_row++;
		break;
	default:
	{
		const size_t index = (VGA_COLS * term_row) + term_col;
		//Bitshift the colour to the left 8 bits of the 16bit number then or the right 8 with the char effectivley putting the char into the right 8;
		vga_buffer[index] = ((uint16_t)term_color << 8) | c;
		term_col++;
		break;
	}
	}
	if (term_col >= VGA_COLS)
	{
		term_col = 0;
		term_row++;
	}
	if (term_row >= VGA_ROWS)
	{
		term_col = 0;
		term_row = 0;
	}
}

void term_print(const char* str)
{
	for (size_t i = 0; str[i] != '\0'; i++)
	{
		term_putc(str[i]);
	}
}