#![allow(dead_code)]

use core::mem::transmute;

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum UsbRequest {
    GetStatus = 0x00,
    ClearFeature = 0x01,
    Two = 0x2,
    SetFeature = 0x03,
    SetAddress = 0x05,
    GetDescriptor = 0x06,
    SetDescriptor = 0x07,
    GetConfiguration = 0x08,
    SetConfiguration = 0x09,
    GetInterface = 0x0A,
    SetInterface = 0x0B,
    SynchFrame = 0x0C,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Direction {
    OUT = 0b0,
    IN = 0b1,
}

impl Direction {
    pub const fn to_bits(self) -> u8 {
        (self as u8) << 7
    }

    // This should be const-able!!!
    pub fn from_bits(bits: u8) -> Option<Self> {
        match (bits >> 7) & 0x01 {
            0b0 => Some(Direction::OUT),
            0b1 => Some(Direction::IN),
            _ => None,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Type {
    Standard = 0b00,
    Class = 0b01,
    Vendor = 0b10,
}

impl Type {
    pub const fn to_bits(self) -> u8 {
        (self as u8) << 5
    }

    pub fn from_bits(bits: u8) -> Option<Self> {
        match (bits >> 5) & 0x03 {
            0b00 => Some(Type::Standard),
            0b01 => Some(Type::Class),
            0b10 => Some(Type::Vendor),
            _ => None,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Destination {
    Device = 0b00,
    Interface = 0b01,
    Endpoint = 0b10,
    Other = 0b11,
}

impl Destination {
    pub const fn to_bits(self) -> u8 {
        (self as u8)
    }

    pub fn from_bits(bits: u8) -> Option<Self> {
        match bits & 0b11 {
            0b00 => Some(Destination::Device),
            0b01 => Some(Destination::Interface),
            0b10 => Some(Destination::Endpoint),
            0b11 => Some(Destination::Other),
            _ => None,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum UsbRequestType {
    OutStandardDevice = 0b0_00_00000,
    OutStandardInterface = 0b0_00_00001,
    OutStandardEndpoint = 0b0_00_00010,
    OutStandardOther = 0b0_00_00011,
    OutClassDevice = 0b0_01_00000,
    OutClassInterface = 0b0_01_00001,
    OutClassEndpoint = 0b0_01_00010,
    OutClassOther = 0b0_01_00011,
    OutVendorDevice = 0b0_10_00000,
    OutVendorInterface = 0b0_10_00001,
    OutVendorEndpoint = 0b0_10_00010,
    OutVendorOther = 0b0_10_00011,

    InStandardDevice = 0b1_00_00000,
    InStandardInterface = 0b1_00_00001,
    InStandardEndpoint = 0b1_00_00010,
    InStandardOther = 0b1_00_00011,
    InClassDevice = 0b1_01_00000,
    InClassInterface = 0b1_01_00001,
    InClassEndpoint = 0b1_01_00010,
    InClassOther = 0b1_01_00011,
    InVendorDevice = 0b1_10_00000,
    InVendorInterface = 0b1_10_00001,
    InVendorEndpoint = 0b1_10_00010,
    InVendorOther = 0b1_10_00011,
}

impl From<u8> for UsbRequest {
    #[inline]
    fn from(b: u8) -> Self {
        unsafe { transmute(b) }
    }
}

impl From<u8> for UsbRequestType {
    #[inline]
    fn from(b: u8) -> Self {
        unsafe { transmute(b) }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum UsbDescriptorType {
    Device = 1,
    Configuration = 2,
    StringDesc = 3,
    Interface = 4,
    Endpoint = 5,
    DeviceQualifier = 6,
    OtherSpeedConfiguration = 7,
    Debug = 0x0A,
    Bos = 0x0F,
    Hid = 0x21,
    HidReport = 0x22,
}

impl From<u8> for UsbDescriptorType {
    #[inline]
    fn from(b: u8) -> Self {
        unsafe { transmute(b) }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum UsbDeviceState {
    Disabled,
    Attached,
    Powered,
    Reset, // Default in the USB spec, reset makes more sense based on the desc IMO.
    Addressed,
    Configured,
    Suspended,
}
