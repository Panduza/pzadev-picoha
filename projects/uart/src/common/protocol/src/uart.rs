/// HA Protocol implementation for UART control

use crate::ha;
use heapless::Vec;

#[derive(Debug)]
pub enum HWFlowControlState {
    Disable,
    Enable,
}

impl HWFlowControlState {
    pub fn from_u8(x: u8) -> Option<Self> {
        match x {
            0 => Some(Self::Disable),
            1 => Some(Self::Enable),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Self::Disable  => 0,
            Self::Enable => 1,
        }
    }

//////////////////////////////////////

#[derive(Debug)]
pub enum Request {
    DataTX(data),
    DataRXGet(data),
    BaudSet(u32),
    BaudGet,
    SetParity(u8),
    SetStopBit(u8),
    SetDataSz(u8),
    HWFlowControl(HWFlowControlState),
    ComErrStart,
    ComErrSize,
}

impl Request {
    pub fn consume_frame(ff: ha::MsgFrame) -> Result<Self, ha::MsgError> {
        match ff.code {
            // TODO
            ha::Code::DataTX => {
                
            },

            // TODO
            ha::Code::DataRXGet => {
                
            },

            ha::Code::BaudSet => {
                let mut argp = ha::ArgParser::new(&ff.data.as_slice());

                let baud_value = match argp.consume_u32() {
                    Some(x) => x,
                    None    => {return Err(ha::MsgError::InvalidArg);}
                };

                Ok(Self::BaudSet(baud_value))
            },

            ha::Code::BaudGet => {
                Ok(Self::BaudGet())
            }

            ha::Code::SetParity => {
                let mut argp = ha::ArgParser::new(&ff.data.as_slice());

                let parity_bit = match argp.consume_u8() {
                    Some(x) => x,
                    None    => {return Err(ha::MsgError::InvalidArg);}
                };

                Ok(Self::SetParity(parity_bit))
            }

            ha::Code::SetStopBit => {
                let mut argp = ha::ArgParser::new(&ff.data.as_slice());

                let stop_bit = match argp.consume_u8() {
                    Some(x) => x,
                    None    => {return Err(ha::MsgError::InvalidArg);}
                };

                Ok(Self::SetStopBit(stop_bit))
            }

            ha::Code::SetDataSz => {
                let mut argp = ha::ArgParser::new(&ff.data.as_slice());

                let data_size = match argp.consume_u8() {
                    Some(x) => x,
                    None    => {return Err(ha::MsgError::InvalidArg);}
                };

                Ok(Self::SetDataSz(data_size))
            }

            ha::Code::HWFlowControl => {
                let mut argp = ha::ArgParser::new(&ff.data.as_slice());

                let flowcontrol_state = match argp.consume_u8() {
                    Some(x) => match HWFlowControlState::from_u8(x) {
                        Some(v) => v,
                        None    => {return Err(ha::MsgError::InvalidArg);}
                    },
                    None => {return Err(ha::MsgError::InvalidArg);}
                };

                Ok(Self::HWFlowControl(flowcontrol_state))
            }

            ha::Code::ComErrStart => {
                Ok(Self::ComErrStart())
            }

            ha::Code::ComErrSize => {
                Ok(Self::ComErrSize())
            }

            _ => Err(ha::MsgError::NotARequest(ff.code))
        }
    }
}

//////////////////////////////////////

#[derive(Debug)]
pub enum Response<'a> {
    Good,

    DataRX(u8, GpioValue),
    Baud(u32),

    ErrInvalidArgs,
    ErrGeneric(&'a str),
}

// TODO // Configure framesize at crate level?
impl<'a> Response<'a> {
    pub fn to_frame(&self) -> ha::MsgFrame {
        match self {
            Self::Good => {
                ha::MsgFrame {
                    code: ha::Code::Good,
                    data: Vec::new(),
                }
            }
            // TODO
            Self::DataRX(data) => {
                ha::MsgFrame {
                    
                }
            }

            Self::Baud(baud_value) => {
                ha::MsgFrame {
                    code: ha::Code::Baud,
                    data: Vec::from_slice(baud_value.to_u8()).unwrap()
                }
            }
            
            ///////////////////////////////////
            
            Self::ErrInvalidArgs => {
                ha::MsgFrame {
                    code: ha::Code::ErrInvalidArgs,
                    data: Vec::new()
                }
            }

            Self::ErrGeneric(reason) => {
                ha::MsgFrame {
                    code: ha::Code::ErrGeneric,
                    data: Vec::from_slice(reason.as_bytes()).unwrap(),
                }
            }
        }
    }
}