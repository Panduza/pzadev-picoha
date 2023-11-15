#![no_std]

pub mod common;
pub mod ha;
pub mod slip;

#[cfg(tests)]
mod tests {
    #[test]
    fn basic_test() {
        const MSG_DATA: [u8; 6] = [0x01, 0x00, 0x02, 0x02, 0xB4, 0x54];

        let msg: ha::MsgFrame = ha::MsgFrame::from_slice(&MSG_DATA).unwrap();

        println!("{:?}", msg);
        println!("{:?}", msg.crc());

        let req = gpio::Request::consume_frame(msg);
        println!("{:?}", req);

        let resp = gpio::Response::GpioValue(3, GpioValue::Low);
        let msg = resp.to_frame();
        println!("{:?}", resp);
        println!("{:?}", msg);

        let resp = gpio::Response::Good;
        let msg = resp.to_frame();
        println!("{:?}", resp);
        println!("{:?}", msg);
    }
}
