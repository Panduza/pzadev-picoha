use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::digital::v2::PinState;
use embedded_time::rate::*;

use hal::gpio::bank0::{
    Gpio0, Gpio1, Gpio10, Gpio11, Gpio12, Gpio13, Gpio14, Gpio15, Gpio16, Gpio17, Gpio18, Gpio19,
    Gpio2, Gpio20, Gpio3, Gpio4, Gpio5, Gpio6, Gpio7, Gpio8, Gpio9,
};
use hal::gpio::PinId;
use hal::gpio::{FunctionNull, PullDown};

use embedded_hal::PwmPin;

use hal::pwm::FreeRunning;
use hal::pwm::{Pwm0, Pwm1, Pwm2, Pwm3, Pwm4, Pwm5, Pwm6, Pwm7};
use usb_device::class_prelude::UsbBusAllocator;
use usb_device::prelude::{UsbDevice, UsbDeviceBuilder, UsbVidPid};

use usbd_serial::SerialPort;
use usbd_serial::USB_CLASS_CDC;

use rp2040_hal as hal;

use crate::platform::usb_config;

use crate::platform_io::pwm_ctrl::{GpioIndex, PwmCtrlError, PwmCtrlOutputA, PwmSliceIndex, Value};

//////////////////////////////////////////////////////////////////////////

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

pub struct PlatformPinsArray {
    pin0: hal::gpio::Pin<Gpio0, FunctionNull, PullDown>,
    pin1: hal::gpio::Pin<Gpio1, FunctionNull, PullDown>,
    pin2: hal::gpio::Pin<Gpio2, FunctionNull, PullDown>,
    pin3: hal::gpio::Pin<Gpio3, FunctionNull, PullDown>,
    pin4: hal::gpio::Pin<Gpio4, FunctionNull, PullDown>,
    pin5: hal::gpio::Pin<Gpio5, FunctionNull, PullDown>,
    pin6: hal::gpio::Pin<Gpio6, FunctionNull, PullDown>,
    pin7: hal::gpio::Pin<Gpio7, FunctionNull, PullDown>,
    pin8: hal::gpio::Pin<Gpio8, FunctionNull, PullDown>,
    pin9: hal::gpio::Pin<Gpio9, FunctionNull, PullDown>,
    pin10: hal::gpio::Pin<Gpio10, FunctionNull, PullDown>,
    pin11: hal::gpio::Pin<Gpio11, FunctionNull, PullDown>,
    pin12: hal::gpio::Pin<Gpio12, FunctionNull, PullDown>,
    pin13: hal::gpio::Pin<Gpio13, FunctionNull, PullDown>,
    pin14: hal::gpio::Pin<Gpio14, FunctionNull, PullDown>,
    pin15: hal::gpio::Pin<Gpio15, FunctionNull, PullDown>,
    pin16: hal::gpio::Pin<Gpio16, FunctionNull, PullDown>,
    pin17: hal::gpio::Pin<Gpio17, FunctionNull, PullDown>,
    pin18: hal::gpio::Pin<Gpio18, FunctionNull, PullDown>,
    pin19: hal::gpio::Pin<Gpio19, FunctionNull, PullDown>,
    pin20: hal::gpio::Pin<Gpio20, FunctionNull, PullDown>,
}

impl PlatformPinsArray {
    fn new(pins: hal::gpio::Pins) -> Self {
        Self {
            pin0: pins.gpio0,
            pin1: pins.gpio1,
            pin2: pins.gpio2,
            pin3: pins.gpio3,
            pin4: pins.gpio4,
            pin5: pins.gpio5,
            pin6: pins.gpio6,
            pin7: pins.gpio7,
            pin8: pins.gpio8,
            pin9: pins.gpio9,
            pin10: pins.gpio10,
            pin11: pins.gpio11,
            pin12: pins.gpio12,
            pin13: pins.gpio13,
            pin14: pins.gpio14,
            pin15: pins.gpio15,
            pin16: pins.gpio16,
            pin17: pins.gpio17,
            pin18: pins.gpio18,
            pin19: pins.gpio19,
            pin20: pins.gpio20,
        }
    }

