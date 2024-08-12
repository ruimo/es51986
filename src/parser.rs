use serde::{Deserialize, Serialize};

use crate::{Output, OUTPUT_LENGTH};

/// Parse errors
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParseError {
    /// Length error. ES51986's data is 9 byte long followed by CRLF characters.
    /// If the length is wrong, this error will be returned.
    LengthError {
        /// The length detected. If the data was '1234567890\r\n' then this 'len' will become 10.
        len: usize
    },
    /// The first byte of the data is range. If the range byte is invalid, this error will be returned.
    /// The u8 data is actual data byte.
    InvalidRange(u8),
    /// The 5th byte of the data is function. If the function is invalid, this error will be returned.
    /// The u8 data is actual data byte.
    InvalidFunction(u8),
    /// The 1-4 byte of the data is digits. If the digits is invalid, this error will be returned.
    /// The u8 data is actual data byte.
    InvalidDigit(u8),
}

const CR: u8 = 0x0d;
const LF: u8 = 0x0a;

enum ParserState {
    Idle,
    FoundCr,
}

pub struct Parser {
    state: ParserState,
    buf: Vec<u8>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            state: ParserState::Idle,
            buf: vec![]
        }
    }
    
    fn parse_ch(&mut self, ch: u8) -> Result<Option<Output>, ParseError> {
        match self.state {
            ParserState::Idle => {
                if ch == CR {
                    self.state = ParserState::FoundCr;
                    let result = Output::parse(&self.buf);
                    self.buf.clear();
                    result.map(|o| Some(o))
                } else if ch == LF {
                    self.state = ParserState::Idle;
                    let result = Output::parse(&self.buf);
                    self.buf.clear();
                    result.map(|o| Some(o))
                } else {
                    self.buf.push(ch);
                    let len = self.buf.len();
                    if OUTPUT_LENGTH < len {
                        let drain_amount = len - OUTPUT_LENGTH;
                        self.buf.drain(..drain_amount);
                        Err(ParseError::LengthError { len })
                    } else {
                        Ok(None)
                    }
                }
            }
            ParserState::FoundCr => {
                if ch == LF {
                    self.buf.clear();
                    Ok(None)
                } else {
                    self.buf.push(ch);
                    self.state = ParserState::Idle;
                    Ok(None)
                }
            }
        }
    }

    pub fn parse(&mut self, input: &[u8]) -> Vec<Result<Output, ParseError>> {
        let mut results: Vec<Result<Output, ParseError>> = vec![];
        for ch in input.iter() {
            match self.parse_ch(*ch) {
                Ok(Some(out)) => results.push(Ok(out)),
                Ok(None) => {},
                Err(err) => results.push(Err(err)),
            }
        }

        results
    }
}
