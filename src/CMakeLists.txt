
set(ARCH_DIR arch/${ARCH})

set(CMAKE_C_FLAGS "-O2 -std=gnu99 -ffreestanding -g")
set(CMAKE_CXX_FLAGS ${CMAKE_C_FLAGS})
set(CFLAGS ${CMAKE_C_FLAGS})
set(CXXFLAGS ${CMAKE_C_FLAGS})
set(LINKER_FLAGS "-n -ffreestanding -nostdlib -g -T ${LINKER_SCRIPT} -lgcc")

set(LDFLAGS ${LINKER_FLAGS})
set(CMAKE_EXE_LINKER_FLAGS ${LINKER_FLAGS})
set(CMAKE_SHARED_LINKER_FLAGS ${LINKER_FLAGS})
set(CMAKE_MODULE_LINKER_FLAGS ${LINKER_FLAGS})

message("${ARCH_DIR}/multiboot_header.s")

include_directories(${CMAKE_CURRENT_SOURCE_DIR})
#add_library(start OBJECT start.s)
message("${INCLUDE_DIRECTORIES}")
add_executable(kernel.elf
	${ARCH_DIR}/start.asm
	${ARCH_DIR}/multiboot_header.asm
	${ARCH_DIR}/long_mode_init.asm
	#common/common.c
	#display/vga.c
	#serial/serial.c
	#serial/serial-debugging.c
	kernel.c
)

set_target_properties(kernel.elf PROPERTIES LINK_DEPENDS ${LINKER_SCRIPT})