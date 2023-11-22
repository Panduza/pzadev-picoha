/*use crate::platform::{
    PlatformData,
    PlatformSleep,
};*/

use protocol::slip::{
    Decoder,
    Encoder,
    SlipError,
};


use protocol::ha;

use protocol::uart;
use protocol::common;

use const_random::const_random;

//use crate::platform::platform_impl;
use crate::platform::platform_impl::PlatformData;
use crate::platform::platform_impl::PlatformSleep;

///////////////////////////////////////

// Constants
#[cfg(debug_assertions)]
const VERSION: &str = env!("GIT_HASH");

#[cfg(not(debug_assertions))]
const VERSION: &str = env!("CARGO_PKG_VERSION");

const ID: [u8;8] = const_random!([u8;8]);

pub struct App
{
}

impl App
{
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn init<P: PlatformData>(&mut self, platf: &mut P)
    {
    }

    pub fn process_generic<P: PlatformData>(&mut self, platf: &mut P, frame: ha::MsgFrame) -> Result<ha::MsgFrame, ha::MsgError>
    {
        let req  = common::Request::consume_frame(frame)?;
        let resp = match req{
            common::Request::Ping    => common::Response::Good,
            common::Request::ItfType => common::Response::ItfTypeResp(ha::ItfType::Uart),
            common::Request::Version => common::Response::VersionResp(&VERSION),
            common::Request::IdGet   => common::Response::IdResp(&ID),
        };

        Ok(resp.to_frame())
    }

    pub fn process_uart<P: PlatformData>(&mut self, platf: &mut P, frame: ha::MsgFrame) -> Result<ha::MsgFrame, ha::MsgError>
    {
        let req  = uart::Request::consume_frame(frame)?;
        let resp = match req {
            uart::Request::DataTX(tx_data) => {
                uart::Response::Good
                //Err(x) => uart::Response::ErrGeneric("Error data TX"),      //TODO : modify error code
            }

            _ => {
                uart::Response::ErrGeneric("Error data TX")      //TODO : modify error code
            }
        };

        Ok(resp.to_frame())
    }

    pub fn process_frame<P: PlatformData>(&mut self, platf: &mut P, frame: ha::MsgFrame) -> Result<ha::MsgFrame, ha::MsgError>
    {
        match ha::CodeCategory::categorize(&frame.code) {
            ha::CodeCategory::ReqGeneric => self.process_generic(platf, frame),
            ha::CodeCategory::ReqUart    => self.process_uart   (platf, frame),
            _                            => Err(ha::MsgError::UnknownCode),
        }
    }
}
