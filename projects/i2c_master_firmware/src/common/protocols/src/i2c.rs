/// HA Protocol implementation for I2C master control

use crate::ha;
use heapless::Vec;

/// Enums for I2C stuff
#[derive(Debug)]
pub enum Request {
    Write(u8, Vec<u8, 32>),  // size TBD
    Read(u8, Vec<u8, 32>), // size TBD
}
impl Request {
    pub fn consume_frame(ff: ha::MsgFrame) -> Result<Self, ha::MsgError> {
        match ff.code {
            ha::Code::I2cWrite => {
                let mut argp = ha::ArgParser::new(&ff.data.as_slice());

                let addr = match argp.consume_u8() {
                    Some(x) => x,
                    None => return Err(ha::MsgError::InvalidArg),
                };

                let data = match argp.consume_u8() {
                    //size de la data TBD
                    Some(size) => {
                        let mut vec = Vec::new();
                        for _ in 0..size {
                            match argp.consume_u8() {
                                Some(byte) => vec.push(byte),
                                None => return Err(ha::MsgError::InvalidArg),
                            };
                        }
                        // La valeur de vec est la valeur renvoyée pour Some(size) et affecte a data
                        vec
                    }
                    None => return Err(ha::MsgError::InvalidArg),
                };

                Ok(Self::Write(addr, data))
            }

            ha::Code::I2cRead => {
                let mut argp = ha::ArgParser::new(&ff.data.as_slice());

                let addr = match argp.consume_u8() {
                    Some(x) => x,
                    None => return Err(ha::MsgError::InvalidArg),
                };

                let data = match argp.consume_u8() {
                    //size de la data TBD
                    Some(size) => {
                        let mut vec = Vec::new();
                        for _ in 0..size {
                            match argp.consume_u8() {
                                Some(byte) => vec.push(byte),
                                None => return Err(ha::MsgError::InvalidArg),
                            };
                        }
                        // La valeur de vec est la valeur renvoyée pour Some(size) et affecte a data
                        vec
                    }
                    None => return Err(ha::MsgError::InvalidArg),
                };

                Ok(Self::Read(addr, data))
            }
            _ => Err(ha::MsgError::NotARequest(ff.code))
        }
    }
}
}
