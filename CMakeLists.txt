cmake_minimum_required(VERSION 3.1)
# message("${CMAKE_C_FLAGS}")
# message("${CMAKE_LINKER_EXE_FLAGS}")


project(tos VERSION 1.0.0 LANGUAGES NONE)
set(LINKER_SCRIPT ${CMAKE_SOURCE_DIR}/linker.ld)


#set(CMAKE_ASM_NASM_COMPILER /home/tom/opt/cross/bin/nasm)
enable_language(C)
enable_language(ASM_NASM)


set(ARCH x86_64)

add_subdirectory(src)