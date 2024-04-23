use crate::page_allocator;
use crate::uart;
use core::fmt::Write;

#[repr(i64)]
#[derive(Copy, Clone)]
pub enum EntryBits {
	None = 0,
	Valid = 1 << 0,
	Read = 1 << 1,
	Write = 1 << 2,
	Execute = 1 << 3,
	User = 1 << 4,
	Global = 1 << 5,
	Access = 1 << 6,
	Dirty = 1 << 7,

	ReadWrite = 1 << 1 | 1 << 2,
	ReadExecute = 1 << 1 | 1 << 3,
	ReadWriteExecute = 1 << 1 | 1 << 2 | 1 << 3,

	UserReadWrite = 1 << 1 | 1 << 2 | 1 << 4,
	UserReadExecute = 1 << 1 | 1 << 3 | 1 << 4,
	UserReadWriteExecute = 1 << 1 | 1 << 2 | 1 << 3 | 1 << 4,
}

impl EntryBits {
	pub fn val(self) -> i64 {
		self as i64
	}
}

pub struct PageTable {
    pub entries: [PageTableEntry; 512],
}

impl PageTable {
    pub fn len() -> usize {
        512
    }
}

#[derive(Copy, Clone)]
pub struct PageTableEntry {
    pub entry: i64,
}

impl PageTableEntry {
    pub fn is_valid(&self) -> bool {
        self.get_entry() & EntryBits::Valid.val() != 0
    }

    pub fn is_invalid(&self) -> bool {
        !self.is_valid()
    }

    pub fn is_branch(&self) -> bool {
        self.get_entry() & 0xe == 0
    }

    pub fn is_leaf(&self) -> bool {
        !self.is_branch()
    }

    pub fn set_entry(&mut self, entry: i64) {
        self.entry = entry
    }

    pub fn get_entry(&self) -> i64 {
        self.entry
    }
}

pub fn get_address_from_entry(entry: &PageTableEntry) -> i64 {
    (entry.get_entry() >> 10) << 12
}

pub fn get_virtual_offsets(virtual_address: usize) -> [usize; 3]{
    [(virtual_address >> 30) & 0x1ff,(virtual_address >> 21) & 0x1ff, (virtual_address >> 12) & 0x1ff]
}
 

pub fn map(virtual_address: usize, physical_address: usize, bits: i64) {
    // Safety assertion
    assert!(bits & 0xe != 0);

    let virtual_offsets = get_virtual_offsets(virtual_address);

    let physical_offsets = [(physical_address >> 30) & 0x3ff_ffff, (physical_address >> 21) & 0x1ff, (physical_address >> 12) & 0x1ff];

    unsafe {
        let mut current = &mut ROOT.entries[virtual_offsets[0]];

        for i in 1..=2 {
            if current.is_invalid() {
                // Create page
                let page = page_allocator::alloc(1);
                // Binds page
                current.set_entry((page as i64 >> 2) | EntryBits::Valid.val());
            }
            // Write to page data structure
            let entry = get_address_from_entry(current) as *mut PageTableEntry;
            current = entry.add(virtual_offsets[i]).as_mut().unwrap()
        }
        // Finally, we can write the leaf
    
        let mut entry = 0x0;
    
        // Build entry
        entry |= (physical_offsets[0] << 28) as i64;
        entry |= (physical_offsets[1] << 19) as i64;
        entry |= (physical_offsets[0] << 10) as i64;
        entry |= bits | EntryBits::Valid.val();
    
        // Set the entry
        current.set_entry(entry);
    }
}


pub fn virtual_to_physical(virtual_address: usize) -> Option<usize> {
    let virtual_offsets = get_virtual_offsets(virtual_address);

    unsafe {
        let mut current = &ROOT.entries[virtual_offsets[0]];

        for i in 0..=2 {
            if current.is_invalid() {
                return None
            }
    
            if current.is_leaf() {
                let offset_mask = (1 << (12 + i*9)) - 1;
                return Some(((get_address_from_entry(current) & !offset_mask) | (offset_mask & (virtual_address as i64))) as usize)
            }
    
            let current_entry = get_address_from_entry(current) as *mut PageTableEntry;
            current = current_entry.add(virtual_offsets[i+1]).as_ref().unwrap()
        }
    
        assert!(false, "This part should not be reachable");
    }

    None
}

pub const fn page_align_round_down(val: usize) -> usize {
	let o = 4096 - 1;
	val & !o
}


pub fn identity_map_range(start: usize, end: usize, bits: i64) {
    let current_address_start = page_align_round_down(start);
    let current_address_end = page_allocator::page_align_round_up(end);

    let number_pages = (current_address_end - current_address_start) / page_allocator::PAGE_SIZE;

    for i in 0..number_pages {
        map(current_address_start + 4096 * i, current_address_start + 4096*i, bits);
    }
}

pub fn identity_map_range_read_write(virtual_address: usize, physical_address: usize) {
    identity_map_range(virtual_address, physical_address, EntryBits::ReadWrite.val())
}

pub fn identity_map_range_read_execute(virtual_address: usize, physical_address: usize) {
    identity_map_range(virtual_address, physical_address, EntryBits::ReadExecute.val());
}

extern "C" {
    static TEXT_START:usize;
    static TEXT_END: usize;
    static DATA_START: usize;
    static DATA_END: usize;
    static RODATA_START: usize;
    static RODATA_END: usize;
	static BSS_START: usize;
	static BSS_END: usize;
    static KERNEL_STACK_START: usize;
    static KERNEL_STACK_END: usize;
    static HEAP_START: usize;
}

pub static mut ROOT: PageTable = PageTable{
    entries: [PageTableEntry{entry:0,}; 512],
};

pub fn init() {
    unsafe {
        // Map kernel code
        identity_map_range_read_write(TEXT_START,TEXT_END);    

        // Map kernel stack
        identity_map_range_read_write(KERNEL_STACK_START,KERNEL_STACK_END);    

        // Data section
        identity_map_range_read_write(DATA_START,DATA_END);   

        // Rodata section
        identity_map_range_read_execute(RODATA_START, RODATA_END);

        // Bss section
        identity_map_range_read_write(BSS_START,BSS_END);    
       
        // Map memory used for page allocation
        identity_map_range_read_write(HEAP_START,HEAP_START + page_allocator::ALLOCATED_PAGE_HEAP_ALLOCATOR);   
        
        // Map uart driver
        identity_map_range_read_write(uart::UART_BASE_ADDRESS, uart::UART_BASE_ADDRESS + page_allocator::PAGE_SIZE);
    }
}

pub fn init_sanity_check() {
    unsafe {
        assert!(TEXT_START == virtual_to_physical(TEXT_START).unwrap(), "Identity mapping is broken for TEXT section");
        assert!(KERNEL_STACK_START == virtual_to_physical(KERNEL_STACK_START).unwrap(), "Identity mapping is broken for KERNEL STACK section");
        assert!(DATA_START == virtual_to_physical(DATA_START).unwrap(), "Identity mapping is broken for DATA section");
        assert!(RODATA_START == virtual_to_physical(RODATA_START).unwrap(), "Identity mapping is broken is RODATA section");
        assert!(BSS_START == virtual_to_physical(BSS_START).unwrap(), "Identity mapping is broken is BSS section");
        assert!(HEAP_START == virtual_to_physical(HEAP_START).unwrap(), "Identity mapping is broken for heap allocator");
        assert!(uart::UART_BASE_ADDRESS == virtual_to_physical(uart::UART_BASE_ADDRESS).unwrap(), "Identity mapping is broken for uart driver");
    }
}


