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

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

/// Entry point to our bare-metal application.
///
/// The `#[rp2040_hal::entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables and the spinlock are initialised.
///
/// The function configures the RP2040 peripherals, then fades the LED in an
/// infinite loop.
#[rp2040_hal::entry]
fn main() -> ! {
    let mut pwm = Pwm::init();

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

    uart.write_full_blocking(b"sssssssssssssssss \r\n");

    let pwm5 = &mut pwm.pwm_slices.pwm5;
    pwm5.set_ph_correct();
    pwm5.enable();

    let mut pwm1 = pwm.pwm_slices.pwm1;
    pwm1.set_ph_correct();
    pwm1.enable();

    let mut pwm1 = pwm1.into_mode::<CountRisingEdge>();

    // Use B channel (which inputs from GPIO 25)
    let mut channel_b = &mut pwm5.channel_b;
    let channel_pin_b = channel_b.output_to(pwm.pins.gpio11);

    // Use A channel (which outputs to GPIO 24)
    let channel_a = &mut pwm5.channel_a;
    let channel_pin_a = channel_a.output_to(pwm.pins.gpio10);

    let mut channel_b_1 = &mut pwm1.channel_b.input_from(pwm.pins.gpio19);

    // Access the SYSINFO peripheral and read the CHIP_ID register
    // let sysinfo = &pac.SYSINFO;
    // let chip_id = sysinfo.chip_id.read().bits();

    // writeln!(uart, "chip id: Ox{chip_id:08X}\r").unwrap();

    // test calcul freq pwm

    // Infinite loop, fading LED up and down
    loop {
        let duty = channel_a.get_max_duty() / 10;
        channel_a.set_duty(duty);

        channel_b.set_duty(duty);

        let counter = pwm1.get_counter();
        pwm1.set_counter(channel_a.get_max_duty() / 2);
        let top = pwm1.get_top();
        let duty_cycle = (counter as f32 / top as f32) * 100.0;

        // let system_frequency = clocks.system_clock.freq();
        // writeln!(uart, "system_frequency : {system_frequency}\r").unwrap();

        // writeln!(uart, "top : {top}\r").unwrap();
        writeln!(uart, "counter : {counter}\r").unwrap();
        writeln!(uart, "duty cycle : {duty_cycle}\r").unwrap();
        // delay.delay_ms(500);
    }
}

// End of file
