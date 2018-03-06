#![allow(dead_code)]

pub trait InOut {
	unsafe fn port_in(port: u16) -> Self;
	unsafe fn port_out(port: u16, value: Self);
}

struct Port<T: InOut> {
	port: u16,
	phantom: PhantomData<T>,
	
}

impl InOut for u8 {
	unsafe fn port_in(port: u16) -> u8 { return inb(port); } 
	unsafe fn port_out(port: u16, value: u8) { outb(port, value); }
}

impl InOut for u16 {
	unsafe fn port_in(port: u16) -> u16 { return inw(port); } 
	unsafe fn port_out(port: u16, value: u16) { outw(port, value); }	
}

impl InOut for u32 {
	unsafe fn port_in(port: u16) -> u32 { return inl(port); } 
	unsafe fn port_out(port: u16, value: u32) { outl(port, value); }	
}

impl <T: InOut> Port<T> {
	pub const unsafe fn new(port: u16) -> Port<T> {
		return Port { port: port, phantom: PhantomData };
	}
	
	pub fn read(&self) -> T {
		unsafe { return T::port_in(self.port); }	
	}
	
	pub fn write(&self, value: T) {
		unsafe { T::port_out(self.port, value); }
	}
}








// Deadly port_io 
pub unsafe fn outb(port: u16, val: u8)
{
	asm!("outb $0, $1" : : "{al}"(val), "{dx}N"(port));
}

/// Read a single byte from the specified port
pub unsafe fn inb(port: u16) -> u8
{
	let ret : u8;
	asm!("inb $1, $0" : "={al}"(ret) : "{dx}N"(port));
	return ret;
}

/// Write a word (16-bits) to the specified port
pub unsafe fn outw(port: u16, val: u16)
{
	asm!("outw $0, $1" : : "{ax}"(val), "{dx}N"(port));
}

/// Read a word (16-bits) from the specified port
pub unsafe fn inw(port: u16) -> u16
{
	let ret : u16;
	asm!("inw $1, $0" : "={ax}"(ret) : "{dx}N"(port));
	return ret;
}

/// Write a long/double-word (32-bits) to the specified port
pub unsafe fn outl(port: u16, val: u32)
{
	asm!("outl $0, $1" : : "{eax}"(val), "{dx}N"(port));
}

/// Read a long/double-word (32-bits) from the specified port
pub unsafe fn inl(port: u16) -> u32
{
	let ret : u32;
	asm!("inl $1, $0" : "={eax}"(ret) : "{dx}N"(port));
	return ret;
}
