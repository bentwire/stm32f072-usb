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
    IN  = 0b1,
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
            _ => None
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Type {
    Standard = 0b00,
    Class    = 0b01,
    Vendor   = 0b10
}

impl Type {
    pub const fn to_bits(self) -> u8 {
        (self as u8) << 5
    }

    // This should be const-able!!!
    pub fn from_bits(bits: u8) -> Self {
        match bits >> 5 {
            0b00 => Type::Standard,
            0b01 => Type::Class,
            0b10 => Type::Vendor,
            //_ => compile_error!("Invalid value for Type") If this were const I could do this...
            _ => panic!("Invalid value for Request Type")
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Destination {
    Device = 0b00,
    Interface = 0b01,
    Endpoint = 0b10,
    Other = 0b11
}

impl Destination {
    pub const fn to_bits(self) -> u8 {
        (self as u8)
    }

    // This should be const-able!!!
    pub fn from_bits(bits: u8) -> Self {
        match bits & 0b11 {
            0b00 => Destination::Device,
            0b01 => Destination::Interface,
            0b10 => Destination::Endpoint,
            0b11 => Destination::Other,
            //_ => compile_error!("Invalid value for Destination") If this were const I could do this...
            _ => panic!("Invalid value for Request Destination")
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum RequestType {
    Request(Direction, Type, Destination)
}

impl RequestType {
//    pub const fn request(&self, request: RequestType) -> u8 {
//        match request {
//            RequestType::Request(direction, rtype, dest) => (direction as u8) << 8 | (rtype as u8) << 5 | (dest as u8) 
//        }
//        //let RequestType::Request(const direction, const rtype, const dest) = request;
//    }

//    pub const fn to_bits(&self) -> u8 {
//        match *self {
//            RequestType::Request(dir, rtype, dest) => dir.to_bits() | rtype.to_bits() | dest.to_bits()
//        }
//    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum UsbRequestType {
    OutStandardDevice    = 0b0_00_00000,
    OutStandardInterface = 0b0_00_00001,
    OutStandardEndpoint  = 0b0_00_00010,
    OutStandardOther     = 0b0_00_00011,
    OutClassDevice       = 0b0_01_00000,
    OutClassInterface    = 0b0_01_00001,
    OutClassEndpoint     = 0b0_01_00010,
    OutClassOther        = 0b0_01_00011,
    OutVendorDevice      = 0b0_10_00000,
    OutVendorInterface   = 0b0_10_00001,
    OutVendorEndpoint    = 0b0_10_00010,
    OutVendorOther       = 0b0_10_00011,

    InStandardDevice    = 0b1_00_00000,
    InStandardInterface = 0b1_00_00001,
    InStandardEndpoint  = 0b1_00_00010,
    InStandardOther     = 0b1_00_00011,
    InClassDevice       = 0b1_01_00000,
    InClassInterface    = 0b1_01_00001,
    InClassEndpoint     = 0b1_01_00010,
    InClassOther        = 0b1_01_00011,
    InVendorDevice      = 0b1_10_00000,
    InVendorInterface   = 0b1_10_00001,
    InVendorEndpoint    = 0b1_10_00010,
    InVendorOther       = 0b1_10_00011,
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
