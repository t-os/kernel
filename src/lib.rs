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
extern crate bitflags;
extern crate x86_64;

#[macro_use]
mod io;
#[macro_use]
mod debug;

mod memory;


use io::vga_buffer;

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
    use memory::FrameAllocator;
    let boot_info = unsafe{ multiboot2::load(multiboot_information_address)};
    //debug::init_debug();
    //debug_println!("Hello Outside");
    vga_buffer::print_literal_shit();

    let memory_map_tag = boot_info.memory_map_tag()
    .expect("Memory map tag required");
    let elf_sections_tag = boot_info.elf_sections_tag()
    .expect("Elf-sections tag required");

    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("  start: 0x{:x}, length 0x{:x}", area.base_addr, area.length);
    }

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr).min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size).max().unwrap();
    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);
    println!("Kernel Start: 0x{:x}, Kernel End 0x{:x}", kernel_start, kernel_end);
    println!("MB2 Start: 0x{:x}, MB2 End 0x{:x}", multiboot_start, multiboot_end);

    let mut frame_allocator = memory::AreaFrameAllocator::new(
    kernel_start as usize, kernel_end as usize, multiboot_start,
    multiboot_end, memory_map_tag.memory_areas());

    enable_nxe_bit();
    enable_write_protect_bit();
    memory::remap_the_kernel(&mut frame_allocator, boot_info);
    println!("It did not crash!");

    loop {}
}

fn enable_nxe_bit() {
    use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};

    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}

#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    //debug_println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    //debug_println!("    {}", fmt);
    loop{}
}
