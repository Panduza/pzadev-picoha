#![no_std]
#![no_main]

mod pwm_functions;
use pwm_functions::Pwm;

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
    let mut pwm = Pwm::init();

    let result = pwm.set_freq(1000.3);
    let period_wanted = result.0;
    let period = result.1;
    let top: u16 = result.2;
    let iteration: u16 = result.3;
    let real_fpwm = result.4;
    let div_frac = result.5;
    let div_int = result.6;

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

    writeln!(uart, "period_wanted : {period_wanted}\r").unwrap();
    writeln!(uart, "period : {period}\r").unwrap();
    writeln!(uart, "top : {top}\r").unwrap();
    writeln!(uart, "iteration : {iteration}\r").unwrap();
    writeln!(uart, "real_fpwm : {real_fpwm}\r").unwrap();
    writeln!(uart, "div_frac : {div_frac}\r").unwrap();
    writeln!(uart, "div_int : {div_int}\r").unwrap();

    uart.write_full_blocking(b"test end \r\n");

    let pwm5 = &mut pwm.pwm_slices.pwm5;

    let mut pwm1 = &mut pwm.pwm_slices.pwm1.into_mode::<InputHighRunning>();
    let _ = &mut pwm1.channel_b.input_from(pwm.pins.gpio19);

    pwm1.enable();

    pwm5.set_ph_correct();
    pwm5.set_top(top);
    pwm5.set_div_frac(div_frac);
    pwm5.set_div_int(div_int);
    pwm5.enable();

    let mut channel_a = &mut pwm5.channel_a;
    let mut channel_b = &mut pwm5.channel_b;
    channel_a.output_to(pwm.pins.gpio10);
    channel_b.output_to(pwm.pins.gpio11);

    channel_a.set_duty(top / 10);
    channel_b.set_duty(top / 10);

    loop {
        let counter = pwm1.get_counter();
        writeln!(uart, "counter : {counter}\r").unwrap();
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
