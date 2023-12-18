/// HA Protocol implementation for PWM control
use crate::ha;
use heapless::Vec;

// #[derive(Debug)]
// pub enum PwmOutputState {
//     Disable,
//     Enable,
// }

// impl PwmOutputState {
//     pub fn from_u8(x: u8) -> Option<Self> {
//         match x {
//             0 => Some(Self::Disable),
//             1 => Some(Self::Enable),
//             _ => None,
//         }
//     }

//     pub fn to_u8(&self) -> u8 {
//         match self {
//             Self::Disable => 0,
//             Self::Enable => 1,
//         }
//     }
// }

//////////////////////////////////////

#[derive(Debug)]
pub enum Request {
    Enable,
    Disable,
    SetFrequency(f32),
    GetFrequency,
    SetDutyCycle(f32),
    GetDutycycle,
}

impl Request {
    pub fn consume_frame(ff: ha::MsgFrame) -> Result<Self, ha::MsgError> {
        match ff.code {
            ha::Code::SetFrequency => {
                let mut argp = ha::ArgParser::new(&ff.data.as_slice());

                let frequency_value = match argp.consume_f32() {
                    Some(x) => x,
                    None => {
                        return Err(ha::MsgError::InvalidArg);
                    }
                };

                Ok(Self::SetFrequency(frequency_value))
            }

            ha::Code::GetFrequency => Ok(Self::GetFrequency),

            ha::Code::SetDutyCycle => {
                let mut argp = ha::ArgParser::new(&ff.data.as_slice());

                let duty_cycle_value = match argp.consume_f32() {
                    Some(x) => x,
                    None => {
                        return Err(ha::MsgError::InvalidArg);
                    }
                };

                Ok(Self::SetDutyCycle(duty_cycle_value))
            }

            ha::Code::GetDutycycle => Ok(Self::GetDutycycle),

            _ => Err(ha::MsgError::NotARequest(ff.code)),
        }
    }
}

//////////////////////////////////////

#[derive(Debug)]
pub enum Response<'a> {
    Good,

    Frequency(f32),
    DutyCycle(f32),

    ErrInvalidArgs,
    ErrGeneric(&'a str),
}

// TODO // Configure framesize at crate level?
// impl<'a> Response<'a> {
//     pub fn to_frame(&self) -> ha::MsgFrame {
//         match self {
//             Self::Good => ha::MsgFrame {
//                 code: ha::Code::Good,
//                 data: Vec::new(),
//             },
//             Self::Frequency(freq) => ha::MsgFrame {
//                 code: ha::Code::Frequency,
//                 data: Vec::from_slice(freq.to_f32()).unwrap(),
//             },

//             Self::DutyCycle(duty) => ha::MsgFrame {
//                 code: ha::Code::DutyCycle,
//                 data: Vec::from_slice(&(duty.to_f32())).unwrap(),
//             },

//             ///////////////////////////////////
//             Self::ErrInvalidArgs => ha::MsgFrame {
//                 code: ha::Code::ErrInvalidArgs,
//                 data: Vec::new(),
//             },

//             Self::ErrGeneric(reason) => ha::MsgFrame {
//                 code: ha::Code::ErrGeneric,
//                 data: Vec::from_slice(reason.as_bytes()).unwrap(),
//             },
//         }
//     }
// }
