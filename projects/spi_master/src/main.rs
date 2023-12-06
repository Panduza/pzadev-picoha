//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin};
use embedded_hal::prelude::*;
use panic_probe as _;

use core::convert::Infallible;

// the interrupt attribute comes from the device crate not from cortex-m-rt
use rp_pico::hal::pac::interrupt;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

// Embed the `Hz` function/trait:
use fugit::RateExtU32;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
    gpio, gpio::bank0::{Gpio2, Gpio3, Gpio4},
    spi,
    usb
};

// USB Device support
use usb_device::{class_prelude::*, prelude::*};
// USB Communications Class Device support
use usbd_serial::SerialPort;

/// The USB Device Driver (shared with the interrupt).
static mut USB_DEVICE: Option<UsbDevice<usb::UsbBus>> = None;
/// The USB Bus Driver (shared with the interrupt).
static mut USB_BUS: Option<UsbBusAllocator<usb::UsbBus>> = None;
/// The USB Serial Device Driver (shared with the interrupt).
static mut USB_SERIAL: Option<SerialPort<usb::UsbBus>> = None;

/// Alias the type for our SPI to make things clearer.
// type SpiPins = (
//     gpio::Pin<Gpio2, gpio::FunctionSpi, gpio::PullNone>,
//     gpio::Pin<<Gpio3 as spi::ValidPinIdTx<pac::SPI0>> , gpio::FunctionSpi, gpio::PullNone>,
//     gpio::Pin<Gpio4, gpio::FunctionSpi, gpio::PullNone>,
// );
// // type SpiPins = (
// //     dyn spi::ValidPinIdSck<pac::SPI0>,
// //     dyn spi::ValidPinIdTx<pac::SPI0>,
// //     dyn spi::ValidPinIdRx<pac::SPI0>,
// // );

// type Spi = spi::Spi<spi::Enabled, pac::SPI0, SpiPins, 8>;

// static mut SPI_INSTANCE: Option<Spi> = None;



#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let clocks = init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Gpio16 used istead of led pin as pico W + dev board is used as a target
    let mut led_pin = pins.gpio16.into_push_pull_output();

    /*****************************************************************
     * USB Configuration
     *****************************************************************/

    // Create the device-specific USB peripheral driver using rp-hal usb bus
    let usb_bus = UsbBusAllocator::new(usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet
        USB_BUS = Some(usb_bus);
    }

    // Grab a reference to the USB Bus allocator. We are promising to the
    // compiler not to take mutable access to this global variable whilst this
    // reference exists!
    let bus_ref = unsafe { USB_BUS.as_ref().unwrap() };

    // Set up the USB Communications Class Device driver
    let serial = SerialPort::new(bus_ref);
    unsafe {
        USB_SERIAL = Some(serial);
    }

    // Create a USB device with a fake VID and PID
    let usb_dev = UsbDeviceBuilder::new(bus_ref, UsbVidPid(0x16c0, 0x05E1))
        .manufacturer("panduza.io")
        .product("picoha-spi")
        .serial_number("XXXX")
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build();
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet
        USB_DEVICE = Some(usb_dev);
    }

    // Enable the USB interrupt
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::USBCTRL_IRQ);
    };

    // ------ No more USB code after this point in main! --------------------

    
    /*****************************************************************
     * SPI Configuration
     *****************************************************************/
    
    let spi_sclk: gpio::Pin<_, gpio::FunctionSpi, gpio::PullNone> = pins.gpio2.reconfigure();
    let spi_mosi: gpio::Pin<_, gpio::FunctionSpi, gpio::PullNone> = pins.gpio3.reconfigure();
    let spi_miso: gpio::Pin<_, gpio::FunctionSpi, gpio::PullUp> = pins.gpio4.reconfigure();
    let mut spi_cs = pins.gpio5.into_push_pull_output();

    // Create the device-specific SPI peripheral
    let spi = spi::Spi::<_, _, _, 8>::new(pac.SPI0, (spi_mosi, spi_miso, spi_sclk));

    // initialised SPI
    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        400.kHz(),
        embedded_hal::spi::MODE_0,
    );
    // unsafe {
    //     SPI_INSTANCE = Some(spi);
    // }

    spi_cs.set_high().unwrap(); // set inactif
    delay.delay_ms(200);

    loop {
        // Check for new data
        // if usb_dev.poll(&mut [&mut serial]) {
        //     let mut buf = [0u8; 64];
        //     match serial.read(&mut buf) {
        //         Err(_e) => {
        //             // Do nothing
        //         }
        //         Ok(0) => {
        //             // Do nothing
        //         }
        //         Ok(count) => {
        //             led_pin.toggle().unwrap(); // Toggle led each time data are received

        //             // send all received data on SPI at once
        //             {
        //                 spi_cs.set_low().unwrap();
        //                 let _transfer_success = spi.transfer(&mut buf[..count]);
        //                 spi_cs.set_high().unwrap();
        //                 let _ = serial.write(_transfer_success.unwrap());
        //             }

        //             // clean all to ensure no issue
        //             for n in 0..count {
        //                 buf[n] = 0u8;
        //             }

        //         }
        //     }
        // }
        led_pin.toggle().unwrap();
        delay.delay_ms(500);
    }
}

#[interrupt]
unsafe fn USBCTRL_IRQ() {
    use core::sync::atomic::{AtomicBool, Ordering};

    /// Note whether we've already printed the "hello" message.
    static SAID_HELLO: AtomicBool = AtomicBool::new(false);

    // Grab the global objects. This is OK as we only access them under interrupt.
    let usb_dev = USB_DEVICE.as_mut().unwrap();
    let serial = USB_SERIAL.as_mut().unwrap();
    // let spi = SPI_INSTANCE.as_mut().unwrap();

    // Say hello exactly once on start-up
    if !SAID_HELLO.load(Ordering::Relaxed) {
        SAID_HELLO.store(true, Ordering::Relaxed);
        let _ = serial.write(b"Hello, World!\r\n");
    }

    // Poll the USB driver with all of our supported USB Classes
    if usb_dev.poll(&mut [serial]) {
        let mut buf = [0u8; 64];
        match serial.read(&mut buf) {
            Err(_e) => {
                // Do nothing
            }
            Ok(0) => {
                // Do nothing
            }
            Ok(count) => {
                // Convert to upper case
                buf.iter_mut().take(count).for_each(|b| {
                    b.make_ascii_uppercase();
                });

                // Send back to the host
                let mut wr_ptr = &buf[..count];
                while !wr_ptr.is_empty() {
                    let _ = serial.write(wr_ptr).map(|len| {
                        wr_ptr = &wr_ptr[len..];
                    });
                }
            }
        }
    }
}
// End of file
