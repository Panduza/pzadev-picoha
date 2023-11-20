use hal::clocks::ClocksManager;
// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Alias for our HAL crate
use rp2040_hal as hal;

use embedded_hal::PwmPin;
use rp2040_hal::clocks::Clock;

use rp2040_hal::pwm::{CountFallingEdge, CountRisingEdge, InputHighRunning};

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use hal::pac;

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

const FSYS: u32 = 125_000_000u32;

pub struct Pwm {
    pub pwm_slices: hal::pwm::Slices,
    pub delay: cortex_m::delay::Delay,
    pub pins: hal::gpio::Pins,
    pub clocks: ClocksManager,
    pub uart0: hal::pac::UART0,
    pub resets: hal::pac::RESETS,
}

impl Pwm {
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
        let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

        // Init PWMs
        let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

        Self {
            delay: delay,
            pwm_slices: pwm_slices,
            pins: pins,
            clocks: clocks,
            uart0: pac.UART0,
            resets: pac.RESETS,
        }
    }

    pub fn set_freq(&mut self, fpwm: f32) -> (f32, f32, u16, u16, f32, u8, u8) {
        let period_wanted = (FSYS as f32) / fpwm;
        // let top = self.pwm_slices.pwm5.get_top();
        let precision = 0.1f32;
        let mut top = (period_wanted / 2.0).clamp(0.0, u16::MAX as f32);

        self.pwm_slices.pwm5.set_ph_correct();
        self.pwm_slices.pwm5.set_div_int(1);
        self.pwm_slices.pwm5.set_div_frac(0);

        let csr_ph_correct = 1u8;
        let div_int = 1u8;
        let div_frac = 0u8;

        let period = (top as f32 + 1.0)
            * (csr_ph_correct as f32 + 1.0)
            * (div_int as f32 + div_frac as f32 / 16.0);

        let (period_wanted, period, top, iterations, real_fpwm, div_frac, div_int) =
            binary_search(period_wanted, period, top as u16, precision);

        (
            period_wanted,
            period,
            top,
            iterations,
            real_fpwm,
            div_frac,
            div_int,
        )
    }
}

fn binary_search(
    period_wanted: f32,
    mut period: f32,
    mut top: u16,
    precision: f32,
) -> (f32, f32, u16, u16, f32, u8, u8) {
    let mut csr_ph_correct = 1u8;
    let mut div_int = 1u8;
    let mut div_frac = 0u8;

    // Iterative adjustment to achieve the desired frequency
    let mut iterations = 0;
    let max_iterations = 20; // Set an upper limit on iterations

    let mut lower_bound_div_frac = 0;
    let mut upper_bound_div_frac = u8::MAX;

    let mut lower_bound_div_int = 1;
    let mut upper_bound_div_int = u8::MAX;

    let fpwm_wanted = (FSYS as f32) / period_wanted;
    let mut real_fpwm = 0.0;

    // Iterative adjustment to achieve the desired frequency
    while iterations < max_iterations {
        period = (top as f32 + 1.0)
            * (csr_ph_correct as f32 + 1.0)
            * (div_int as f32 + (div_frac as f32 / 16.0));

        real_fpwm = (FSYS as f32) / period;
        // Check if the adjusted period is within the desired range
        if (real_fpwm == fpwm_wanted)
            || fpwm_wanted >= real_fpwm - precision && fpwm_wanted <= real_fpwm + precision
        {
            break;
        }
        // Binary search for a faster adjustment
        if period_wanted < period {
            upper_bound_div_frac = div_frac;
        } else {
            lower_bound_div_frac = div_frac;
        }

        div_frac = (lower_bound_div_frac + upper_bound_div_frac) / 2;

        // if period_wanted < period {
        //     upper_bound_div_int = div_int;
        // } else {
        //     lower_bound_div_int = div_int;
        // }

        // div_int = (lower_bound_div_int + upper_bound_div_int) / 2;

        iterations += 1;
    }
    return (
        period_wanted,
        period,
        top,
        iterations,
        real_fpwm,
        div_frac,
        div_int,
    );
}

// pub enum PwmSlice {
//     pwm0 = &mut self.pwm_slices.pwm0,
//     pwm1 = &mut self.pwm_slices.pwm1,
// }

// trait PwmConfig {
//     fn set_freq(pwm_channel: u8, frequency: f32);
// }

// trait SelectPwm {
//     fn pwm_number(&mut self, number: u8);
// }

// impl SelectPwm for Pwm {
//     fn pwm_number(&mut self, number: u8) {
//         match number {
//             0 => self.pwm_slices.pwm0,
//             1 => self.pwm_slices.pwm1,
//             2 => self.pwm_slices.pwm2,
//             3 => self.pwm_slices.pwm3,
//             4 => self.pwm_slices.pwm4,
//             5 => self.pwm_slices.pwm5,
//             6 => self.pwm_slices.pwm6,
//             7 => self.pwm_slices.pwm7,
//             _ => None {
//                 // Traitement par défaut ou erreur si nécessaire
//             },
//         }
//     }
// }

// End of file
