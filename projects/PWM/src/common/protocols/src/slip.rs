const END: u8     = 0xC0;
const ESC: u8     = 0xDB;
const ESC_END: u8 = 0xDC;
const ESC_ESC: u8 = 0xDD;

#[derive(Debug)]
pub enum SlipErrorCode {
    BadEsc,
    BufferFull,
}

#[derive(Debug)]
pub struct SlipError {
    pub pos: usize,
    pub code: SlipErrorCode,
}

impl From<SlipErrorCode> for SlipError {
    fn from(err: SlipErrorCode) -> Self {
        Self {pos: 0, code: err}
    }
}

pub struct BasicBuffer<const CAPACITY: usize> {
    idx: usize,
    buf: [u8; CAPACITY],
}

pub struct Decoder<const CAPACITY: usize> {
    buf: BasicBuffer<CAPACITY>,
    is_escaping: bool
}

pub struct Encoder<const CAPACITY: usize> {
    buf: BasicBuffer<CAPACITY>,
}

////////////////////////////////////////////

impl<const CAPACITY: usize> BasicBuffer<CAPACITY> {
    pub fn new() -> Self {
        Self {idx: 0, buf: [0u8; CAPACITY] }
    }

    pub fn reset(&mut self) {
        self.idx = 0;
    }

    pub fn slice(&self) -> &[u8] {
        &self.buf[..self.idx]
    }

    pub fn put(&mut self, c: u8)-> Result<(), SlipErrorCode> {
        if self.idx >= CAPACITY {
            Err(SlipErrorCode::BufferFull)
        }

        else {
            self.buf[self.idx] = c;
            self.idx+=1;
            Ok(())
        }
    }
}

////////////////////////////////////////////

impl<const CAPACITY: usize> Decoder<CAPACITY> {
    pub fn new() -> Self {
        Self {
            buf: BasicBuffer::new(),
            is_escaping: false
        }
    }

    pub fn reset(&mut self) {
        self.is_escaping = false;
        self.buf.reset();
    }

    pub fn slice(&self) -> &[u8] {
        self.buf.slice()
    }

    pub fn feed(&mut self, input: &[u8]) -> Result<(usize, bool), SlipError> {
        let mut i = 0;

        while i < input.len() {
            let c = input[i]; // Consume 1 char from input buffer
            i += 1;           // Increment counter
            
            if self.is_escaping {
                self.is_escaping = false;
                match c {
                    ESC_END => {
                        match self.buf.put(END) {
                            Ok(_) => {},
                            Err(code) => {return Err(SlipError{ pos: i, code: code});}
                        }
                    }

                    ESC_ESC => {
                        match self.buf.put(ESC) {
                            Ok(_) => {},
                            Err(code) => {return Err(SlipError{ pos: i, code: code});}
                        }
                    }

                    _ => {return Err(SlipError{ pos: i, code: SlipErrorCode::BadEsc});}
                }
            }

            else {
                match c {
                    END => {
                        return Ok((i, true));
                    }

                    ESC => {
                        self.is_escaping = true;
                    }

                    // otherwise, put stuff in buffer
                    _ => {
                        match self.buf.put(c) {
                            Ok(_) => {},
                            Err(code) => {return Err(SlipError{ pos: i, code: code});}
                        }
                    }
                }
            }
        }

        // Input buffer processed, but no packet end yet detected
        Ok((i, false))
    }
}

impl<const CAPACITY: usize> Encoder<CAPACITY> {
    pub fn new() -> Self {
        Self {
            buf: BasicBuffer::new(),
        }
    }

    pub fn reset(&mut self) {
        self.buf.reset();
    }

    pub fn slice(&self) -> &[u8] {
        self.buf.slice()
    }

    pub fn feed(&mut self, input: &[u8]) -> Result<usize, SlipError>{
        let mut i = 0;
        while i < input.len() {
            let c = input[i];
            i += 1;

            match c {
                END => {
                    match self.buf.put(ESC) {
                        Ok(_) => {},
                        Err(code) => {return Err(SlipError{pos: i, code: code})}
                    }

                    match self.buf.put(ESC_END) {
                        Ok(_) => {},
                        Err(code) => {return Err(SlipError{pos: i, code: code})}
                    }
                },

                ESC => {
                    match self.buf.put(ESC) {
                        Ok(_) => {},
                        Err(code) => {return Err(SlipError{pos: i, code: code})}
                    }

                    match self.buf.put(ESC_ESC) {
                        Ok(_) => {},
                        Err(code) => {return Err(SlipError{pos: i, code: code})}
                    }
                },

                other => {
                    match self.buf.put(other) {
                        Ok(_)     => {}
                        Err(code) => {return Err(SlipError{pos: i, code: code})}
                    }
                }
            }
        }

        Ok(i)
    }

    pub fn finish(&mut self) -> Result<(), SlipErrorCode>{
        Ok(self.buf.put(END)?)
    }
}
