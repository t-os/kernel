use memory::PAGE_SIZE;

const ENTRY_COUNT: usize = 512;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

mod entry;
mod table;




pub fn translate(virtual_address: VirtualAddress)
    -> Option<PhysicalAddress>
{
    let offset = virtual_address % PAGE_SIZE;
    translate_page(Page::containing_address(virtual_address))
        .map(|frame| frame.number * PAGE_SIZE + offset)
}
