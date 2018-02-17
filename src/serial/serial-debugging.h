#ifndef SERIAL_DEBUGGING_H
#define SERIAL_DEBUGGING_H

#define DEBUG_PORT 0x3F8

void init_debug_port();
void debug_print(const char* str);

#endif