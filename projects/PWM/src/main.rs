#![no_std]
#![no_main]

mod board;
mod platform;

use board::Board;
use platform::{PwmInput, PwmOutput_A, PwmOutput_B};

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Alias for our HAL crate
use rp2040_hal as hal;

// Some traits we need
use core::fmt::Write;
use fugit::RateExtU32;

use embedded_hal::PwmPin;
use rp2040_hal::clocks::Clock;

use rp2040_hal::pwm::{CountFallingEdge, CountRisingEdge, DynChannelId::B, InputHighRunning};

// UART related types
use hal::uart::{DataBits, StopBits, UartConfig};

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
/// Note: This boot block is not necessary when using a rp-hal based BSP
/// as the BSPs already perform this step.
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

/// Entry point to our bare-metal application.
///
/// The `#[rp2040_hal::entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables and the spinlock are initialised.
///
/// The function configures the RP2040 peripherals, then fades the LED in an
/// infinite loop.
#[rp2040_hal::entry]

fn main() -> ! {
    // Init Pwm
    let mut pwm = Board::init();

    // Configure UART
    let uart_pins = (
        // UART TX (characters sent from RP2040) on pin 1 (GPIO0)
        pwm.pins.gpio0.into_function(),
        // UART RX (characters received by RP2040) on pin 2 (GPIO1)
        pwm.pins.gpio1.into_function(),
    );
    let mut uart = hal::uart::UartPeripheral::new(pwm.uart0, uart_pins, &mut pwm.resets)
        .enable(
            UartConfig::new(9600.Hz(), DataBits::Eight, None, StopBits::One),
            pwm.clocks.peripheral_clock.freq(),
        )
        .unwrap();

    ///////////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////////

    // For now PWM OUTPUT can only generate a frequency above 3.7 Hz

    let mut pwm_output_5_a = PwmOutput_A::new(pwm.pwm_slices.pwm5, pwm.pins.gpio10);
    let result = pwm_output_5_a.set_freq(5.6);
    pwm_output_5_a.set_duty(15.7);
    let duty_5_a = pwm_output_5_a.get_duty();
    pwm_output_5_a.enable();

    let mut pwm_output_6_a = PwmOutput_A::new(pwm.pwm_slices.pwm6, pwm.pins.gpio12);
    let result = pwm_output_6_a.set_freq(7.7);
    pwm_output_6_a.set_duty(35.8);
    let duty_6_a = pwm_output_6_a.get_duty();
    pwm_output_6_a.enable();

    // writeln!(uart, "duty_5_a : {duty_5_a}\r").unwrap();
    // writeln!(uart, "duty_6_a : {duty_6_a}\r").unwrap();

    // let mut pwm_output_1_b = PwmOutput_B::new(pwm.pwm_slices.pwm1, pwm.pins.gpio19);
    // let result = pwm_output_1_b.set_freq(1543.5);
    // pwm_output_1_b.set_duty(20.0);
    // pwm_output_1_b.enable();

    let period_wanted = result.0;
    let period = result.1;
    let top: u16 = result.2;
    let iteration: u16 = result.3;
    let real_fpwm = result.4;
    let div_frac = result.5;
    let div_int = result.6;

    writeln!(uart, "period_wanted : {period_wanted}\r").unwrap();
    writeln!(uart, "period : {period}\r").unwrap();
    writeln!(uart, "top : {top}\r").unwrap();
    // writeln!(uart, "iteration : {iteration}\r").unwrap();
    writeln!(uart, "real_fpwm : {real_fpwm}\r").unwrap();
    writeln!(uart, "div_frac : {div_frac}\r").unwrap();
    writeln!(uart, "div_int : {div_int}\r").unwrap();

    let mut pwm_input_1_b = PwmInput::new(
        pwm.pwm_slices.pwm1.into_mode::<InputHighRunning>(),
        pwm.pins.gpio19,
    );

    // let mut delay = pwm.delay.delay_ms(20);
    let mut timer = pwm.timer;

    let counter = pwm_input_1_b.measure_freq(&mut timer).0;
    let freq = pwm_input_1_b.measure_freq(&mut timer).1;

    writeln!(uart, "counter : {counter}\r").unwrap();
    writeln!(uart, "freq : {freq}\r").unwrap();

    // let system_frequency = pwm.clocks.system_clock.freq();
    // writeln!(uart, "system_frequency : {system_frequency}\r").unwrap();

    loop {}
}

// End of file
