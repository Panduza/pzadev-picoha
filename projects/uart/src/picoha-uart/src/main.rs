#![no_std]
#![no_main]

mod board;
mod platform;

use board::Board;
use platform::platform_impl::Platform;

use rp_pico as bsp;
use bsp::entry;

use protocol::slip::{
    Encoder,
    Decoder,
    SlipError,
};

use protocol::ha;

#[entry]
fn main() -> ! {
    let board        = Board::init();

    let usb_bus      = board.usb_bus;
    let mut platform = Platform::init(
        //board.pins,
        board.delay,
        &usb_bus
    ).unwrap();

    //platform.pins.init().unwrap();

    //let mut app = App::new();
    //app.init(&mut platform);

    // Encoder/Decoder for commands
    let mut encoder = Encoder::<64>::new();
    let mut decoder = Decoder::<64>::new();

    let mut state   = true;

    loop {
        if platform.usb.dev.poll(&mut [&mut platform.usb.serial]) {
            let mut buf = [0u8; 64];


            match platform.usb.serial.read(&mut buf) {
                Err(_) => {},
                Ok(0)  => {},

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
}