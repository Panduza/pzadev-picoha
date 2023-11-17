use embedded_time::rate::*;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::PinState;


use usb_device::class_prelude::UsbBusAllocator;
use usb_device::prelude::{
    UsbDevice,
    UsbDeviceBuilder,
    UsbVidPid,
};

use usbd_serial::SerialPort;
use usbd_serial::USB_CLASS_CDC;


use panic_probe as _;

use rp_pico as bsp;

use bsp::hal::{
    self,
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
    gpio::{
        self,
        DynPin,
    },
};
use bsp::hal::gpio::{DYN_PULL_DOWN_INPUT, DYN_PULL_UP_INPUT, DYN_FLOATING_INPUT, DYN_READABLE_OUTPUT, DynPinMode};

use crate::platform::usb_config;

use crate::board::Board;

//////////////////////////////////////////////////////////////////////////

//pub struct MyPlatformLed {
//    pin: gpio::Pin<gpio::bank0::Gpio25, gpio::PushPullOutput>
//}
//
//impl PlatformLed for MyPlatformLed {
//    fn led_on(&mut self) {
//        self.pin.set_high().unwrap();
//    }
//
//    fn led_off(&mut self) {
//        self.pin.set_low().unwrap();
//    }
//}

#[derive(Debug)]
pub enum PlatformError {
    InitError,
}

pub trait PlatformSleep {
    fn sleep_ms(&mut self, delay_ms: u32);
}

pub trait PlatformData {
    //fn get_led  (&mut self) -> &mut dyn PlatformLed;
    fn get_sleep(&mut self) -> &mut dyn PlatformSleep;
    //fn get_pins (&mut self) -> &mut dyn GpioCtrl;
}

pub struct MyPlatformSleep {
    delay: cortex_m::delay::Delay,
}

impl PlatformSleep for MyPlatformSleep {
    fn sleep_ms(&mut self, delay_ms: u32) {
        self.delay.delay_ms(delay_ms);
    }
}

//////////////////////////////////////////////////////////////////////////


pub struct PlatformUsbConfig<'a> {
    manufacturer_id: u16,
    product_id: u16,
    manufacturer_name: &'a str,
    product_name: &'a str,
    serial_number: &'a str,
}

pub struct PlatformUsb<'a> {
    pub dev:    UsbDevice<'a, hal::usb::UsbBus>,
    pub serial: SerialPort<'a, hal::usb::UsbBus>,
}

impl<'a> PlatformUsb<'a> {
    pub fn new(
        bus: &'a UsbBusAllocator<hal::usb::UsbBus>,
    ) -> Self {

        let serial = SerialPort::new(bus);

        let dev = UsbDeviceBuilder::new(
            bus,
            UsbVidPid(usb_config::USB_MANUFACTURER_ID, usb_config::USB_PRODUCT_ID)
        )
            .manufacturer(usb_config::USB_MANUFACTURER_NAME)
            .product(usb_config::USB_PRODUCT_NAME)
            .serial_number(usb_config::USB_SERIAL_NUMBER)
            .device_class(USB_CLASS_CDC)
            .build();
        

        Self {
            dev: dev,
            serial: serial,
        }
    }
}

//////////////////////////////////////////////////////////////////////////

pub struct Platform<'a> {
    pub sleep: MyPlatformSleep,
    //pub pins: PlatformPins,

    pub usb: PlatformUsb<'a>,
}

impl<'a> PlatformData for Platform<'a> {
    //fn get_led(&mut self) -> &mut dyn PlatformLed {
    //    &mut self.led
    //}

    fn get_sleep(&mut self) -> &mut dyn PlatformSleep {
        &mut self.sleep
    }

    //fn get_pins(&mut self) -> &mut dyn GpioCtrl {
        //&mut self.pins
    //}
}

impl<'a> Platform<'a> {
    pub fn init(
        //pins: bsp::Pins,
        delay: cortex_m::delay::Delay,
        usb_bus: &'a UsbBusAllocator<hal::usb::UsbBus>,
    ) -> Result<Self, PlatformError> {

        let usb = PlatformUsb::new(
            usb_bus,
        );
        
        //let pins = PlatformPins::new(pins);

        Ok(Self {
            sleep: MyPlatformSleep { delay: delay },
            //pins: pins,
            usb: usb,
        })
    }
}
