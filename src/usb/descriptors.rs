use core::mem::transmute;

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct Device {
    bLength: u8,
    bDescriptorType: u8,
    bcdUSB: u16,
    bDeviceClass: u8,
    bDeviceSubClass: u8,
    bDeviceProtocol: u8,
    bMaxPacketSize0: u8,
    idVendor: u16,
    idProduct: u16,
    bcdDevice: u16,
    iManufacturer: u8,
    iProduct: u8,
    iSerialNumber: u8,
    bNumConfigurations: u8,
}

impl Device {
    pub const fn bcdUSB(& self, bcdUSB: u16) -> Self {
        self.bcdUSB = bcdUSB;
        *self
        //Device { bcdUSB: bcdUSB }
    }
}
