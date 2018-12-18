#![allow(non_snake_case)]
#![allow(dead_code)]

use core::mem::{size_of, transmute};
use core::slice::*;
use core::marker::Sized;

use crate::usb::constants;

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

impl Default for Device {
    fn default() -> Self {
        Self::new()
    }
}

impl Device {
    pub const fn new() -> Self {
        Self {
            bLength: size_of::<Device>() as u8,
            bDescriptorType: constants::UsbDescriptorType::Device as u8,
            bcdUSB: 0x0200,
            bDeviceClass: 0x00,
            bDeviceSubClass: 0x00,
            bDeviceProtocol: 0x00,
            bMaxPacketSize0: 0x40,
            idVendor: 0xffff,
            idProduct: 0xffff,
            bcdDevice: 0x0200,
            iManufacturer: 0x00,
            iProduct: 0x00,
            iSerialNumber: 0x00,
            bNumConfigurations: 0x01,
        }
    }

    pub const fn bcdUSB(&self, bcdUSB: u16) -> Self {
        Self {
            bcdUSB,
            ..*self
        }
    }

    pub const fn bDeviceClass(&self, bDeviceClass: u8) -> Self {
        Self {
            bDeviceClass,
            ..*self
        }
    }

    pub const fn bDeviceSubClass(&self, bDeviceSubClass: u8) -> Self {
        Self {
            bDeviceSubClass,
            ..*self
        }
    }

    pub const fn bDeviceProtocol(&self, bDeviceProtocol: u8) -> Self {
        Self {
            bDeviceProtocol,
            ..*self
        }
    }

    pub const fn bMaxPacketSize0(&self, bMaxPacketSize0: u8) -> Self {
        Self {
            bMaxPacketSize0,
            ..*self
        }
    }

    pub const fn idVendor(&self, idVendor: u16) -> Self {
        Self {
            idVendor,
            ..*self
        }
    }

    pub const fn idProduct(&self, idProduct: u16) -> Self {
        Self {
            idProduct,
            ..*self
        }
    }

    pub const fn iSerialNumber(&self, iSerialNumber: u8) -> Self {
        Self {
            iSerialNumber,
            ..*self
        }
    }

    pub const fn bNumConfigurations(&self, bNumConfigurations: u8) -> Self {
        Self {
            bNumConfigurations,
            ..*self
        }
    }
}

//impl From<[u8; size_of::<Device>()]> for Device {
//    #[inline]
//    fn from(b: [u8; size_of::<Device>()]) -> Self {
//        Self {
//            
//        }
//        //unsafe { transmute(b) }
//    }
//}

//unsafe fn as_u8_arry<T: Sized>(ptr: &T) -> &[u8]
//    where T: Sized {
//    from_raw_parts(
//        (ptr as *const T) as *const u8,
//        size_of::<T>())
//}

//impl From<Device> for &[u8] {
//    #[inline]
//    fn from(a: Device) -> &'static[u8] {
//       unsafe { as_u8_arry(& a) }
//    }
//}

//impl From<Device> for [u8; size_of::<Device>()] {
//    #[inline]
//    fn from(a: Device) -> [u8; size_of::<Device>()] {
//       *as_u8_arry(&a) 
//    }
//}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct DeviceQualifier {
    bLength: u8,
    bDescriptorType: u8,
    bcdUSB: u16,
    bDeviceClass: u8,
    bDeviceSubClass: u8,
    bDeviceProtocol: u8,
    bMaxPacketSize0: u8,
    bNumConfigurations: u8,
    bReserved: u8,
}

impl Default for DeviceQualifier {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceQualifier {
    pub const fn new() -> Self {
        Self {
            bLength: size_of::<DeviceQualifier>() as u8,
            bDescriptorType: constants::UsbDescriptorType::DeviceQualifier as u8,
            bcdUSB: 0x0200,
            bDeviceClass: 0x00,
            bDeviceSubClass: 0x00,
            bDeviceProtocol: 0x00,
            bMaxPacketSize0: 0x40,
            bNumConfigurations: 0x01,
            bReserved: 0x00,
        }
    }

    pub const fn bcdUSB(&self, bcdUSB: u16) -> Self {
        Self {
            bcdUSB,
            ..*self
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct Configuration { // Also other speed configuration.
    bLength: u8,
    bDescriptorType: u8,
    wTotalLength: u16,
    bNumInterfaces: u8,
    bConfigurationValue: u8,
    iConfiguration: u8,
    bmAttributes: u8,
    bMaxPower: u8,
}

impl Default for Configuration {
    fn default() -> Self {
        Self::new()
    }
}

impl Configuration {
    pub const fn new() -> Self {
        Self {
            bLength: size_of::<Configuration>() as u8,
            bDescriptorType: constants::UsbDescriptorType::Configuration as u8,
            wTotalLength: 0x0000,
            bNumInterfaces: 0x00,
            bConfigurationValue: 0x00,
            iConfiguration: 0x00,
            bmAttributes: 0x00,
            bMaxPower: 0x00,
        }
    }

    pub const fn wTotalLength(&self, wTotalLength: u16) -> Self {
        Self {
            wTotalLength,
            ..*self
        }
    }

    pub const fn bNumInterfaces(&self, bNumInterfaces: u8) -> Self {
        Self {
            bNumInterfaces,
            ..*self
        }
    }

    pub const fn bConfigurationValue(&self, bConfigurationValue: u8) -> Self {
        Self {
            bConfigurationValue,
            ..*self
        }
    }

    pub const fn iConfiguration(&self, iConfiguration: u8) -> Self {
        Self {
            iConfiguration,
            ..*self
        }
    }

    pub const fn bmAttributes(&self, bmAttributes: u8) -> Self {
        Self {
            bmAttributes,
            ..*self
        }
    }

    pub const fn bMaxPower(&self, bMaxPower: u8) -> Self {
        Self {
            bMaxPower,
            ..*self
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct Interface { 
    bLength: u8,
    bDescriptorType: u8,
    bInterfaceNumber: u8,
    bAlternateSetting: u8,
    bNumEndpoints: u8,
    bInterfaceClass: u8,
    bInterfaceSubClass: u8,
    bInterfaceProtocol: u8,
    iInterface: u8,
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct Endpoint { 
    bLength: u8,
    bDescriptorType: u8,
    bEndpointAddress: u8,
    bmAttributes: u8,
    wMaxPacketSize: u16,
    bInterval: u8,
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct String0 { 
    bLength: u8,
    bDescriptorType: u8,
    wLANGID: &'static [u16],
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct String { 
    bLength: u8,
    bDescriptorType: u8,
    uString: &'static [u8],
}
