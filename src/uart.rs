use core::convert::TryInto;
use core::fmt::Write;
use core::fmt::Error;

static receiver_offset: usize = 0;
static transmitter_offset: usize = 0;
static interrupt_enable_register: usize = 1;
static fifo_control_register_offset: usize = 2;
static line_control_register_offset: usize = 3;
static line_status_register_offset: usize = 5;

static data_ready_mask: u8 = 0x1;

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
    pub fn create(base_address: usize) -> Self {
        Uart {
            base_address
        }
    }

    pub fn start_driver(base_address: usize) -> u8 {
        let pointer = base_address as *mut u8;

        unsafe {
            // Set word length
            let line_control_register = 0x2;
            pointer.add(line_control_register_offset).write_volatile(0x2);

            // Enable fifo
            pointer.add(fifo_control_register_offset).write_volatile(0x1);

            // Enable receiver buffer interrups
            pointer.add(interrupt_enable_register).write_volatile(0x1);

            // TODO: DO I need more?
        }

        0
    }

    pub fn read(&mut self) -> Option<u8> {
        let pointer = self.base_address as *mut u8;

        unsafe {
            if pointer.add(line_status_register_offset).read_volatile() & data_ready_mask == 0 {
                Some(pointer.add(receiver_offset).read_volatile())
            } 
            else {
                None
            }
        }
    }

    pub fn write(&mut self, payload: u8) {
        let pointer = self.base_address as *mut u8;

        unsafe {
            pointer.add(transmitter_offset).write_volatile(payload);
        }
    }
}
