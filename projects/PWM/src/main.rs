#![no_std]
#![no_main]

mod board;
mod platform;

use board::Board;
use hal::pwm::{AnySlice, FreeRunning, A};
use platform::{Pwm_Channel_A, Pwm_Channel_B};

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

use rp2040_hal::pwm::{CountFallingEdge, CountRisingEdge, InputHighRunning};

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

    let mut pwm_5_a = Pwm_Channel_A::new(pwm.pwm_slices.pwm5, pwm.pins.gpio10);
    let result = pwm_5_a.set_freq(12000.9);
    pwm_5_a.set_duty(2000);
    pwm_5_a.enable();

    // let mut pwm_1_b = Pwm_Channel_B::new(pwm.pwm_slices.pwm1, pwm.pins.gpio19);
    // let result = pwm_1_b.set_freq(22000.9);
    // pwm_1_b.set_duty(2000);
    // pwm_1_b.enable();

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
    writeln!(uart, "iteration : {iteration}\r").unwrap();
    writeln!(uart, "real_fpwm : {real_fpwm}\r").unwrap();
    writeln!(uart, "div_frac : {div_frac}\r").unwrap();
    writeln!(uart, "div_int : {div_int}\r").unwrap();

    // let system_frequency = pwm.clocks.system_clock.freq();
    // writeln!(uart, "system_frequency : {system_frequency}\r").unwrap();

    // uart.write_full_blocking(b"test end \r\n");

    // let pwm2 = &mut pwm.pwm_slices.pwm2;

    // let pwm1 = &mut pwm.pwm_slices.pwm1.into_mode::<CountRisingEdge>();
    // let _ = &mut pwm1.channel_b.input_from(pwm.pins.gpio19);

    // pwm1.set_ph_correct();
    // pwm1.enable();

    // pwm2.clr_ph_correct();
    // pwm2.enable();

    // let start_time = pwm2.get_counter();
    // let mut last_rising_edge = pwm1.get_counter();

    loop {
        // Wait until a rising edge occurs
        // while pwm1.get_counter() == last_rising_edge {}

        // // Capture the current rising edge time
        // let current_rising_edge = pwm1.get_counter();

        // // Calculate time period between consecutive rising edges
        // let time_period = current_rising_edge - last_rising_edge;

        // // Calculate frequency
        // let frequency = 125000000.0 / (time_period as f32);

        // let counter = pwm2.get_counter();

        // // let mut freq = 125000000.0 / (counter) as f32 * 2.0;
        // // writeln!(uart, "counter : {counter}\r").unwrap();
        // writeln!(uart, "time_period : {time_period}\r").unwrap();
        // writeln!(uart, "frequency : {frequency}\r").unwrap();
    }

    // let pwm5 = &mut pwm.pwm_slices.pwm5;
    // pwm5.set_ph_correct();
    // pwm5.enable();

    // let mut pwm1 = pwm.pwm_slices.pwm1;
    // pwm1.set_ph_correct();
    // //pwm1.set_top(30000);
    // pwm1.enable();

    // let mut pwm1 = pwm1.into_mode::<CountRisingEdge>();

    // // Use B channel (which inputs from GPIO 25)
    // let mut channel_b = &mut pwm5.channel_b;
    // let channel_pin_b = channel_b.output_to(pwm.pins.gpio11);

    // // Use A channel (which outputs to GPIO 24)
    // let channel_a = &mut pwm5.channel_a;
    // let channel_pin_a = channel_a.output_to(pwm.pins.gpio10);

    // let mut channel_b_1 = &mut pwm1.channel_b.input_from(pwm.pins.gpio19);

    // // test calcul freq pwm

    // // Infinite loop, fading LED up and down
    // loop {
    //     let duty = channel_a.get_max_duty() / 10;
    //     channel_a.set_duty(duty);

    //     channel_b.set_duty(duty);

    //     let counter = pwm1.get_counter();
    //     // pwm1.set_counter(30000);
    //     let top = pwm1.get_top();

    //     let duty_cycle = (counter as f32 / top as f32) * 100.0;

    //     // let system_frequency = clocks.system_clock.freq();
    //     // writeln!(uart, "system_frequency : {system_frequency}\r").unwrap();

    //     // writeln!(uart, "top : {top}\r").unwrap();
    //     // writeln!(uart, "counter : {counter}\r").unwrap();
    //     // writeln!(uart, "duty cycle : {duty_cycle}\r").unwrap();
    //     // delay.delay_ms(500);
    // }
}

// End of file
