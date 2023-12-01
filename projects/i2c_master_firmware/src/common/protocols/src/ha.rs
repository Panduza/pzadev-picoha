/// Picoha simple protocol

use crc16;

use heapless::{
    Vec
};


#[derive(Debug)]
pub enum Code {
    // Generic requests
    Ping,
    ItfType,
    Version,
    IdGet,

    // GPIO interface specific codes
    GpioDirSet,
    GpioDirGet,
    GpioWrite,
    GpioRead,

    // Gpio interface specific resp codes
    GpioValue,
    GpioDir,

    // Response codes
    Good,
    ErrGeneric,
    ErrCRC,
    ErrUnknownCode,
    ErrInvalidArgs,
    ErrBusy,

    VersionResp,
    ItfTypeResp,
    IdResp,

    // I2C master interface specific codes

    I2cWrite,
    I2cRead,
    I2CWriteRead,
}

impl Code {
    pub fn from_slice(ss: &[u8; 2]) -> Option<Self> {
        Self::from_u16(u16::from_be_bytes([ss[0], ss[1]]))
    }

    pub fn from_u16(code: u16) -> Option<Self> {
        match code {
            // Generic requests
            0x0000 => Some(Self::Ping           ),
            0x0001 => Some(Self::ItfType        ),
            0x0002 => Some(Self::Version        ),
            0x0003 => Some(Self::IdGet          ),
            //0x0004 => Some(Self::IdSet          ),

            // GPIO interface specific codes
            0x0100 => Some(Self::GpioDirSet     ),
            0x0101 => Some(Self::GpioDirGet     ),
            0x0102 => Some(Self::GpioRead       ),
            0x0103 => Some(Self::GpioWrite      ),

            // Response codes for GPIO starting from 0xFEFF
            0xFDFF => Some(Self::GpioValue      ),
            0xFDFE => Some(Self::GpioDir        ),

            // Response codes stating from FFFF
            0xFFFF => Some(Self::Good           ),
            0xFFFE => Some(Self::ErrGeneric     ),
            0xFFFD => Some(Self::ErrCRC         ),
            0xFFFC => Some(Self::ErrUnknownCode ),
            0xFFFB => Some(Self::ErrInvalidArgs ),
            0xFFFA => Some(Self::ErrBusy        ),

            0xFEFF => Some(Self::VersionResp    ),
            0xFEFE => Some(Self::ItfTypeResp    ),
            0xFEFD => Some(Self::IdResp         ),

            // I2C interface specific codes
            0x3B00 => Some(Self::I2cWrite       ),
            0x3B01 => Some(Self::I2cRead        ),
            0x3B02 => Some(Self::I2CWriteRead   ),

            _ => None,
        }
    }

    pub fn to_u16(&self) -> u16 {
        match self {
            Self::Ping           => 0x0000,
            Self::ItfType        => 0x0001,
            Self::Version        => 0x0002,
            Self::IdGet          => 0x0003,

            Self::GpioDirSet     => 0x0100,
            Self::GpioDirGet     => 0x0101,
            Self::GpioRead       => 0x0102,
            Self::GpioWrite      => 0x0103,

            // Response codes for GPIO
            Self::GpioValue      => 0xFDFF,
            Self::GpioDir        => 0xFDFE,

            // Response codes for generic calls
            Self::VersionResp    => 0xFEFF,
            Self::ItfTypeResp    => 0xFEFE,
            Self::IdResp         => 0xFEFD,

            // Generic status codes
            Self::Good           => 0xFFFF,
            Self::ErrGeneric     => 0xFFFE,
            Self::ErrCRC         => 0xFFFD,
            Self::ErrUnknownCode => 0xFFFC,
            Self::ErrInvalidArgs => 0xFFFB,
            Self::ErrBusy        => 0xFFFA,

            // I2C interface specific codes
            Self::I2cWrite       => 0x3B00,
            Self::I2cRead        => 0x3B01,
            Self::I2CWriteRead   => 0x3B02,
        }
    }
}

#[derive(Debug)]
pub enum CodeCategory {
    ReqGeneric,
    ReqGpio,

    RespGpio,
    RespGeneric,
    StatusGeneric,
}

impl CodeCategory {
    pub fn categorize(code: &Code) -> Self {
        match code {
            Code::Ping           => Self::ReqGeneric,
            Code::ItfType        => Self::ReqGeneric,
            Code::Version        => Self::ReqGeneric,
            Code::IdGet          => Self::ReqGeneric,

            Code::GpioDirGet     => Self::ReqGpio,
            Code::GpioDirSet     => Self::ReqGpio,
            Code::GpioWrite      => Self::ReqGpio,
            Code::GpioRead       => Self::ReqGpio,

            Code::GpioValue      => Self::RespGpio,
            Code::GpioDir        => Self::RespGpio,

            Code::VersionResp    => Self::RespGeneric,
            Code::ItfTypeResp    => Self::RespGeneric,
            Code::IdResp         => Self::RespGeneric,

            Code::Good           => Self::StatusGeneric,
            Code::ErrGeneric     => Self::StatusGeneric,
            Code::ErrCRC         => Self::StatusGeneric,
            Code::ErrUnknownCode => Self::StatusGeneric,
            Code::ErrInvalidArgs => Self::StatusGeneric,
            Code::ErrBusy        => Self::StatusGeneric,
        }
    }

