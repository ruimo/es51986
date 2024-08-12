use crate::{Output, OUTPUT_LENGTH};

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    LengthError {
        len: usize
    },
    NoLfAfterCr,
    InvalidRange(u8),
    InvalidFunction(u8),
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
