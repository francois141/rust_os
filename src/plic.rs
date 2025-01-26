const PLIC_PRIORITY: usize = 0x0c00_0000;
const _PLIC_PENDING: usize = 0x0c00_1000;
const PLIT_INT_ENABLE_TABLE: usize = 0x0c00_2000;
const PLIC_THRESHOLD: usize = 0x0c20_0000;
const PLIC_CLAIM: usize = 0x0c20_0004;

const UART_DEVICE: u32 = 10;

pub fn init() {
    set_threshold(0);
    enable_device(UART_DEVICE);
    set_priority(UART_DEVICE, 1);
}

pub fn init_sanity_check() {}

pub fn enable_device(id: u32) {
    let plic_enable_mask = PLIT_INT_ENABLE_TABLE as *mut u32;
    unsafe {
        plic_enable_mask.write_volatile(plic_enable_mask.read_volatile() | (1 << id));
    }
}

pub fn set_priority(id: u32, priority: u8) {
    let current_priority = priority as u32 & 0x7;
    let priority_register = PLIC_PRIORITY as *mut u32;

    unsafe {
        priority_register
            .add(id as usize)
            .write_volatile(current_priority)
    }
}

pub fn set_threshold(threshold: u8) {
    let actual_threshold = threshold & 0x7;
    let threshold_register = PLIC_THRESHOLD as *mut u32;
    unsafe {
        threshold_register.write_volatile(actual_threshold as u32);
    }
}

pub fn next_interrupt() -> Option<u32> {
    let claim_register = PLIC_CLAIM as *const u32;
    let claim_number;

    unsafe {
        claim_number = claim_register.read_volatile();
    }

    if claim_number == 0 {
        None
    } else {
        Some(claim_number)
    }
}

pub fn clear_interrupt(id: u32) {
    let complete_register = PLIC_CLAIM as *mut u32;
    unsafe {
        complete_register.write_volatile(id);
    }
}
