#![no_std]
#![no_main]
//#![feature(min_const_fn)]
//#![feature(const_let)]

// pick a panicking behavior
//extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger


extern crate stm32f0xx_hal as hal;
extern crate stm32f0;
extern crate embedded_hal;

use core::mem::size_of;
use stm32f0::stm32f0x2;

use hal::delay::Delay;
use hal::i2c::*;
use hal::gpio::*;
use hal::prelude::*;

use cortex_m_semihosting::{debug, hprintln};
use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::Peripherals as c_m_Peripherals;
//use cortex_m_rt::{entry, interrupt};
use cortex_m_rt::entry;

//use embedded_hal::blocking::i2c::Write;
use core::fmt::Write;

pub use hal::stm32;
pub use hal::stm32::{interrupt, Interrupt, EXTI, Peripherals, USB, I2C1};
//pub use hal::stm32::*;

use embedded_graphics::fonts::Font6x8;
use embedded_graphics::prelude::*;
use ssd1306::prelude::*;
use ssd1306::Builder;

use core::cell::RefCell;
use core::ops::DerefMut;
mod usb;

use crate::usb::descriptors::*;

// Make our LED globally available
static LED: Mutex<RefCell<Option<gpioa::PA5<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));

// Make our delay provider globally available
static DELAY: Mutex<RefCell<Option<Delay>>> = Mutex::new(RefCell::new(None));

// Make external interrupt registers globally available
static INT: Mutex<RefCell<Option<EXTI>>> = Mutex::new(RefCell::new(None));

// Make USB Driver globally available
static USBDEV: Mutex<RefCell<Option<usb::Usb<USB, (gpioa::PA11<Alternate<AF0>>, gpioa::PA12<Alternate<AF0>>)>>>> = Mutex::new(RefCell::new(None));

const DEV_DESC: Device = Device::new()
                         .iManufacturer(1)
                         .iProduct(2)
                         .iSerialNumber(3)
                         .bNumConfigurations(1);

const DEV_QUAL: DeviceQualifier = DeviceQualifier::new().bcdUSB(0x0200);

const INTERFACE_DESC: Interface = Interface::new().bNumEndpoints(1).iInterface(5);

const EP01_DESC: Endpoint = Endpoint::new().bEndpointAddress(0x01).wMaxPacketSize(64).bInterval(1);

const CONF_DESC: Configuration = Configuration::new()
    .wTotalLength(size_of::<Configuration>() as u16 + size_of::<Interface>() as u16 + size_of::<Endpoint>() as u16 * 1) // add all descs up.
    .bNumInterfaces(1)
    .bConfigurationValue(1)
    .iConfiguration(4)
    .bmAttributes(0b1_1_0_00000) // Self powered no remote wakeup.
    .bMaxPower(0xFA); // 500mA.

const ints: [Interface; 1] = [INTERFACE_DESC];
const eps: [Endpoint; 1] = [EP01_DESC];

const DESCS: usb::Descriptors = usb::Descriptors {
    Device: DEV_DESC,
    Configuration: CONF_DESC,
    Interfaces: &ints,
    Endpoints: &eps
};

#[entry]
fn main() -> ! {
    hprintln!("main()").unwrap();
    if let (Some(p), Some(cp)) = (Peripherals::take(), c_m_Peripherals::take()) {
        let gpioa = p.GPIOA.split();
        let gpiob = p.GPIOB.split();
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
        
        let usb = usb::Usb::usb(p.USB, (dm, dp), DESCS);

        // Configure I2C
        let scl = gpiob.pb8
            .into_alternate_af1()
            .internal_pull_up(true)
            .set_open_drain();
        let sda = gpiob.pb9
            .into_alternate_af1()
            .internal_pull_up(true)
            .set_open_drain();

        let i2c = I2c::i2c1(p.I2C1, (scl, sda), 400.khz());

        // Configure display
        let mut disp: GraphicsMode<_> = Builder::new()
            .with_size(DisplaySize::Display128x32)
            .connect_i2c(i2c).into();

        disp.init().unwrap();
        disp.flush().unwrap();

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

        disp.draw(
            Font6x8::render_str("this is a test")
            .with_stroke(Some(1u8.into()))
            .into_iter(),);


        disp.flush().unwrap();

    }
    loop {
        // your code goes here
    }
}

#[interrupt]
fn USB() {
    //hprintln!("USB_ISR:").unwrap();
    cortex_m::interrupt::free(|cs| {
        if let &mut Some(ref mut usb) = USBDEV.borrow(cs).borrow_mut().deref_mut() {
            usb.interrupt();
        }
    });
}

#[interrupt]
fn EXTI4_15() {
    // Enter critical section
    hprintln!("BUTTON_PRESS").unwrap();
    cortex_m::interrupt::free(|cs| {
        // Obtain all Mutex protected resources
        if let (&mut Some(ref mut led), &mut Some(ref mut delay), &mut Some(ref mut exti)) = (
            LED.borrow(cs).borrow_mut().deref_mut(),
            DELAY.borrow(cs).borrow_mut().deref_mut(),
            INT.borrow(cs).borrow_mut().deref_mut(),
        ) {
            //hprintln!("Borrow OK!").unwrap();
            // Turn on LED
            led.set_high();

            //hprintln!("Led ON OK!").unwrap();
            // Wait a second
            delay.delay_ms(100_u16);

            //hprintln!("Delay OK!").unwrap();
            // Turn off LED
            led.set_low();

            //hprintln!("Led OFF OK!").unwrap();
            // Clear interrupt
            exti.pr.modify(|_, w| w.pif13().set_bit());
        }
    });
}

