use cortex_m;
use embedded_time::rate::*;

use rp_pico as bsp;

use bsp::{
    hal::{
        self,
        clocks::{init_clocks_and_plls, Clock},
        pac,
        sio::Sio,
        watchdog::Watchdog,
        gpio,
    },

    Pins,
};

use usb_device::class_prelude::UsbBusAllocator;


pub struct Board {
    pub delay: cortex_m::delay::Delay,
    pub pins: bsp::Pins,
    pub usb_bus: UsbBusAllocator<hal::usb::UsbBus>,
}

impl Board {
    pub fn init() -> Self {
        let mut pac = pac::Peripherals::take().unwrap();
        let core    = pac::CorePeripherals::take().unwrap();

        let mut watchdog = Watchdog::new(pac.WATCHDOG);
        let sio          = Sio::new(pac.SIO);

        let external_xtal_freq_hz = 12_000_000u32.Hz();
        let clocks = init_clocks_and_plls(
            external_xtal_freq_hz.integer(),
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
            .ok()
            .unwrap();

        // ---- USB init
        
        let usb = UsbBusAllocator::new(hal::usb::UsbBus::new(
            pac.USBCTRL_REGS,
            pac.USBCTRL_DPRAM,
            clocks.usb_clock,
            true,
            &mut pac.RESETS,
        ));


        // ---- Peripherals init

        let pins = bsp::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        let delay = cortex_m::delay::Delay::new(
            core.SYST,
            clocks.system_clock.freq().integer()
        );

        Self {
            usb_bus: usb,
            delay:   delay,
            pins:    pins,
        }
    }
}
