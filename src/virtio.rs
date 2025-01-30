use crate::block::initialize_block_device;
use crate::page_allocator::PAGE_SIZE;
use core::fmt::Write;

/* ------------ */

// VirtIO structures

pub const VIRTIO_F_RING_INDIRECT_DESC: u32 = 28;
pub const VIRTIO_F_RING_EVENT_IDX: u32 = 29;
pub const VIRTIO_F_VERSION_1: u32 = 32;

pub const VIRTIO_DESC_F_NEXT: u16 = 1;
pub const VIRTIO_DESC_F_WRITE: u16 = 2;
pub const VIRTIO_DESC_F_INDIRECT: u16 = 4;

pub const VIRTIO_AVAIL_F_NO_INTERRUPT: u16 = 1;

pub const VIRTIO_USED_F_NO_NOTIFY: u16 = 1;

// According to the documentation, this must be a power
// of 2 for the new style. So, I'm changing this to use
// 1 << instead because that will enforce this standard.
pub const VIRTIO_RING_SIZE: usize = 1 << 7;

// The descriptor holds the data that we need to send to
// the device. The address is a physical address and NOT
// a virtual address. The len is in bytes and the flags are
// specified above. Any descriptor can be chained, hence the
// next field, but only if the F_NEXT flag is specified.
#[repr(C)]
pub struct Descriptor {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

#[repr(C)]
pub struct Available {
    pub flags: u16,
    pub idx: u16,
    pub ring: [u16; VIRTIO_RING_SIZE],
    pub event: u16,
}

#[repr(C)]
pub struct UsedElem {
    pub id: u32,
    pub len: u32,
}

#[repr(C)]
pub struct Used {
    pub flags: u16,
    pub idx: u16,
    pub ring: [UsedElem; VIRTIO_RING_SIZE],
    pub event: u16,
}

#[repr(C)]
pub struct Queue {
    pub desc: [Descriptor; VIRTIO_RING_SIZE],
    pub avail: Available,
    // Calculating padding, we need the used ring to start on a page boundary. We take the page size, subtract the
    // amount the descriptor ring takes then subtract the available structure and ring.
    pub padding0:
        [u8; PAGE_SIZE - size_of::<Descriptor>() * VIRTIO_RING_SIZE - size_of::<Available>()],
    pub used: Used,
}

const MMIO_BASE: usize = 0x100001000;
const MMIO_END: usize = 0x10008000;
const MMIO_STRIDE: usize = 0x1000;
const MMIO_MAGIC: u32 = 0x74726967;

#[allow(dead_code)]
pub enum MmioOffset {
    MagicValue = 0x0,
    Version = 0x4,
    DeviceId = 0x8,
    VendorId = 0xc,
    DeviceFeatures = 0x10,
    DriverFeatures = 0x20,
    GuestPageSize = 0x28,
    QueueSel = 0x30,
    QueueNumMax = 0x34,
    QueueNum = 0x38,
    QueueAlign = 0x3c,
    QueuePfn = 0x40,
    QueueReady = 0x44,
    QueueNotify = 0x50,
    InterruptStatus = 0x60,
    InterruptAck = 0x64,
    Status = 0x70,
}

impl From<MmioOffset> for usize {
    fn from(device: MmioOffset) -> Self {
        device as usize
    }
}

enum DeviceType {
    NoDevice = 0x0,
    Network = 0x1,
    Disk = 0x2,
    Entropy = 0x4,
    Unknown = 0x5,
}

pub enum StatusField {
    Acknowledge = 1,
    Driver = 2,
    DriverOk = 4,
    FeaturesOk = 8,
    DeviceNeedsReset = 64,
    Failed = 128,
}

impl DeviceType {
    pub fn new(value: u32) -> Self {
        match value {
            0x0 => DeviceType::NoDevice,
            0x1 => DeviceType::Network,
            0x2 => DeviceType::Disk,
            0x4 => DeviceType::Entropy,
            _ => DeviceType::Unknown,
        }
    }
}

pub fn init() {
    println!("Hello");
    for addr in (MMIO_BASE..=MMIO_END).step_by(MMIO_STRIDE) {
        println!("test");
        let magic_value: u32;
        let device_id: u32;

        let ptr = addr as *mut u32;

        unsafe {
            magic_value = ptr.add(MmioOffset::MagicValue.into()).read_volatile();
            device_id = ptr.add(MmioOffset::DeviceId.into()).read_volatile();
        }

        if magic_value == MMIO_MAGIC {

            println!("WTF");
            match DeviceType::new(device_id) {
                DeviceType::NoDevice => {
                    println!("No device")
                }
                DeviceType::Network => {
                    println!("Network device")
                }
                DeviceType::Disk => {

                    initialize_block_device(ptr);
                    println!("Disk device")
                }
                DeviceType::Entropy => {
                    println!("Entropy device")
                }
                DeviceType::Unknown => {
                    println!("Unknown device")
                }
            }
        }
    }
}

pub fn init_sanity_check() {
    // Nothing to do at the moment
}