    fn borrow(&self, idx: GpioIndex) -> Option<&hal::gpio::Pin<Gpio12, FunctionNull, PullDown>> {
        match idx {
            // 0 => Some(&self.pin0),
            // 1 => Some(&self.pin1),
            // 2 => Some(&self.pin2),
            // 3 => Some(&self.pin3),
            // 4 => Some(&self.pin4),
            // 5 => Some(&self.pin5),
            // 6 => Some(&self.pin6),
            // 7 => Some(&self.pin7),
            // 8 => Some(&self.pin8),
            // 9 => Some(&self.pin9),
            // 10 => Some(&self.pin10),
            // 11 => Some(&self.pin11),
            12 => Some(&self.pin12),
            // 13 => Some(&self.pin13),
            // 14 => Some(&self.pin14),
            // 15 => Some(&self.pin15),
            // 16 => Some(&self.pin16),
            // 17 => Some(&self.pin17),
            // 18 => Some(&self.pin18),
            // 19 => Some(&self.pin19),
            // 20 => Some(&self.pin20),
            _ => None,
        }
    }

    fn borrow_mutable(
        &mut self,
        idx: GpioIndex,
    ) -> Option<&mut hal::gpio::Pin<Gpio12, FunctionNull, PullDown>> {
        match idx {
            // 0 => Some(&mut self.pin0),
            // 1 => Some(&mut self.pin1),
            // 2 => Some(&mut self.pin2),
            // 3 => Some(&mut self.pin3),
            // 4 => Some(&mut self.pin4),
            // 5 => Some(&mut self.pin5),
            // 6 => Some(&mut self.pin6),
            // 7 => Some(&mut self.pin7),
            // 8 => Some(&mut self.pin8),
            // 9 => Some(&mut self.pin9),
            // 10 => Some(&mut self.pin10),
            // 11 => Some(&mut self.pin11),
            12 => Some(&mut self.pin12),
            // 13 => Some(&mut self.pin13),
            // 14 => Some(&mut self.pin14),
            // 15 => Some(&mut self.pin15),
            // 16 => Some(&mut self.pin16),
            // 17 => Some(&mut self.pin17),
            // 18 => Some(&mut self.pin18),
            // 19 => Some(&mut self.pin19),
            // 20 => Some(&mut self.pin20),
            _ => None,
        }
    }
}

pub struct PlatformPwmArray {
    pwm0: hal::pwm::Slice<Pwm0, FreeRunning>,
    pwm1: hal::pwm::Slice<Pwm1, FreeRunning>,
    pwm2: hal::pwm::Slice<Pwm2, FreeRunning>,
    pwm3: hal::pwm::Slice<Pwm3, FreeRunning>,
    pwm4: hal::pwm::Slice<Pwm4, FreeRunning>,
    pwm5: hal::pwm::Slice<Pwm5, FreeRunning>,
    pwm6: hal::pwm::Slice<Pwm6, FreeRunning>,
    pwm7: hal::pwm::Slice<Pwm7, FreeRunning>,
}

impl PlatformPwmArray {
    fn new(slices: hal::pwm::Slices) -> Self {
        Self {
            pwm0: slices.pwm0,
            pwm1: slices.pwm1,
            pwm2: slices.pwm2,
            pwm3: slices.pwm3,
            pwm4: slices.pwm4,
            pwm5: slices.pwm5,
            pwm6: slices.pwm6,
            pwm7: slices.pwm7,
        }
    }

    fn borrow(&self, idx: PwmSliceIndex) -> Option<&hal::pwm::Slice<Pwm6, FreeRunning>> {
        match idx {
            // 0 => Some(&self.pwm0),
            // 1 => Some(&self.pwm1),
            // 2 => Some(&self.pwm2),
            // 3 => Some(&self.pwm3),
            // 4 => Some(&self.pwm4),
            // 5 => Some(&self.pwm5),
            6 => Some(&self.pwm6),
            // 7 => Some(&self.pwm7),
            _ => None,
        }
    }

    fn borrow_mutable(
        &mut self,
        idx: PwmSliceIndex,
    ) -> Option<&hal::pwm::Slice<Pwm6, FreeRunning>> {
        match idx {
            // 0 => Some(&mut self.pwm0),
            // 1 => Some(&mut self.pwm1),
            // 2 => Some(&mut self.pwm2),
            // 3 => Some(&mut self.pwm3),
            // 4 => Some(&mut self.pwm4),
            // 5 => Some(&mut self.pwm5),
            6 => Some(&mut self.pwm6),
            // 7 => Some(&mut self.pwm7),
            _ => None,
        }
    }
}

