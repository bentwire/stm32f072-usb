#![allow(non_snake_case)]
#![allow(dead_code)]

use core::marker::Sized;
use core::mem::{size_of, transmute};
use core::slice::*;

use crate::usb::constants;
use crate::usb::descriptors;

#[derive(Debug)]
pub struct Device<'a> {
    descriptor: &'a descriptors::Device,
    configurations: &'a [Configuration<'a>],
}

#[derive(Debug, Clone)]
pub struct Configuration<'a> {
    descriptor: &'a descriptors::Configuration,
    interfaces: &'a [Interface<'a>],
}

#[derive(Debug, Copy, Clone)]
pub struct Interface<'a> {
    descriptor: &'a descriptors::Interface,
    other_descriptors: &'a [&'a [u8]],
    endpoints: &'a [Endpoint<'a>],
}

#[derive(Debug, Copy, Clone)]
pub struct Endpoint<'a> {
    descriptor: &'a descriptors::Endpoint,
}
