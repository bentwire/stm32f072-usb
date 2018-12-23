#![allow(non_snake_case)]
#![allow(dead_code)]

extern crate vcell;

use self::vcell::VolatileCell;
use bare_metal::Peripheral;
use core::mem::size_of;
use core::ops::Deref;

// TODO: make this take-able? or at least move into the main usb part
// RM0091 30.6.2
// The first packet memory location is located at 0x4000 6000. The buffer descriptor table
// entry associated with the USB_EPnR registers is described below. The packet memory
// should be accessed only by byte (8-bit) or half-word (16-bit) accesses. Word (32-bit)
// accesses are not allowed.
pub const PMA: Peripheral<PMA> = unsafe { Peripheral::new(0x4000_6000) };
pub const PMA_SIZE: usize = 1024; // Size in bytes.

//const BTABLE: usize = 0;

pub struct PMA {
    pub pma_area: PMA_Area,
}

impl PMA {
    pub fn zero(&mut self) {
        for i in 0..PMA_SIZE {
            self.pma_area.set_u8(i, 0);
        }
    }
}

impl Deref for PMA {
    type Target = PMA_Area;
    fn deref(&self) -> &PMA_Area {
        &self.pma_area
    }
}

// RM0091 30.6.2
// The first packet memory location is located at 0x4000 6000. The buffer descriptor table
// entry associated with the USB_EPnR registers is described below. The packet memory
// should be accessed only by byte (8-bit) or half-word (16-bit) accesses. Word (32-bit)
// accesses are not allowed.

#[repr(C)]
pub struct PMA_Area {
    bytes: [VolatileCell<u8>; PMA_SIZE],
    //    words: [VolatileCell<u16>; PMA_SIZE / 2],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct USB_EpBufferDescriptor {
    ADDR_TX: u16,  // Offset in to PMA where packet buffer resides.
    COUNT_TX: u16, // Bytes to be transmitted
    ADDR_RX: u16,  // Offset in to PMA where packet buffer resides.
    COUNT_RX: u16, // BLSIZE, NUM_BLOCK[4:0], COUNT_RX[9:0] 0bx_xxxxx_xxxxxxxxxx
}

// TODO - USB_DoubleBuffer{Tx,Rx}Descriptor

impl PMA_Area {
    //LSB first...
    pub fn get_u16(&self, offset: usize) -> u16 {
        //self.words[offset/2].get()
        (self.bytes[offset].get() as u16) | (((self.bytes[offset + 1].get() as u16) << 8) & 0xff00)
    }

    // LSB first...
    pub fn set_u16(&self, offset: usize, val: u16) {
        self.bytes[offset].set((val & 0x00ff) as u8);
        self.bytes[offset + 1].set((val >> 8 & 0x00ff) as u8);
    }

    pub fn get_u8(&self, offset: usize) -> u8 {
        self.bytes[offset].get()
    }

    pub fn set_u8(&self, offset: usize, val: u8) {
        self.bytes[offset].set(val);
    }

    pub fn borrow_slice(&self, offset: usize, size: usize) -> &[VolatileCell<u8>] {
        &self.bytes[offset..size]
    }

    pub fn get_buffer_descriptor(&self, offset: usize) -> &USB_EpBufferDescriptor {
        unsafe { &*((&self.bytes as *const VolatileCell<u8>) as *const USB_EpBufferDescriptor) }
    }

    pub fn get_buffer_descriptor2(&self, offset: usize) -> &USB_EpBufferDescriptor {
        let slice = self.borrow_slice(offset, size_of::<USB_EpBufferDescriptor>());
        unsafe { &*((slice as *const [VolatileCell<u8>]) as *const USB_EpBufferDescriptor) }
    }

    pub fn write_buffer_u8(&self, offset: usize, buf: &[u8]) {
        for (off, val) in buf.iter().enumerate() {
            self.set_u8(offset + off, *val);
        }
    }
}
