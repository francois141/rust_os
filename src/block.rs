use crate::page_allocator::{alloc, dealloc, PAGE_SIZE};
use crate::virtio;

use crate::virtio::{Descriptor, MmioOffset, Queue, VIRTIO_DESC_F_NEXT, VIRTIO_RING_SIZE};
use core::arch::asm;
use core::ptr;
use core::ptr::null_mut;

pub fn init() {}

pub fn init_sanity_check() {}

pub struct BlockDevice {
    queue: *mut Queue,
    dev: *mut u32,
    idx: u16,
}

pub static mut VIRTIO_BLOCK_DEVICE: BlockDevice = BlockDevice {
    queue: null_mut(),
    dev: null_mut(),
    idx: 0,
};

pub fn initialize_block_device(pointer: *mut u32) -> bool {
    let mut current_status_bit: u32 = 0;

    unsafe {
        // 1. Reset the device.
        current_status_bit |= virtio::StatusField::Acknowledge as u32;
        pointer
            .add(virtio::MmioOffset::Status as usize / 4)
            .write_volatile(0);

        // 2. Set the ACKNOWLEDGE status bit: the guest OS has noticed the device.
        pointer
            .add(virtio::MmioOffset::Status as usize / 4)
            .write_volatile(virtio::StatusField::Acknowledge as u32);

        // 3. Set the DRIVER status bit: the guest OS knows how to drive the device.
        current_status_bit |= virtio::StatusField::Driver as u32;
        pointer
            .add(virtio::MmioOffset::Status as usize / 4)
            .write_volatile(current_status_bit);

        // 4. Read device feature bits, and write the subset of feature bits understood by the OS and driver to the device. During this step the driver MAY read (but MUST NOT write) the device-specific configuration fields to check that it can support the device before accepting it.
        let features = pointer
            .add(virtio::MmioOffset::DeviceFeatures as usize / 4)
            .read_volatile();

        pointer
            .add(virtio::MmioOffset::DeviceFeatures as usize / 4)
            .write_volatile(features);

        // 5. Set the FEATURES_OK status bit. The driver MUST NOT accept new feature bits after this step.
        current_status_bit |= virtio::StatusField::FeaturesOk as u32;
        pointer
            .add(virtio::MmioOffset::Status as usize / 4)
            .write_volatile(current_status_bit);

        // 6. Re-read device status to ensure the FEATURES_OK bit is still set: otherwise, the device does not support our subset of features and the device is unusable.
        if pointer
            .add(virtio::MmioOffset::Status as usize / 4)
            .read_volatile()
            & virtio::StatusField::FeaturesOk as u32
            == 0
        {
            panic!("features fail...");
        };

        // 7. Perform device-specific setup, including discovery of virtqueues for the device,optional per-bussetup, reading and possibly writing the device’s virtio configuration space, and population of virtqueues.
        let number_queues = pointer
            .add(virtio::MmioOffset::QueueNumMax as usize / 4)
            .read_volatile();

        // This must be a power of two
        if VIRTIO_RING_SIZE > number_queues as usize {
            panic!("queue fails...");
        }

        // Writes the size of the queue
        pointer
            .add(virtio::MmioOffset::QueueNum as usize / 4)
            .write_volatile(VIRTIO_RING_SIZE as u32);

        // Selects the queue
        pointer
            .add(virtio::MmioOffset::QueueSel as usize / 4)
            .write_volatile(0);

        // Negotiates the page size with the host
        pointer
            .add(virtio::MmioOffset::GuestPageSize as usize / 4)
            .write_volatile(PAGE_SIZE as u32);

        let num_pages = (core::mem::size_of::<Queue>() + PAGE_SIZE - 1) / PAGE_SIZE;
        let queue_ptr = alloc(num_pages) as *mut Queue;

        // Physical memory address shifted by the page size
        pointer
            .add(virtio::MmioOffset::QueuePfn as usize / 4)
            .write_volatile((queue_ptr as u32) / PAGE_SIZE as u32);

        assert_ne!(
            queue_ptr,
            null_mut(),
            "queue_ptr is null, allocation failed"
        );
        assert_ne!(pointer, null_mut(), "dev is null, allocation failed");

        VIRTIO_BLOCK_DEVICE.queue = queue_ptr;
        VIRTIO_BLOCK_DEVICE.dev = pointer;
        VIRTIO_BLOCK_DEVICE.idx = 0;

        // 8. Set the DRIVER_OK status bit. At this point the device is “live”.
        current_status_bit |= virtio::StatusField::DriverOk as u32;
        pointer
            .add(virtio::MmioOffset::Status as usize / 4)
            .write_volatile(current_status_bit);

        // Now we can use the virtio block driver
        true
    }
}

