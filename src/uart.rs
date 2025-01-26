use core::fmt::Error;
use core::fmt::Write;

static RECEIVER_OFFSET: usize = 0;
static TRANSMITTER_OFFSET: usize = 0;
static INTERRUPT_ENABLE_REGISTER_OFFSET: usize = 1;
static FIFO_CONTROL_REGISTER_OFFSET: usize = 2;
static LINE_CONTROL_REGISTER_OFFSET: usize = 3;
static LINE_STATUS_REGISTER_OFFSET: usize = 5;

static DATA_READY_MASK: u8 = 0x1;

pub static UART_BASE_ADDRESS: usize = 0x1000_0000;

pub struct Uart {
    base_address: usize,
}

impl Write for Uart {
    fn write_str(&mut self, out: &str) -> Result<(), Error> {
        for c in out.bytes() {
            self.write(c);
        }
        Ok(())
    }
}

impl Uart {
    pub fn get() -> Self {
        Uart {
            base_address: 0x1000_0000,
        }
    }

    pub fn start_driver(base_address: usize) -> u8 {
        let pointer = base_address as *mut u8;

        unsafe {
            // Set word length
            pointer
                .add(LINE_CONTROL_REGISTER_OFFSET)
                .write_volatile(0x2);

            // Enable fifo
            pointer
                .add(FIFO_CONTROL_REGISTER_OFFSET)
                .write_volatile(0x1);

            // Enable receiver buffer interrups
            pointer
                .add(INTERRUPT_ENABLE_REGISTER_OFFSET)
                .write_volatile(0x1);
        }

        0
    }

    pub fn read(&mut self) -> Option<u8> {
        let pointer = self.base_address as *mut u8;

        unsafe {
            if pointer.add(LINE_STATUS_REGISTER_OFFSET).read_volatile() & DATA_READY_MASK == 0 {
                None
            } else {
                Some(pointer.add(RECEIVER_OFFSET).read_volatile())
            }
        }
    }

    pub fn write(&mut self, payload: u8) {
        let pointer = self.base_address as *mut u8;

        unsafe {
            pointer.add(TRANSMITTER_OFFSET).write_volatile(payload);
        }
    }
}
