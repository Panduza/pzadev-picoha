use hal::clocks::ClocksManager;
// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Alias for our HAL crate
use rp2040_hal as hal;

use rp2040_hal::clocks::Clock;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use hal::pac;

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

pub struct Board {
    pub pwm_slices: hal::pwm::Slices,
    pub delay: cortex_m::delay::Delay,
    pub pins: hal::gpio::Pins,
    pub clocks: ClocksManager,
    pub uart0: hal::pac::UART0,
    pub resets: hal::pac::RESETS,
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

        // The delay object lets us wait for specified amounts of time (in
        // milliseconds)
        let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

        // Init PWMs
        let pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

        Self {
            delay: delay,
            pwm_slices: pwm_slices,
            pins: pins,
            clocks: clocks,
            uart0: pac.UART0,
            resets: pac.RESETS,
        }
    }
}

// End of file
