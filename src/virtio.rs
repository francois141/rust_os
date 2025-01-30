use crate::block::{initialize_block_device, read_block_device};
use crate::page_allocator::PAGE_SIZE;
use core::fmt::Write;

pub const VIRTIO_DESC_F_NEXT: u16 = 1;
pub const VIRTIO_DESC_F_WRITE: u16 = 2;

pub const VIRTIO_RING_SIZE: usize = 1 << 7;

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
    // Required by the documentation to ensure we have a proper alignment
    pub padding0:
        [u8; PAGE_SIZE - size_of::<Descriptor>() * VIRTIO_RING_SIZE - size_of::<Available>()],
    pub used: Used,
}

const MMIO_BASE: usize = 0x10001000;
const MMIO_END: usize = 0x10008000;
const MMIO_STRIDE: usize = 0x1000;
const MMIO_MAGIC: u32 = 0x74726976;

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
    for addr in (MMIO_BASE..=MMIO_END).step_by(MMIO_STRIDE) {
        let magic_value: u32;
        let device_id: u32;
        let version_id: u32;
        let vendor_id: u32;

        let ptr = addr as *mut u32;

        unsafe {
            magic_value = ptr.add(MmioOffset::MagicValue as usize / 4).read_volatile();
            device_id = ptr.add(MmioOffset::DeviceId as usize / 4).read_volatile();
            version_id = ptr.add(MmioOffset::Version as usize / 4).read_volatile();
            vendor_id = ptr.add(MmioOffset::VendorId as usize / 4).read_volatile();
        }

        if magic_value == MMIO_MAGIC {
            match DeviceType::new(device_id) {
                DeviceType::NoDevice => {}
                DeviceType::Network => {
                    println!("Network device found")
                }
                DeviceType::Disk => {
                    println!("Disk device found");
                    assert_eq!(version_id, 1);
                    assert_eq!(vendor_id, 0x554d4551);
                    initialize_block_device(ptr);
                }
                DeviceType::Entropy => {
                    println!("Entropy device found")
                }
                DeviceType::Unknown => {
                    panic!("Unknown device found")
                }
            }
        }
    }
}

pub fn init_sanity_check() {
    let output = unsafe { read_block_device(0) };

    assert_eq!(output[3], 97, "must be equal 97");
}
