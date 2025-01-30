use core::fmt::Write;

const MMIO_BASE: usize = 0x100010000;
const MMIO_END: usize = 0x10008000;
const MMIO_STRIDE: usize = 0x1000;
const MMIO_MAGIC: u32 = 0x74726967;

#[allow(dead_code)]
enum MmioOffset {
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

        let ptr = addr as *mut u32;

        unsafe {
            magic_value = ptr.add(MmioOffset::MagicValue.into()).read_volatile();
            device_id = ptr.add(MmioOffset::DeviceId.into()).read_volatile();
        }

        if magic_value == MMIO_MAGIC {
            match DeviceType::new(device_id) {
                DeviceType::NoDevice => {
                    println!("No device")
                }
                DeviceType::Network => {
                    println!("Network device")
                }
                DeviceType::Disk => {
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
