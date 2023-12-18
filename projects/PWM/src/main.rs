#![no_std]
#![no_main]

// mod app;
mod board;
mod platform;
mod platform_io;

use board::Board;
use platform::platform_impl::Platform;
use platform::test::{PwmInput_Duty, PwmInput_Freq, PwmOutput_A, PwmOutput_B};

use protocols::slip::{Decoder, Encoder, SlipError};

use protocols::ha;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Alias for our HAL crate
use rp2040_hal as hal;

use embedded_hal::PwmPin;
use rp2040_hal::clocks::Clock;

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
    let mut board = Board::init();

    let usb_bus = board.usb_bus;

    let mut platform = Platform::init(
        board.pins,
        board.pwm_slices,
        board.delay,
        board.timer,
        &usb_bus,
    )
    .unwrap();

    let mut encoder = Encoder::<64>::new();
    let mut decoder = Decoder::<64>::new();

    let mut state = true;

    // let mut pwm_output_6_a = PwmOutput_A::new(board.pwm_slices.pwm6, board.pins.gpio12);
    // let result = pwm_output_6_a.set_freq(1550.3);
    // pwm_output_6_a.set_duty(27.9);
    // let duty_6_a = pwm_output_6_a.get_duty();
    // pwm_output_6_a.enable();

    loop {
        if !platform.usb.dev.poll(&mut [&mut platform.usb.serial]) {
            continue;
        }

        let mut buf = [0u8; 64];

        match platform.usb.serial.read(&mut buf) {
            Err(_) => {}
            Ok(0) => {}

            Ok(count) => {
                // Send back to the host
                let mut wr_ptr = &buf[..count];
                while !wr_ptr.is_empty() {
                    match platform.usb.serial.write(wr_ptr) {
                        Ok(len) => wr_ptr = &wr_ptr[len..],
                        // On error, just drop unwritten data.
                        // One possible error is Err(WouldBlock), meaning the USB
                        // write buffer is full.
                        Err(_) => break,
                    };
                }
            }
        }
    }
}
// End of file
