use protocols::slip::{Decoder, Encoder, SlipError};

use protocols::ha;

use protocols::common;
use protocols::pwm;

use crate::platform_io::PlatformData;

use const_random::const_random;

///

///////////////////////////////////////

// Constants
#[cfg(debug_assertions)]
const VERSION: &str = env!("GIT_HASH");

#[cfg(not(debug_assertions))]
const VERSION: &str = env!("CARGO_PKG_VERSION");

const ID: [u8; 8] = const_random!([u8; 8]);

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub fn init<P: PlatformData>(&mut self, platf: &mut P) {}

    pub fn process_generic<P: PlatformData>(
        &mut self,
        platf: &mut P,
        frame: ha::MsgFrame,
    ) -> Result<ha::MsgFrame, ha::MsgError> {
        let req = common::Request::consume_frame(frame)?;
        let resp = match req {
            common::Request::Ping => common::Response::Good,
            common::Request::ItfType => common::Response::ItfTypeResp(ha::ItfType::Pwm),
            common::Request::Version => common::Response::VersionResp(&VERSION),
            common::Request::IdGet => common::Response::IdResp(&ID),
        };

        Ok(resp.to_frame())
    }

    pub fn process_pwm<P: PlatformData>(
        &mut self,
        platf: &mut P,
        frame: ha::MsgFrame,
    ) -> Result<ha::MsgFrame, ha::MsgError> {
        let req = pwm::Request::consume_frame(frame)?;
        let resp = match req {
            pwm::Request::SetDutyCycle(idx, dir) => {
                match platf
                    .get_pins()
                    .dir_set(idx as PinIndex, dir.into_pin_dir())
                {
                    Ok(_) => pwm::Response::Good,
                    Err(x) => pwm::Response::ErrGeneric("Cannot set into desired duty cyle"),
                }
            }

            pwm::Request::GetDutycycle(idx) => match platf.get_pins().dir_get(idx as PinIndex) {
                Ok(x) => pwm::Response::GpioDir(idx, x.into_gpio_dir()),
                Err(x) => pwm::Response::ErrInvalidArgs,
            },

            pwm::Request::SetFrequency(idx, value) => {
                match platf
                    .get_pins()
                    .pin_write(idx as PinIndex, value.into_pin_value())
                {
                    Ok(_) => pwm::Response::Good,
                    Err(_) => pwm::Response::ErrGeneric("Cannot set into desired frequency"),
                }
            }

            pwm::Request::GetFrequency(idx) => match platf.get_pins().pin_read(idx as PinIndex) {
                Ok(x) => pwm::Response::GpioValue(idx, x.into_gpio_value()),
                Err(x) => pwm::Response::ErrInvalidArgs,
            },
        };

        Ok(resp.to_frame())
    }

    pub fn process_frame<P: PlatformData>(
        &mut self,
        platf: &mut P,
        frame: ha::MsgFrame,
    ) -> Result<ha::MsgFrame, ha::MsgError> {
        match ha::CodeCategory::categorize(&frame.code) {
            ha::CodeCategory::ReqGeneric => self.process_generic(platf, frame),
            ha::CodeCategory::ReqGpio => self.process_pwm(platf, frame),
            _ => Err(ha::MsgError::UnknownCode),
        }
    }
}
