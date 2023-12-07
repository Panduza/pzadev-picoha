// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Alias for our HAL crate
use rp2040_hal as hal;

use rp2040_hal::clocks::Clock;

use rp2040_hal::Timer;

use usb_device::class_prelude::UsbBusAllocator;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use hal::pac;

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

pub struct Board {
    pub pwm_slices: hal::pwm::Slices,
    pub pins: hal::gpio::Pins,

    pub timer: hal::Timer,
    pub delay: cortex_m::delay::Delay,
    pub usb_bus: UsbBusAllocator<hal::usb::UsbBus>,
}

impl Board {
    pub fn init() -> Self {
        // Grab our singleton objects
        let mut pac = pac::Peripherals::take().unwrap();

        let core = pac::CorePeripherals::take().unwrap();

        // Set up the watchdog driver - needed by the clock setup code
        let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

        // The single-cycle I/O block controls our GPIO pins
        let sio = hal::Sio::new(pac.SIO);

        // Configure the clocks
        //
        // The default is to generate a 125 MHz system clock
        let clocks = hal::clocks::init_clocks_and_plls(
            XTAL_FREQ_HZ,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        // Set the pins up according to their function on this particular board
        let pins = hal::gpio::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        // Init Timer
        let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

        let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

        // ---- USB init

        let usb = UsbBusAllocator::new(hal::usb::UsbBus::new(
            pac.USBCTRL_REGS,
            pac.USBCTRL_DPRAM,
            clocks.usb_clock,
            true,
            &mut pac.RESETS,
        ));

        // Init PWMs
        let pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

        Self {
            pwm_slices: pwm_slices,
            pins: pins,

            timer: timer,
            usb_bus: usb,
            delay: delay,
        }
    }
}

// End of file