// The header/data/status is a block request
// packet. We send the header to tell the direction
// (blktype: IN/OUT) and then the starting sector
// we want to read. Then, we put the data buffer
// as the Data structure and finally an 8-bit
// status. The device will write one of three values
// in here: 0 = success, 1 = io error, 2 = unsupported
// operation.
#[repr(C)]
pub struct Header {
    blktype: u32,
    reserved: u32,
    sector: u64,
}

#[repr(C)]
pub struct Data {
    data: *mut u8,
}

#[repr(C)]
pub struct Status {
    status: u8,
}

#[repr(C)]
pub struct Request {
    header: Header,
    data: Data,
    status: Status,
    head: u16,
}

fn is_next_flag_set(flag: u16) -> bool {
    flag & VIRTIO_DESC_F_NEXT != 0
}

pub fn fill_next_descriptor(desc: Descriptor) -> u16 {
    unsafe {
        let current_idx: u16 = VIRTIO_BLOCK_DEVICE.idx;
        let next_idx = (current_idx + 1) % VIRTIO_RING_SIZE as u16;

        (*VIRTIO_BLOCK_DEVICE.queue).desc[current_idx as usize] = desc;
        if is_next_flag_set((*VIRTIO_BLOCK_DEVICE.queue).desc[current_idx as usize].flags) {
            // If the next flag is set, we need another descriptor.
            (*VIRTIO_BLOCK_DEVICE.queue).desc[current_idx as usize].next = next_idx;
        }

        VIRTIO_BLOCK_DEVICE.idx = next_idx as u16;

        current_idx
    }
}

pub const VIRTIO_BLK_T_IN: u32 = 0;
pub const BUFFER_LEN: usize = 512;

pub unsafe fn read_block_device(_sector_idx: usize) -> [u8; BUFFER_LEN] {
    // Safety assertions
    assert_ne!(
        VIRTIO_BLOCK_DEVICE.queue,
        null_mut(),
        "It seems the block driver is not initalized"
    );

    let block_request_size = size_of::<Request>();
    assert!(
        block_request_size < 4096,
        "Block request size needs more space"
    );

    //----------- The block request  ---------------//

    // TODO: Implement better allocation here
    let block_request = alloc(1) as *mut Request;

    let buffer = alloc(1) as *mut u8;
    (*block_request).header.sector = 0 as u64;
    // VIRTIO_BLK_T_IN -> block read
    (*block_request).header.blktype = VIRTIO_BLK_T_IN;
    (*block_request).data.data = buffer;
    (*block_request).header.reserved = 0;
    (*block_request).status.status = 0;

    //----------- Descriptor ring 1  ---------------//

    let descriptor = Descriptor {
        addr: &(*block_request).header as *const Header as u64,
        len: size_of::<Header>() as u32,
        flags: virtio::VIRTIO_DESC_F_NEXT,
        next: 1,
    };

    let head_idx: usize = fill_next_descriptor(descriptor) as usize;

    //----------- Descriptor ring 2  ---------------//

    let desc = Descriptor {
        addr: buffer as u64,
        len: BUFFER_LEN as u32,
        flags: virtio::VIRTIO_DESC_F_NEXT | virtio::VIRTIO_DESC_F_WRITE,
        next: 2,
    };

    fill_next_descriptor(desc);

    //----------- Descriptor ring 3  ---------------//

    let desc = Descriptor {
        addr: &(*block_request).status as *const Status as u64,
        len: size_of::<Status>() as u32,
        flags: virtio::VIRTIO_DESC_F_WRITE,
        next: 0,
    };

    fill_next_descriptor(desc);

    //----------- Submit to the queue ---------------//

    asm!("sfence.vma");
    (*(VIRTIO_BLOCK_DEVICE.queue)).avail.ring[(*VIRTIO_BLOCK_DEVICE.queue).avail.idx as usize] =
        head_idx as u16;
    (*VIRTIO_BLOCK_DEVICE.queue).avail.idx += 1;
    (*VIRTIO_BLOCK_DEVICE.queue).avail.idx %= VIRTIO_RING_SIZE as u16;
    asm!("sfence.vma");

    VIRTIO_BLOCK_DEVICE
        .dev
        .add(MmioOffset::QueueNotify as usize / 4)
        .write_volatile(0);

    //----------- Notify the device ---------------//

    // TODO: Implement blocking properly here
    for _ in 0..100000 {}

    let mut output: [u8; BUFFER_LEN] = [0; BUFFER_LEN];

    unsafe {
        ptr::copy_nonoverlapping(buffer, output.as_mut_ptr(), BUFFER_LEN);
    }

    // Now we can free the buffer (after copying the data)
    dealloc(buffer);

    output
}
