#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(const_unique_new)]
#![feature(ptr_internals)]
#![feature(asm)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;

#[macro_use]
mod io;
#[macro_use]
mod debug;

use io::vga_buffer;

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
    let boot_info = unsafe{ multiboot2::load(multiboot_information_address)};
    debug::init_debug();
    debug_println!("Hello Outside");
    vga_buffer::print_literal_shit();

    let memory_map_tag = boot_info.memory_map_tag()
    .expect("Memory map tag required");

    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("  start: 0x{:x}, length 0x{:x}", area.base_addr, area.length);
    }
    panic!();
    loop{};
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str,
    line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    debug_println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    debug_println!("    {}", fmt);
    loop{}
}
