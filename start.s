//Declare kernal main as external function for linking
.extern kernel_main

//Set start to global for linking
.global start


//Multiboot grub bullshit.
// Our bootloader, GRUB, needs to know some basic information about our kernel before it can boot it.
// We give GRUB this information using a standard known as 'Multiboot'.
// To define a valid 'Multiboot header' that will be recognised by GRUB, we need to hard code some
// constants into the executable. The following code calculates those constants.
.set MB_MAGIC, 0x1BADB002          // This is a 'magic' constant that GRUB will use to detect our kernels location.
.set MB_FLAGS, (1 << 0) | (1 << 1) // This tells GRUB to 1: load modules on page boundaries and 2: provide a memory map (this is useful later in development)
// Finally, we calculate a checksum that includes all the previous values
.set MB_CHECKSUM, (0 - (MB_MAGIC + MB_FLAGS))


//Multiboot section 
.section .multiboot
	.align 4
	.long MB_MAGIC
	.long MB_FLAGS
	.long MB_CHECKSUM

//This section is zeroed out when the kernel starts.
.section .bss
	.align 16
	stack_bottom:
		.skip 4096 //We allocate 4k of ram for the stack.
		stack_top:

.section .text
	start:
		mov $stack_top, %esp //stack grows down so move stack pointer to the top.
		call kernel_main // call c code
		//if it errors clear interrupts then halt if that doesnt work try again.
		hang:
			cli
			hlt
			jmp hang