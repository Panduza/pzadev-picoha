/// HA Protocol implementation for generic calls

use crate::ha;
use heapless::Vec;

#[derive(Debug)]
pub enum Request {
    Ping,
    ItfType,
    Version,
    IdGet
}

impl Request {
    pub fn consume_frame(ff: ha::MsgFrame) -> Result<Self, ha::MsgError> {
        match ff.code {
            ha::Code::Ping    => Ok(Self::Ping   ),
            ha::Code::ItfType => Ok(Self::ItfType),
            ha::Code::Version => Ok(Self::Version),
            ha::Code::IdGet   => Ok(Self::IdGet  ),

            _                 => Err(ha::MsgError::NotARequest(ff.code))
        }
    }
}

#[derive(Debug)]
pub enum Response<'a> {
    Good,
    VersionResp(&'a str),
    ItfTypeResp(ha::ItfType),
    IdResp(&'a [u8]),
}

impl<'a> Response<'a> {
    pub fn to_frame(&self) -> ha::MsgFrame {
        match self {
            Self::Good => ha::MsgFrame {
                code: ha::Code::Good,
                data: Vec::new(),
            },

            Self::VersionResp(version_str) => ha::MsgFrame {
                code: ha::Code::VersionResp,
                data: Vec::from_slice(version_str.as_bytes()).unwrap(),
            },

            Self::ItfTypeResp(itf_type) => ha::MsgFrame {
                code: ha::Code::ItfTypeResp,
                data: Vec::from_slice(&[itf_type.to_u8()]).unwrap(),
            },

            Self::IdResp(id_data) => ha::MsgFrame {
                code: ha::Code::IdResp,
                data: Vec::from_slice(id_data).unwrap(),
            }
        }
    }
}
