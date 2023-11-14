#![no_std]

pub mod slip;
pub mod ha;
pub mod uart;
pub mod common;

#[cfg(tests)]
mod tests {
    #[test]
    fn basic_test() {
        const MSG_DATA: [u8; 6] = [
            0x10, 0x03,
            0x00, 0x00,
            0xC6, 0x37,
        ];

        let msg: ha::MsgFrame = ha::MsgFrame::from_slice(&MSG_DATA).unwrap();

        println!("{:?}", msg);
        println!("{:?}", msg.crc());

        let req = uart::Request::consume_frame(msg);
        println!("{:?}", req);

        let resp = uart::Response::Baud(115200);
        let msg  = resp.to_frame();
        println!("{:?}", resp);
        println!("{:?}", msg );

        let resp = uart::Response::Good;
        let msg  = resp.to_frame();
        println!("{:?}", resp);
        println!("{:?}", msg );
    }
}