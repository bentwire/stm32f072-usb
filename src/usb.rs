use crate::usb::pma::PMA;

use hal::gpio::gpioa::{PA11, PA12};
use hal::gpio::{Alternate, AF0};
use hal::prelude::*;

use cortex_m_semihosting::{debug, hprintln};

pub use hal::stm32;
pub use hal::stm32::{USB, RCC, CRS};


//use pma::PMA;
mod pma;
mod constants;
mod usb_ext;

use self::constants::UsbRequest;
use self::usb_ext::UsbEpExt;

#[derive(Debug)]
pub enum UsbState {
    BootReset,
    Reset,
    Initialized,
    Addressed(u8),
}

const MAX_PACKET_SIZE: u32 = 64;

pub struct Usb<USB, PINS> {
    usb: USB,
    pins: PINS,
    state: UsbState,
    pma: &'static mut PMA,
}

pub trait Pins<Usb> {}

// Only pins PA11, PA12, AF is not important, USB takes over the pins.
impl Pins<USB> for (PA11<Alternate<AF0>>, PA12<Alternate<AF0>>) {}

#[derive(Debug)]
pub enum Error {
    INITFAIL,
}

impl<PINS> Usb<USB, PINS> {
    pub fn usb(usb: USB, pins: PINS) -> Self
        where
        PINS: Pins<USB>,
        {
            // NOTE(unsafe) This executes only during initialisation
            let rcc = unsafe { &(*RCC::ptr()) };
            let pma = unsafe { &mut *PMA.get() };
            let crs = unsafe { &(*CRS::ptr()) };


            // Enable USB clock. and Clock recovery
	    rcc.apb1enr.modify(|_, w| w.usbrst().set_bit().crsen().set_bit());	
	    let _ = rcc.apb1enr.read(); // Delay

            // Initialize clock recovery
            // Set autotrim enabled.
            crs.cr.modify(|_, w| w.autotrimen().set_bit());
            // Enable CR
            crs.cr.modify(|_, w| w.cen().set_bit());

            // ENable USB
            usb.cntr.modify(|_, w| w.pdwn().clear_bit());

            // Clear PMA
            pma.zero();


            hprintln!("DELAY").unwrap();

            // Set BTable address to default.
            usb.btable.reset();

            // Set imask
            usb.cntr.modify(|_, w| w
                           .ctrm().set_bit()
                           .wkupm().set_bit()
                           .suspm().set_bit()
                           //.errm().set_bit()
                           //.sofm().set_bit()
                           //.esofm().set_bit()
                           .resetm().set_bit());

            // Take out of reset.
            usb.cntr.modify(|_, w| w.fres().clear_bit());

            // Clear interrupts
            usb.istr.reset();
            
            // Enable
            usb.daddr.modify(|_, w| w.ef().set_bit());

            // Enable pu
            usb.bcdr.modify(|_, w| w.dppu().set_bit());

            let state = UsbState::BootReset;
            
            Usb {
                usb, pins, state, pma
            }
        }

    fn reset(&mut self) {
        // Init EP0
        self.pma.pma_area.set_u16(0, 0x40); // ADDR0_TX, buffer at offset 0x40 in PMA.
        self.pma.pma_area.set_u16(2, 0);    // COUNT0_TX, 0 bytes in buffer
        self.pma.pma_area.set_u16(4, 0x20); // ADDR0_RX, buffer at offset 0x20 in PMA.
        self.pma.pma_area.set_u16(6, (0x8000 | ((MAX_PACKET_SIZE / 32) - 1) << 10) as u16); // COUNT0_RX, Set buffer count.

        self.usb.ep0r.write(|w| unsafe {
            w.ep_type().bits(0b01) // Ctrl endpoint
                .stat_tx().bits(0b10) // NAK
                .stat_rx().bits(0b11) // VALID
        });

        self.usb.daddr.write(|w| w.ef().set_bit());

        self.state = UsbState::Reset;

        //hprintln!("USB RESET COMPLETE").unwrap();
    }

    fn do_work(&mut self) {
        if self.usb.istr.read().dir().bit_is_set() {
            self.rx()
        } else {
            self.tx()
        }
    }

