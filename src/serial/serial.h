#ifndef SERIAL_H
#define SERIAL_H
#include <stdint.h>
#include "common/common.h"


void init_serial_port(uint16_t port);
int serial_received(uint16_t port);
char read_serial(uint16_t port);
int is_transmit_empty(uint16_t port);
void write_serial(uint16_t port, char a);


#endif