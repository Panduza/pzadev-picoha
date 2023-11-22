#![no_std]
#![no_main]

mod board;
mod platform;
mod app;

use board::Board;
use platform::platform_impl::Platform;
use app::App;

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

    let mut app = App::new();
    app.init(&mut platform);

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
                    let mut idx = 0;
                    while idx < count {
                        match decoder.feed(&buf[idx..(count)]) {
                            Err(e) => {
                                idx += e.pos;
                                decoder.reset();
                            }

                            Ok((nbytes,is_end)) => {
                                idx += nbytes;

                                if is_end {
                                    state = !state;

                                    fn _process_slice(app: &mut App, platform: &mut Platform, slice: &[u8]) -> Result<ha::MsgFrame, ha::MsgError> {
                                        let req_frame = ha::MsgFrame::from_slice(slice)?;
                                        app.process_frame(platform, req_frame)
                                    }

                                    // Get and process incoming slice
                                    let slice = decoder.slice();
                                    let response_frame = match _process_slice(&mut app, &mut platform, &slice) {
                                        Ok(frame) => frame,
                                        Err(exc)  => exc.to_frame(),
                                    };

                                    // Try encode response frame
                                    fn _build_response<const BUFLEN: usize>(ff: &ha::MsgFrame, encoder: &mut Encoder<BUFLEN>) -> Result<(), SlipError> {
                                        encoder.feed(ff.code.to_u16().to_be_bytes().as_slice())?;
                                        encoder.feed(ff.data.as_slice())?;
                                        encoder.feed(ff.crc().to_be_bytes().as_slice())?;
                                        encoder.finish()?;

                                        Ok(())
                                    }

                                    match _build_response(&response_frame, &mut encoder) {
                                        Ok(_) => {platform.usb.serial.write(encoder.slice()).ok();}
                                        Err(_) => {}
                                    }

                                    // Reset encoder and decoder
                                    decoder.reset();
                                    encoder.reset();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}