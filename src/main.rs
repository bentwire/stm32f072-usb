#![no_std]
#![no_main]

// pick a panicking behavior
//extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

#[macro_use]
extern crate stm32f0xx_hal as hal;
extern crate stm32f0;

use core::mem::size_of;
use stm32f0::stm32f0x2;

use hal::delay::Delay;
use hal::gpio::*;
use hal::prelude::*;

use cortex_m_semihosting::{debug, hprintln};
use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::Peripherals as c_m_Peripherals;
use cortex_m_rt::entry;

pub use hal::stm32;
pub use hal::stm32::{Interrupt, EXTI, Peripherals, USB};

use core::cell::RefCell;
use core::ops::DerefMut;
mod usb;

// Make our LED globally available
static LED: Mutex<RefCell<Option<gpioa::PA5<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));

// Make our delay provider globally available
static DELAY: Mutex<RefCell<Option<Delay>>> = Mutex::new(RefCell::new(None));

// Make external interrupt registers globally available
static INT: Mutex<RefCell<Option<EXTI>>> = Mutex::new(RefCell::new(None));

// Make USB Driver globally available
static USBDEV: Mutex<RefCell<Option<usb::Usb<USB, (gpioa::PA11<Alternate<AF0>>, gpioa::PA12<Alternate<AF0>>)>>>> = Mutex::new(RefCell::new(None));

//const DEV_DESC : usb::descriptors::Device = usb::descriptors::Device {
//    bLength: size_of::<usb::descriptors::Device>() as u8,
//    bDescriptorType: usb::constants::UsbDescriptorType::Device as u8,
//    bcdUSB: 0x0200,
//    bDeviceClass: 0x00,
//    bDeviceSubClass: 0x00,
//    bDeviceProtocol: 0x00,
//    bMaxPacketSize0: 0x40, // 64 bytes
//    idVendor: 1155,
//    idProduct: 49389,
//    bcdDevice: 0x0200,
//    iManufacturer: 0x00,
//    iProduct: 0x00,
//    iSerialNumber: 0x00,
//    bNumConfigurations: 0x01,
//};

const DEV_QUAL: usb::descriptors::DeviceQualifier = usb::descriptors::DeviceQualifier::new().bcdUSB(0x0000);

#[entry]
fn main() -> ! {
    hprintln!("main()").unwrap();
    if let (Some(p), Some(cp)) = (Peripherals::take(), c_m_Peripherals::take()) {
        let gpioa = p.GPIOA.split();
        let gpioc = p.GPIOC.split();
        let syscfg = p.SYSCFG_COMP;
        let exti = p.EXTI;
        let rcc = p.RCC;

        hprintln!("{:?}", DEV_QUAL).unwrap();
        // Set HSI48 as clock source. Both prescalers to /1.
        rcc.cfgr.modify(|_, w| unsafe { w.sw().bits(0b11)
                                        .ppre().bits(0) 
                                        .hpre().bits(0)});

        // Enable clock for SYSCFG
        rcc.apb2enr.modify(|_, w| w.syscfgen().set_bit());

        // Configure PC13 as input (button)
        let _ = gpioc.pc13.into_pull_down_input();

        // Configure PA5 as output (LED)
        let mut led = gpioa.pa5.into_push_pull_output();

        // Turn off LED
        led.set_low();

        // Configure clock to 48 MHz and freeze it
        let clocks = rcc.constrain().cfgr.sysclk(48.mhz())
                                            .hclk(48.mhz())
                                            .pclk(48.mhz())
                                            .freeze();
        hprintln!("sysclk: {}", clocks.sysclk().0).unwrap();
        hprintln!("hclk: {}", clocks.hclk().0).unwrap();
        hprintln!("pclk: {}", clocks.pclk().0).unwrap();
        // Initialise delay provider
        let mut delay = Delay::new(cp.SYST, clocks);

        // Enable external interrupt for PC13
        syscfg
            .syscfg_exticr4
            .modify(|_, w| unsafe { w.exti13().bits(0b_010) });

        // Set interrupt request mask for line 13
        exti.imr.modify(|_, w| w.mr13().set_bit());

        // Set interrupt falling trigger for line 13
        exti.ftsr.modify(|_, w| w.tr13().set_bit());

        let dm = gpioa.pa11.into_alternate_af0();
        let dp = gpioa.pa12.into_alternate_af0();
        
        let mut usb = usb::Usb::usb(p.USB, (dm, dp));

        // Move control over LED and DELAY and EXTI into global mutexes
        cortex_m::interrupt::free(move |cs| {
            *LED.borrow(cs).borrow_mut() = Some(led);
            *DELAY.borrow(cs).borrow_mut() = Some(delay);
            *INT.borrow(cs).borrow_mut() = Some(exti);
            *USBDEV.borrow(cs).borrow_mut() = Some(usb);
        });

        // Enable EXTI IRQ, set prio 1 and clear any pending IRQs
        let mut nvic = cp.NVIC;
        nvic.enable(Interrupt::EXTI4_15);
        nvic.enable(Interrupt::USB);
        unsafe { nvic.set_priority(Interrupt::EXTI4_15, 0) };
        cortex_m::peripheral::NVIC::unpend(Interrupt::EXTI4_15);
        
        hprintln!("init complete.").unwrap();

    }
    loop {
        // your code goes here
    }

}

/* Define an intterupt handler, i.e. function to call when exception occurs. Here if our external
 * interrupt trips the flash function which will be called */
interrupt!(EXTI4_15, button_press);
interrupt!(USB, usb_isr);

fn usb_isr() {
    //hprintln!("USB_ISR:").unwrap();
    cortex_m::interrupt::free(|cs| {
        if let (&mut Some(ref mut usb)) = (USBDEV.borrow(cs).borrow_mut().deref_mut()) {
            usb.interrupt();
        }
    });
}

fn button_press() {
    // Enter critical section
    hprintln!("BUTTON_PRESS").unwrap();
    cortex_m::interrupt::free(|cs| {
        // Obtain all Mutex protected resources
        if let (&mut Some(ref mut led), &mut Some(ref mut delay), &mut Some(ref mut exti)) = (
            LED.borrow(cs).borrow_mut().deref_mut(),
            DELAY.borrow(cs).borrow_mut().deref_mut(),
            INT.borrow(cs).borrow_mut().deref_mut(),
        ) {
            hprintln!("Borrow OK!").unwrap();
            // Turn on LED
            led.set_high();

            hprintln!("Led ON OK!").unwrap();
            // Wait a second
            delay.delay_ms(1_u16);

            hprintln!("Delay OK!").unwrap();
            // Turn off LED
            led.set_low();

            hprintln!("Led OFF OK!").unwrap();
            // Clear interrupt
            exti.pr.modify(|_, w| w.pif13().set_bit());
        }
    });
}