    pub fn is_request(&self) -> bool {
        match self {
            Self::ReqGeneric | Self::ReqGpio => true,
            _                                => false,
        }
    }

    pub fn is_response(&self) -> bool {
        match self {
            Self::RespGpio | Self::RespGeneric | Self::StatusGeneric => true,
            _                                                        => false,
        }
    }
}

////////////////////////////

#[derive(Debug)]
pub enum ItfType {
    Dummy,
    Gpio,
}

impl ItfType {
    pub fn from_u8(x: u8) -> Option<Self> {
        match x {
            0x00 => Some(Self::Dummy),
            0x01 => Some(Self::Gpio),
            _    => None
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Self::Dummy => 0x00,
            Self::Gpio  => 0x01,
        }
    }
}

////////////////////////////

#[derive(Debug)]
pub enum MsgError {
    InvalidLength,

    InvalidCRC(u16, u16),
    UnknownCode,
    InvalidArg,

    NotARequest(Code),
}

#[derive(Debug)]
pub struct MsgFrame {
    pub code: Code,
    pub data: Vec<u8, 64>,
}

impl MsgError {
    pub fn to_frame(&self) -> MsgFrame {
        match self {
            Self::InvalidLength => MsgFrame::new(
                Code::ErrGeneric,
                "Invalid length".as_bytes()
            ),

            Self::InvalidCRC(_a, _b) => MsgFrame::new(
                Code::ErrGeneric,
                "CRC error".as_bytes()
            ),

            Self::UnknownCode => MsgFrame::new(
                Code::ErrUnknownCode,
                "Unknown code".as_bytes()
            ),

            Self::InvalidArg => MsgFrame::new(
                Code::ErrInvalidArgs,
                "Invalid argument".as_bytes(),
            ),

            Self::NotARequest(_c) => MsgFrame::new(
                Code::ErrUnknownCode,
                "Not a request code".as_bytes()
            ),
        }
    }
}

impl MsgFrame {
    pub fn new(code: Code, data: &[u8]) -> Self {
        Self {
            code,
            data: Vec::from_slice(data).unwrap()
        }
    }

    pub fn from_slice(ss: &[u8]) -> Result<Self, MsgError> {
        // Initial length check
        // 4: 2 for code + 2 for crc
        if ss.len() < 4 {
            return Err(MsgError::InvalidLength);
        }

        // Compute and validate CRC
        let crc_frame = u16::from_be_bytes([ss[ss.len()-2], ss[ss.len()-1]]);

        let crc_real: u16 = crc16::State::<crc16::CCITT_FALSE>::calculate(
            &ss[..ss.len()-2]
        );

        if crc_real != crc_frame {
            return Err(MsgError::InvalidCRC(crc_real, crc_frame));
        }

        let code     = match Code::from_slice(&ss[..2].try_into().unwrap()) {
            Some(x) => x,
            None    => return Err(MsgError::UnknownCode)
        };

        Ok(Self {
            code: code,
            data: match Vec::from_slice(&ss[2..ss.len()-2]) {
                Ok(x)  => x,
                Err(_) => {return Err(MsgError::InvalidLength);},
            },
        })
    }

    pub fn crc(&self) -> u16 {
        let code_u16 = self.code.to_u16();
        let mut crc  = crc16::State::<crc16::CCITT_FALSE>::new();

        crc.update(&code_u16.to_be_bytes() );
        crc.update(self.data.as_slice());
        
        crc.get()
    }
}

////////////////////////////

/// Simple utility struct to consume args
pub struct ArgParser<'a> {
    buf: &'a [u8],
    idx: usize,
}

impl<'a> ArgParser<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self {
            buf: buf,
            idx: 0
        }
    }

    ///////////////////////
    
    pub fn consume_u8(&mut self) -> Option<u8> {
        if self.idx < self.buf.len() {
            let c = self.buf[self.idx];
            self.idx += 1;

            Some(c)
        }

        else {
            None
        }
    }

    pub fn consume_u16(&mut self) -> Option<u16> {
        if self.idx < (self.buf.len()-1) {
                let x = u16::from_be_bytes([self.buf[self.idx], self.buf[self.idx+1]]);
                self.idx += 2;

                Some(x)
        }

        else {
            None
        }
    }
}
