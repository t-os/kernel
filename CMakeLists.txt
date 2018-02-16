cmake_minimum_required(VERSION 3.1)
# message("${CMAKE_C_FLAGS}")
# message("${CMAKE_LINKER_EXE_FLAGS}")


project(tos VERSION 1.0.0 LANGUAGES NONE)
set(LINKER_SCRIPT ${CMAKE_SOURCE_DIR}/linker.ld)

set(CMAKE_C_FLAGS "-O2 -std=gnu99 -ffreestanding -g")
set(CMAKE_CXX_FLAGS ${CMAKE_C_FLAGS})
set(CFLAGS ${CMAKE_C_FLAGS})
set(CXXFLAGS ${CMAKE_C_FLAGS})
set(LINKER_FLAGS "-ffreestanding -nostdlib -g -T ${LINKER_SCRIPT} -lgcc")

set(LDFLAGS ${LINKER_FLAGS})
set(CMAKE_EXE_LINKER_FLAGS ${LINKER_FLAGS})
set(CMAKE_SHARED_LINKER_FLAGS ${LINKER_FLAGS})
set(CMAKE_MODULE_LINKER_FLAGS ${LINKER_FLAGS})
enable_language(C)
enable_language(ASM-ATT)


include_directories(${CMAKE_SOURCE_DIR})

message("${INCLUDE_DIRECTORIES}")
add_executable(kernel.elf
	start.s
	common/common.c
	display/vga.c
	serial/serial.c
	serial/serial-debugging.c
	kernel.c
)

set_target_properties(kernel.elf PROPERTIES LINK_DEPENDS ${LINKER_SCRIPT})