// pub struct PlatformPins {
//     pins: PlatformPinsArray,
// }

// impl PlatformPins {
//     pub fn new(pins: hal::gpio::Pins) -> Self {
//         Self {
//             pins: PlatformPinsArray::new(pins),
//         }
//     }
// }

// pub struct PlatformPwm {
//     pwm: PlatformPwmArray,
// }

// impl PlatformPwm {
//     pub fn new(slices: hal::pwm::Slices) -> Self {
//         Self {
//             pwm: PlatformPwmArray::new(slices),
//         }
//     }
// }

pub struct PlatformPwmPins {
    pins: PlatformPinsArray,
    pwm: PlatformPwmArray,
}

impl PlatformPwmPins {
    pub fn new(slices: hal::pwm::Slices, pins: hal::gpio::Pins) -> Self {
        Self {
            pwm: PlatformPwmArray::new(slices),
            pins: PlatformPinsArray::new(pins),
        }
    }
}

// impl PwmCtrlOutputA for PlatformPwmPins {
//     // TODO : test if it works
//     fn enable(&mut self, pwm: PwmSliceIndex, gpio: GpioIndex) -> Result<(), PwmCtrlError> {
//         let Some(mut slice) = self.pwm.borrow_mutable(6);
//         let Some(pin) = self.pins.borrow_mutable(12);

//         slice.channel_a.output_to(pin);
//         slice.channel_a.set_duty(3000);
//         slice.enable();

//         Ok(())
//     }

//     fn disable(&mut self, pwm: PwmSliceIndex, gpio: GpioIndex) -> Result<(), PwmCtrlError> {
//         let Some(mut slice) = self.pwm.borrow_mutable(6);
//         let Some(pin) = self.pins.borrow_mutable(12);

//         slice.disable();

//         Ok(())
//     }

//     fn duty_get(&mut self, pwm: PwmSliceIndex, gpio: GpioIndex) -> Result<Value, PwmCtrlError> {
//         Ok(());
//     }
//     fn duty_set(
//         &mut self,
//         pwm: PwmSliceIndex,
//         gpio: GpioIndex,
//         duty: Value,
//     ) -> Result<(), PwmCtrlError> {
//         Ok(())
//     }

//     fn freq_set(
//         &mut self,
//         pwm: PwmSliceIndex,
//         gpio: GpioIndex,
//         freq: Value,
//     ) -> Result<(), PwmCtrlError> {
//         Ok(())
//     }
// }

pub struct MyPlatformSleep {
    delay: cortex_m::delay::Delay,
}

impl PlatformSleep for MyPlatformSleep {
    fn sleep_ms(&mut self, delay_ms: u32) {
        self.delay.delay_ms(delay_ms);
    }
}

pub struct Timer {
    timer: hal::Timer,
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
    pub dev: UsbDevice<'a, hal::usb::UsbBus>,
    pub serial: SerialPort<'a, hal::usb::UsbBus>,
}

impl<'a> PlatformUsb<'a> {
    pub fn new(bus: &'a UsbBusAllocator<hal::usb::UsbBus>) -> Self {
        let serial = SerialPort::new(bus);

        let dev = UsbDeviceBuilder::new(
            bus,
            UsbVidPid(usb_config::USB_MANUFACTURER_ID, usb_config::USB_PRODUCT_ID),
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
    pub pins: PlatformPinsArray,
    pub pwm: PlatformPwmArray,
    pub usb: PlatformUsb<'a>,
    pub timer: hal::Timer,
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
        pins: hal::gpio::Pins,
        pwm_slices: hal::pwm::Slices,
        delay: cortex_m::delay::Delay,
        timer: hal::Timer,
        usb_bus: &'a UsbBusAllocator<hal::usb::UsbBus>,
    ) -> Result<Self, PlatformError> {
        let usb = PlatformUsb::new(usb_bus);

        let pwm_pins = PlatformPwmPins::new(pwm_slices, pins);
        let pwm = pwm_pins.pwm;
        let gpio = pwm_pins.pins;

        Ok(Self {
            sleep: MyPlatformSleep { delay: delay },
            pins: gpio,
            pwm: pwm,
            timer: timer,
            usb: usb,
        })
    }
}