    fn rx(&mut self) {
        let request16 = self.pma.pma_area.get_u16(0x20); // First u16 in RX buffer 
        let value = self.pma.pma_area.get_u16(0x22);   // Second u16 in RX buffer
        let index = self.pma.pma_area.get_u16(0x24);   // Third...
        let length = self.pma.pma_area.get_u16(0x26);  // Fourth...

        //hprintln!("request16: {:x}, value: {:x}, index: {:x}, length: {:x}", request16, value, index, length).unwrap();
        // set COUNT0_RX to max acceptable size.
        self.pma.pma_area.set_u16(6, (0x8000 | ((MAX_PACKET_SIZE / 32) - 1) << 10) as u16);

        let request_type = (request16 & 0xff) as u8;
        let request = UsbRequest::from(((request16 & 0xff00) >> 8) as u8);

        match (request_type, request) {
            (0x00, UsbRequest::GetStatus) => {
                self.usb.ep0r.toggle_tx_stall();
                //hprintln!("GET STATUS: {:x}", self.usb.ep0r.read().bits() as u16).unwrap();
            }

            (0x00, UsbRequest::SetAddress) => {

                hprintln!("Set Address: {:x}", value as u8);
                self.usb
                .daddr
                .modify(|_, w| unsafe { w.add().bits(value as u8) });

                self.usb.ep0r.toggle_0();
            }

            // Fall though
            (_, _) => {
                hprintln!("RequestType: {:x}, Request: {:x}", request_type, request16).unwrap();
            }
        }

    }

    fn tx(&mut self) {

        hprintln!("TX");
        //self.pma.pma_area.set_u16(6, 0); // Set COUNT0_RX to 0.
        //self.usb.ep0r.toggle_tx_out();
    }

    pub fn interrupt(&mut self) {
        let istr = self.usb.istr.read();
        let istr_val: u32 = istr.bits();

        //hprintln!("ISTR: {:x}", istr_val).unwrap();
        if istr.reset().bit_is_set() {
            // Clear reset bit
            self.usb.istr.modify(|_, w| w.reset().clear_bit());

            // Execute reset
            self.reset();
        }

        if istr.err().bit_is_set() {
            self.usb.istr.modify(|_, w| w.err().clear_bit());
            return;
        }
        
        // Ignore these for now...
        self.usb.istr.modify(|_, w| w.susp().clear_bit().sof().clear_bit().esof().clear_bit());

        let istr = self.usb.istr.read();

        // As long as ctr is set, do work.
        if istr.ctr().bit_is_set() {
            //hprintln!("ISTR: {:x} EP0R: {:x}", istr.bits() as u16, self.usb.ep0r.read().bits() as u16).unwrap();
            let ep = istr.ep_id().bits();
            let dir = istr.dir().bit_is_set();
           
            if ep == 0 {
                if dir {
                    //self.rx();
                    //self.usb.ep0r.write(|w| w.ctr_rx().clear_bit());
                    // Setup packet?
                    if self.usb.ep0r.read().setup().bit_is_set() {
                        let rx_count =  self.pma.pma_area.get_u16(6) & 0x03FF;
                        //self.usb.ep0r.write(|w| w.ctr_rx().clear_bit());
                        //hprintln!("E0: {:x}", self.usb.ep0r.read().bits()).unwrap();
                        self.usb.ep0r.toggle_tx_stall();
                        //self.usb.ep0r.toggle_rx();
                        self.usb.ep0r.clear_ctr_rx();
                        //hprintln!("E0: {:x}", self.usb.ep0r.read().bits()).unwrap();
                        //hprintln!("S! {}", rx_count).unwrap();

                    } else if self.usb.ep0r.read().ctr_rx().bit_is_set() {
                        self.usb.ep0r.write(|w| w.ctr_rx().clear_bit());
                        let rx_count =  self.pma.pma_area.get_u16(6) & 0x03FF;
                        hprintln!("DATA OUT! {}", rx_count).unwrap();
                    }
                } else {
                    self.tx();
                    //self.usb.ep0r.write(|w| w.ctr_tx().clear_bit());
                }
            }
            //hprintln!("EP: {}", ep);
            //self.do_work();
        }
    }
}

