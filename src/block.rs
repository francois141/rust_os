use crate::page_allocator::{alloc, PAGE_SIZE};
use crate::virtio;

use crate::virtio::{Queue, VIRTIO_RING_SIZE};
use core::fmt::Write;
use core::ptr::null_mut;

pub fn init() {}

pub fn init_sanity_check() {
    // Nothing to do for the moment here
}

// TODO: Create a global block device here

const VIRTIO_BLK_FEATURE_READ_ONLY: u32 = 5;

// Internal block device structure
// We keep our own used_idx and idx for
// descriptors. There is a shared index, but that
// tells us or the device if we've kept up with where
// we are for the available (us) or used (device) ring.
pub struct BlockDevice {
    queue: *mut Queue,
    dev: *mut u32,
    idx: u16,
    ack_used_idx: u16,
}

pub static mut BLOCK_DEVICE: BlockDevice = BlockDevice {
    queue: null_mut(),
    dev: null_mut(),
    idx: 0,
    ack_used_idx: 0,
};

pub fn initialize_block_device(pointer: *mut u32) -> bool {
    // TODO: What is this status bit doing
    let mut current_status_bit = virtio::StatusField::Acknowledge as u32;

    unsafe {
        // 1. Reset the device.
        pointer
            .add(virtio::MmioOffset::Status as usize / 4)
            .write_volatile(0);
        // 2. Set the ACKNOWLEDGE status bit: the guest OS has noticed the device.
        pointer
            .add(virtio::MmioOffset::Status as usize / 4)
            .write_volatile(virtio::StatusField::Acknowledge as u32);
        // 3. Set the DRIVER status bit: the guest OS knows how to drive the device.
        current_status_bit |= virtio::StatusField::DriverOk as u32;
        pointer
            .add(virtio::MmioOffset::Status as usize / 4)
            .write_volatile(current_status_bit);
        // 4. Read device feature bits, and write the subset of feature bits understood by the OS and driver to the device. During this step the driver MAY read (but MUST NOT write) the device-specific configuration fields to check that it can support the device before accepting it.
        let _device_features = pointer
            .add(virtio::MmioOffset::DeviceFeatures as usize / 4)
            .read_volatile();
        // For the moment, we assume the device is read only

        pointer
            .add(virtio::MmioOffset::DriverFeatures as usize / 4)
            .write_volatile(_device_features);
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
            != 0
        {
            print!("features fail...");
            pointer
                .add(virtio::MmioOffset::Status as usize / 4)
                .write_volatile(virtio::StatusField::Failed as u32);
            return false;
        };
        // 7. Perform device-specific setup, including discovery of virtqueues for the device,optional per-bussetup, reading and possibly writing the device’s virtio configuration space, and population of virtqueues.
        let number_queues = pointer
            .add(virtio::MmioOffset::QueueNumMax as usize / 4)
            .read_volatile();
        // This must be a power of two
        if VIRTIO_RING_SIZE > number_queues as usize {
            println!("queue fails...");
            pointer
                .add(virtio::MmioOffset::Status as usize / 4)
                .write_volatile(virtio::StatusField::Failed as u32);
            return false;
        }
        pointer
            .add(virtio::MmioOffset::QueueNum as usize / 4)
            .write_volatile(VIRTIO_RING_SIZE as u32);

        // First, if the block device array is empty, create it!
        // We add 4095 to round this up and then do an integer
        // divide to truncate the decimal. We don't add 4096,
        // because if it is exactly 4096 bytes, we would get two
        // pages, not one.
        let num_pages = (core::mem::size_of::<Queue>() + PAGE_SIZE - 1) / PAGE_SIZE;
        // println!("np = {}", num_pages);
        // We allocate a page for each device. This will the the
        // descriptor where we can communicate with the block
        // device. We will still use an MMIO register (in
        // particular, QueueNotify) to actually tell the device
        // we put something in memory. We also have to be
        // careful with memory ordering. We don't want to
        // issue a notify before all memory writes have
        // finished. We will look at that later, but we need
        // what is called a memory "fence" or barrier.
        pointer
            .add(virtio::MmioOffset::QueueSel as usize / 4)
            .write_volatile(0);
        // Alignment is very important here. This is the memory address
        // alignment between the available and used rings. If this is wrong,
        // then we and the device will refer to different memory addresses
        // and hence get the wrong data in the used ring.
        // ptr.add(MmioOffsets::QueueAlign.scale32()).write_volatile(2);
        let queue_ptr = alloc(num_pages) as *mut Queue;
        let queue_pfn = queue_ptr as u32;
        pointer
            .add(virtio::MmioOffset::GuestPageSize as usize / 4)
            .write_volatile(PAGE_SIZE as u32);
        // QueuePFN is a physical page number, however it
        // appears for QEMU we have to write the entire memory
        // address. This is a physical memory address where we
        // (the OS) and the block device have in common for
        // making and receiving requests.
        pointer
            .add(virtio::MmioOffset::QueuePfn as usize / 4)
            .write_volatile(queue_pfn / PAGE_SIZE as u32);
        // We need to store all of this data as a "BlockDevice"
        // structure We will be referring to this structure when
        // making block requests AND when handling responses.
        BLOCK_DEVICE.queue = queue_ptr;
        BLOCK_DEVICE.dev = pointer;
        BLOCK_DEVICE.idx = 0;
        BLOCK_DEVICE.ack_used_idx = 0;

        // 8. Set the DRIVER_OK status bit. At this point the device is “live”.
        current_status_bit |= virtio::StatusField::DriverOk as u32;
        pointer
            .add(virtio::MmioOffset::Status as usize / 4)
            .write_volatile(current_status_bit);


        println!("Done with initialization of the disk device");


        // Now we can use the virtio block driver
        true
    }
}

pub fn _read_block_device(_sector_idx: usize) -> [u8; 512] {
    // TODO: Implement the read_block_device_function
    [0; 512]
}
