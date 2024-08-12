use parser::ParseError;

pub mod parser;

#[derive(Debug, Clone, PartialEq)]
pub enum Range {
    Range0,
    Range1,
    Range2,
    Range3,
    Range4,
    Range5,
    Range6,
}

impl Range {
    pub fn parse(c: u8) -> Result<Range, ParseError> {
        match c {
            0x30 => Ok(Self::Range0),
            0x31 => Ok(Self::Range1),
            0x32 => Ok(Self::Range2),
            0x33 => Ok(Self::Range3),
            0x34 => Ok(Self::Range4),
            0x35 => Ok(Self::Range5),
            0x36 => Ok(Self::Range6),
            _ => Err(ParseError::InvalidRange(c)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Function {
    Voltage,
    MicroAmpere,
    MilliAmpere,
    AutoAmpere,
    ManualAmpere,
    Ohm,
    Continuity,
    Diode,
    Frequency,
    Capacitor,
    Temperature,
    Adp0,
    Adp1,
    Adp2,
    Adp3,
}

impl Function {
    pub fn parse(c: u8) -> Result<Function, ParseError> {
        match c {
            0x3b => Ok(Self::Voltage),
            0x3d => Ok(Self::MicroAmpere),
            0x3f => Ok(Self::MilliAmpere),
            0x30 => Ok(Self::AutoAmpere),
            0x39 => Ok(Self::ManualAmpere),
            0x33 => Ok(Self::Ohm),
            0x35 => Ok(Self::Continuity),
            0x31 => Ok(Self::Diode),
            0x32 => Ok(Self::Frequency),
            0x36 => Ok(Self::Capacitor),
            0x34 => Ok(Self::Temperature),
            0x3e => Ok(Self::Adp0),
            0x3c => Ok(Self::Adp1),
            0x38 => Ok(Self::Adp2),
            0x3a => Ok(Self::Adp3),
            _ => Err(ParseError::InvalidFunction(c)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sign(bool);

pub const SIGN_PLUS: Sign = Sign(false);
pub const SIGN_MINUS: Sign = Sign(true);

impl Sign {
    pub fn is_minus(self) -> bool {
        self.0
    }

    pub fn is_not_minus(self) -> bool {
        !self.is_minus()
    }
}

impl Into<i8> for Sign {
    fn into(self) -> i8 {
        if self.is_minus() { -1 } else { 1 }
    }
}

impl Into<i16> for Sign {
    fn into(self) -> i16 {
        if self.is_minus() { -1 } else { 1 }
    }
}

impl Into<i32> for Sign {
    fn into(self) -> i32 {
        if self.is_minus() { -1 } else { 1 }
    }
}

impl Into<i64> for Sign {
    fn into(self) -> i64 {
        if self.is_minus() { -1 } else { 1 }
    }
}

impl Into<i128> for Sign {
    fn into(self) -> i128 {
        if self.is_minus() { -1 } else { 1 }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Status {
    pub temperature_unit: TemperatureUnit,
    pub sign: Sign,
    pub is_battery_depleted: bool,
    pub is_overflow: bool,
}

impl Status {
    pub fn parse(c: u8) -> Status {
        Status {
            temperature_unit: if (c & 0x08) != 0 { TemperatureUnit::Celsius } else { TemperatureUnit::Fahrenheit },
            sign: Sign((c & 0x04) != 0),
            is_battery_depleted: (c & 0x02) != 0,
            is_overflow: (c & 0x01) != 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Option2 {
    pub is_dc: bool,
    pub is_ac: bool,
    pub is_auto: bool,
}

impl Option2 {
    pub fn parse(c: u8) -> Option2 {
        Option2 {
            is_dc: (c & 0x08) != 0,
            is_ac: (c & 0x04) != 0,
            is_auto: (c & 0x02) != 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrefixUnit {
    Mega,
    Kilo,
    None,
    Millis,
    Micro,
    Nano,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BaseUnit {
    Ampere,
    Volt,
    Ohm,
    Hearts,
    Farad,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValueUnit {
    prefix_unit: PrefixUnit,
    base_unit: BaseUnit,
}

impl ValueUnit {
    pub fn new(prefix_unit: PrefixUnit, base_unit: BaseUnit) -> Self {
        Self {
            prefix_unit, base_unit
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Digits {
    digits: [u8; 4],
}

pub enum DigitRadix {
    Zero,
    Minus1,
    Minus2,
    Minus3,
}

impl Digits {
    fn parse_digit(c: u8) -> Result<u8, ParseError> {
        if 0x30 <= c && c <= 0x39 {
            Ok(c - 0x30)
        } else {
            Err(ParseError::InvalidDigit(c))
        }
    }

    pub fn parse(input: &[u8], loc: usize) -> Result<Digits, ParseError> {
        let digits: [u8; 4] = [
            Self::parse_digit(input[loc])?,
            Self::parse_digit(input[loc + 1])?,
            Self::parse_digit(input[loc + 2])?,
            Self::parse_digit(input[loc + 3])?,
        ];
        Ok(Digits { digits })
    }

    pub fn to_value(&self, radix: DigitRadix) -> String {
        match radix {
            DigitRadix::Zero => format!(
                "{}", 
                self.digits[0] as usize * 1000 + self.digits[1] as usize * 100 + self.digits[2] as usize * 10 + self.digits[3] as usize
            ),
            DigitRadix::Minus1 => format!(
                "{}.{}",
                self.digits[0] as usize * 100 + self.digits[1] as usize * 10 + self.digits[2] as usize, self.digits[3]
            ),
            DigitRadix::Minus2 => format!(
                "{}.{}{}",
                self.digits[0] as usize * 10 + self.digits[1] as usize, self.digits[2], self.digits[3]
            ),
            DigitRadix::Minus3 => format!("{}.{}{}{}", self.digits[0], self.digits[1], self.digits[2], self.digits[3]),
        }
    }
}

const OUTPUT_LENGTH: usize = 9;

#[derive(Debug, Clone, PartialEq)]
pub struct Output {
    pub range: Range,
    pub digits: Digits,
    pub function: Function,
    pub status: Status,
    pub option2: Option2,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OutputValue {
    pub digits: String,
    pub value_unit: ValueUnit,
}

impl Output {
    pub fn parse(input: &[u8]) -> Result<Output, ParseError> {
        if input.len() == OUTPUT_LENGTH {
            let range: Range = Range::parse(input[0])?;
            let function: Function = Function::parse(input[5])?;
            let status: Status = Status::parse(input[6]);
            // option1 is not used.
            let option2: Option2 = Option2::parse(input[8]);
            
            Ok(
                Output { range, digits: Digits::parse(input, 1)?, function, status, option2 }
            )
        } else {
            Err(ParseError::LengthError { len: input.len() })
        }
    }

    pub fn get_value(&self) -> Option<OutputValue> {
        match self.range {
            Range::Range0 => match self.function {
                Function::Voltage => {
                    let digits = self.digits.to_value(DigitRadix::Minus3);
                    let value_unit = ValueUnit::new(PrefixUnit::None, BaseUnit::Volt);
                    Some(OutputValue { digits, value_unit })
                }
                Function::MicroAmpere => {
                    let digits = self.digits.to_value(DigitRadix::Minus1);
                    let value_unit = ValueUnit::new(PrefixUnit::Micro, BaseUnit::Ampere);
                    Some(OutputValue { digits, value_unit })
                }
                Function::MilliAmpere => {
                    let digits = self.digits.to_value(DigitRadix::Minus2);
                    let value_unit = ValueUnit::new(PrefixUnit::Millis, BaseUnit::Ampere);
                    Some(OutputValue { digits, value_unit })
                }
                Function::AutoAmpere => {
                    let digits = self.digits.to_value(DigitRadix::Minus3);
                    let value_unit = ValueUnit::new(PrefixUnit::None, BaseUnit::Ampere);
                    Some(OutputValue { digits, value_unit })
                }
                Function::ManualAmpere => {
                    let digits = self.digits.to_value(DigitRadix::Minus3);
                    let value_unit = ValueUnit::new(PrefixUnit::None, BaseUnit::Ampere);
                    Some(OutputValue { digits, value_unit })
                }
                Function::Ohm => {
                    let digits = self.digits.to_value(DigitRadix::Minus1);
                    let value_unit = ValueUnit::new(PrefixUnit::None, BaseUnit::Ohm);
                    Some(OutputValue { digits, value_unit })
                }
                Function::Frequency => {
                    let digits = self.digits.to_value(DigitRadix::Minus3);
                    let value_unit = ValueUnit::new(PrefixUnit::Kilo, BaseUnit::Hearts);
                    Some(OutputValue { digits, value_unit })
                }
                Function::Capacitor => {
                    let digits = self.digits.to_value(DigitRadix::Minus3);
                    let value_unit = ValueUnit::new(PrefixUnit::Nano, BaseUnit::Farad);
                    Some(OutputValue { digits, value_unit })
                }
                _ => None,
            }
            Range::Range1 => match self.function {
                Function::Voltage => {
                    let digits = self.digits.to_value(DigitRadix::Minus2);
                    let value_unit = ValueUnit::new(PrefixUnit::None, BaseUnit::Volt);
                    Some(OutputValue { digits, value_unit })
                }
                Function::MicroAmpere => {
                    let digits = self.digits.to_value(DigitRadix::Zero);
                    let value_unit = ValueUnit::new(PrefixUnit::Micro, BaseUnit::Ampere);
                    Some(OutputValue { digits, value_unit })
                }
                Function::MilliAmpere => {
                    let digits = self.digits.to_value(DigitRadix::Minus1);
                    let value_unit = ValueUnit::new(PrefixUnit::Millis, BaseUnit::Ampere);
                    Some(OutputValue { digits, value_unit })
                }
                Function::AutoAmpere => {
                    let digits = self.digits.to_value(DigitRadix::Minus2);
                    let value_unit = ValueUnit::new(PrefixUnit::None, BaseUnit::Ampere);
                    Some(OutputValue { digits, value_unit })
                }
                Function::Ohm => {
                    let digits = self.digits.to_value(DigitRadix::Minus3);
                    let value_unit = ValueUnit::new(PrefixUnit::Kilo, BaseUnit::Ohm);
                    Some(OutputValue { digits, value_unit })
                }
                Function::Frequency => {
                    let digits = self.digits.to_value(DigitRadix::Minus2);
                    let value_unit = ValueUnit::new(PrefixUnit::Kilo, BaseUnit::Hearts);
                    Some(OutputValue { digits, value_unit })
                }
                Function::Capacitor => {
                    let digits = self.digits.to_value(DigitRadix::Minus2);
                    let value_unit = ValueUnit::new(PrefixUnit::Nano, BaseUnit::Farad);
                    Some(OutputValue { digits, value_unit })
                }
                _ => None,
            }
            Range::Range2 => match self.function {
                Function::Voltage => {
                    let digits = self.digits.to_value(DigitRadix::Minus1);
                    let value_unit = ValueUnit::new(PrefixUnit::None, BaseUnit::Volt);
                    Some(OutputValue { digits, value_unit })
                }
                Function::Ohm => {
                    let digits = self.digits.to_value(DigitRadix::Minus2);
                    let value_unit = ValueUnit::new(PrefixUnit::Kilo, BaseUnit::Ohm);
                    Some(OutputValue { digits, value_unit })
                }
                Function::Frequency => {
                    let digits = self.digits.to_value(DigitRadix::Minus1);
                    let value_unit = ValueUnit::new(PrefixUnit::Kilo, BaseUnit::Hearts);
                    Some(OutputValue { digits, value_unit })
                }
                Function::Capacitor => {
                    let digits = self.digits.to_value(DigitRadix::Minus1);
                    let value_unit = ValueUnit::new(PrefixUnit::Nano, BaseUnit::Farad);
                    Some(OutputValue { digits, value_unit })
                }
                _ => None,
            }
            Range::Range3 => match self.function {
                Function::Voltage => {
                    let digits = self.digits.to_value(DigitRadix::Zero);
                    let value_unit = ValueUnit::new(PrefixUnit::None, BaseUnit::Volt);
                    Some(OutputValue { digits, value_unit })
                }
                Function::Ohm => {
                    let digits = self.digits.to_value(DigitRadix::Minus1);
                    let value_unit = ValueUnit::new(PrefixUnit::Kilo, BaseUnit::Ohm);
                    Some(OutputValue { digits, value_unit })
                }
                Function::Frequency => {
                    let digits = self.digits.to_value(DigitRadix::Minus3);
                    let value_unit = ValueUnit::new(PrefixUnit::Mega, BaseUnit::Hearts);
                    Some(OutputValue { digits, value_unit })
                }
                Function::Capacitor => {
                    let digits = self.digits.to_value(DigitRadix::Minus3);
                    let value_unit = ValueUnit::new(PrefixUnit::Micro, BaseUnit::Farad);
                    Some(OutputValue { digits, value_unit })
                }
                _ => None,
            }
            Range::Range4 => match self.function {
                Function::Voltage => {
                    let digits = self.digits.to_value(DigitRadix::Minus1);
                    let value_unit = ValueUnit::new(PrefixUnit::Millis, BaseUnit::Volt);
                    Some(OutputValue { digits, value_unit })
                }
                Function::Ohm => {
                    let digits = self.digits.to_value(DigitRadix::Minus3);
                    let value_unit = ValueUnit::new(PrefixUnit::Mega, BaseUnit::Ohm);
                    Some(OutputValue { digits, value_unit })
                }
                Function::Frequency => {
                    let digits = self.digits.to_value(DigitRadix::Minus2);
                    let value_unit = ValueUnit::new(PrefixUnit::Mega, BaseUnit::Hearts);
                    Some(OutputValue { digits, value_unit })
                }
                Function::Capacitor => {
                    let digits = self.digits.to_value(DigitRadix::Minus2);
                    let value_unit = ValueUnit::new(PrefixUnit::Micro, BaseUnit::Farad);
                    Some(OutputValue { digits, value_unit })
                }
                _ => None,
            }
            Range::Range5 => match self.function {
                Function::Ohm => {
                    let digits = self.digits.to_value(DigitRadix::Minus2);
                    let value_unit = ValueUnit::new(PrefixUnit::Mega, BaseUnit::Ohm);
                    Some(OutputValue { digits, value_unit })
                }
                Function::Capacitor => {
                    let digits = self.digits.to_value(DigitRadix::Minus2);
                    let value_unit = ValueUnit::new(PrefixUnit::Micro, BaseUnit::Farad);
                    Some(OutputValue { digits, value_unit })
                }
                _ => None,
            }
            Range::Range6 => match self.function {
                Function::Capacitor => {
                    let digits = self.digits.to_value(DigitRadix::Minus3);
                    let value_unit = ValueUnit::new(PrefixUnit::Millis, BaseUnit::Farad);
                    Some(OutputValue { digits, value_unit })
                }
                _ => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use parser::Parser;

    use super::*;
    
    fn to_u8(s: &str) -> Vec<u8> {
        s.chars().map(|c| c as u8).collect()
    }

    #[test]
    fn voltage() {
        let inp: Vec<u8> = to_u8("00000;<0:\r\n");
        let results: Vec<Result<Output, ParseError>> = Parser::new().parse(&inp);
        assert_eq!(results.len(), 1);
        let out: &Output = results[0].as_ref().unwrap();
        assert_eq!(out.status, Status { temperature_unit: TemperatureUnit::Celsius, sign: SIGN_MINUS, is_battery_depleted: false, is_overflow: false });
        assert_eq!(out.function, Function::Voltage);
        assert_eq!(out.range, Range::Range0);
        assert_eq!(out.option2, Option2 { is_dc: true, is_ac: false, is_auto: true });
        assert_eq!(out.get_value(), Some(OutputValue { digits: "0.000".to_owned(), value_unit: ValueUnit { prefix_unit: PrefixUnit::None, base_unit: BaseUnit::Volt}}));

        let inp: Vec<u8> = to_u8("00002;80:\r\n");
        let results: Vec<Result<Output, ParseError>> = Parser::new().parse(&inp);
        assert_eq!(results.len(), 1);
        let out: &Output = results[0].as_ref().unwrap();
        assert_eq!(out.status, Status { temperature_unit: TemperatureUnit::Celsius, sign: SIGN_PLUS, is_battery_depleted: false, is_overflow: false });
        assert_eq!(out.function, Function::Voltage);
        assert_eq!(out.range, Range::Range0);
        assert_eq!(out.option2, Option2 { is_dc: true, is_ac: false, is_auto: true });
        assert_eq!(out.get_value(), Some(OutputValue { digits: "0.002".to_owned(), value_unit: ValueUnit { prefix_unit: PrefixUnit::None, base_unit: BaseUnit::Volt}}));

        let inp: Vec<u8> = to_u8("20989;806\r\n");
        let results: Vec<Result<Output, ParseError>> = Parser::new().parse(&inp);
        assert_eq!(results.len(), 1);
        let out: &Output = results[0].as_ref().unwrap();
        assert_eq!(out.status, Status { temperature_unit: TemperatureUnit::Celsius, sign: SIGN_PLUS, is_battery_depleted: false, is_overflow: false });
        assert_eq!(out.function, Function::Voltage);
        assert_eq!(out.range, Range::Range2);
        assert_eq!(out.option2, Option2 { is_dc: false, is_ac: true, is_auto: true });
        assert_eq!(out.get_value(), Some(OutputValue { digits: "98.9".to_owned(), value_unit: ValueUnit { prefix_unit: PrefixUnit::None, base_unit: BaseUnit::Volt}}));
    }

    #[test]
    fn ohm() {
        let inp: Vec<u8> = to_u8("560003902\r\n");
        let results: Vec<Result<Output, ParseError>> = Parser::new().parse(&inp);
        assert_eq!(results.len(), 1);
        let out: &Output = results[0].as_ref().unwrap();
        assert_eq!(out.status, Status { temperature_unit: TemperatureUnit::Celsius, sign: SIGN_PLUS, is_battery_depleted: false, is_overflow: true });
        assert_eq!(out.function, Function::Ohm);
        assert_eq!(out.range, Range::Range5);
        assert_eq!(out.option2, Option2 { is_dc: false, is_ac: false, is_auto: true });
        assert_eq!(out.get_value(), Some(OutputValue { digits: "60.00".to_owned(), value_unit: ValueUnit { prefix_unit: PrefixUnit::Mega, base_unit: BaseUnit::Ohm}}));

        let inp: Vec<u8> = to_u8("109853802\r\n");
        let results: Vec<Result<Output, ParseError>> = Parser::new().parse(&inp);
        assert_eq!(results.len(), 1);
        let out: &Output = results[0].as_ref().unwrap();
        assert_eq!(out.status, Status { temperature_unit: TemperatureUnit::Celsius, sign: SIGN_PLUS, is_battery_depleted: false, is_overflow: false });
        assert_eq!(out.function, Function::Ohm);
        assert_eq!(out.range, Range::Range1);
        assert_eq!(out.option2, Option2 { is_dc: false, is_ac: false, is_auto: true });
        assert_eq!(out.get_value(), Some(OutputValue { digits: "0.985".to_owned(), value_unit: ValueUnit { prefix_unit: PrefixUnit::Kilo, base_unit: BaseUnit::Ohm}}));

        let inp: Vec<u8> = to_u8("000003802\r\n");
        let results: Vec<Result<Output, ParseError>> = Parser::new().parse(&inp);
        assert_eq!(results.len(), 1);
        let out: &Output = results[0].as_ref().unwrap();
        assert_eq!(out.status, Status { temperature_unit: TemperatureUnit::Celsius, sign: SIGN_PLUS, is_battery_depleted: false, is_overflow: false });
        assert_eq!(out.function, Function::Ohm);
        assert_eq!(out.range, Range::Range0);
        assert_eq!(out.option2, Option2 { is_dc: false, is_ac: false, is_auto: true });
        assert_eq!(out.get_value(), Some(OutputValue { digits: "0.0".to_owned(), value_unit: ValueUnit { prefix_unit: PrefixUnit::None, base_unit: BaseUnit::Ohm}}));
    }

    #[test]
    fn capasitance() {
        let inp: Vec<u8> = to_u8("660006902\r\n");
        let results: Vec<Result<Output, ParseError>> = Parser::new().parse(&inp);
        assert_eq!(results.len(), 1);
        let out: &Output = results[0].as_ref().unwrap();
        assert_eq!(out.status, Status { temperature_unit: TemperatureUnit::Celsius, sign: SIGN_PLUS, is_battery_depleted: false, is_overflow: true });
        assert_eq!(out.function, Function::Capacitor);
        assert_eq!(out.range, Range::Range6);
        assert_eq!(out.option2, Option2 { is_dc: false, is_ac: false, is_auto: true });
        assert_eq!(out.get_value(), Some(OutputValue { digits: "6.000".to_owned(), value_unit: ValueUnit { prefix_unit: PrefixUnit::Millis, base_unit: BaseUnit::Farad}}));

        let inp: Vec<u8> = to_u8("211656802\r\n");
        let results: Vec<Result<Output, ParseError>> = Parser::new().parse(&inp);
        assert_eq!(results.len(), 1);
        let out: &Output = results[0].as_ref().unwrap();
        assert_eq!(out.status, Status { temperature_unit: TemperatureUnit::Celsius, sign: SIGN_PLUS, is_battery_depleted: false, is_overflow: false });
        assert_eq!(out.function, Function::Capacitor);
        assert_eq!(out.range, Range::Range2);
        assert_eq!(out.option2, Option2 { is_dc: false, is_ac: false, is_auto: true });
        assert_eq!(out.get_value(), Some(OutputValue { digits: "116.5".to_owned(), value_unit: ValueUnit { prefix_unit: PrefixUnit::Nano, base_unit: BaseUnit::Farad}}));

        let inp: Vec<u8> = to_u8("000226802\r\n");
        let results: Vec<Result<Output, ParseError>> = Parser::new().parse(&inp);
        assert_eq!(results.len(), 1);
        let out: &Output = results[0].as_ref().unwrap();
        assert_eq!(out.status, Status { temperature_unit: TemperatureUnit::Celsius, sign: SIGN_PLUS, is_battery_depleted: false, is_overflow: false });
        assert_eq!(out.function, Function::Capacitor);
        assert_eq!(out.range, Range::Range0);
        assert_eq!(out.option2, Option2 { is_dc: false, is_ac: false, is_auto: true });
        assert_eq!(out.get_value(), Some(OutputValue { digits: "0.022".to_owned(), value_unit: ValueUnit { prefix_unit: PrefixUnit::Nano, base_unit: BaseUnit::Farad}}));
    }

    #[test]
    fn frequency() {
        let inp: Vec<u8> = to_u8("000002802\r\n");
        let results: Vec<Result<Output, ParseError>> = Parser::new().parse(&inp);
        assert_eq!(results.len(), 1);
        let out: &Output = results[0].as_ref().unwrap();
        assert_eq!(out.status, Status { temperature_unit: TemperatureUnit::Celsius, sign: SIGN_PLUS, is_battery_depleted: false, is_overflow: false });
        assert_eq!(out.function, Function::Frequency);
        assert_eq!(out.range, Range::Range0);
        assert_eq!(out.option2, Option2 { is_dc: false, is_ac: false, is_auto: true });
        assert_eq!(out.get_value(), Some(OutputValue { digits: "0.000".to_owned(), value_unit: ValueUnit { prefix_unit: PrefixUnit::Kilo, base_unit: BaseUnit::Hearts}}));

        let inp: Vec<u8> = to_u8("210012802\r\n");
        let results: Vec<Result<Output, ParseError>> = Parser::new().parse(&inp);
        assert_eq!(results.len(), 1);
        let out: &Output = results[0].as_ref().unwrap();
        assert_eq!(out.status, Status { temperature_unit: TemperatureUnit::Celsius, sign: SIGN_PLUS, is_battery_depleted: false, is_overflow: false });
        assert_eq!(out.function, Function::Frequency);
        assert_eq!(out.range, Range::Range2);
        assert_eq!(out.option2, Option2 { is_dc: false, is_ac: false, is_auto: true });
        assert_eq!(out.get_value(), Some(OutputValue { digits: "100.1".to_owned(), value_unit: ValueUnit { prefix_unit: PrefixUnit::Kilo, base_unit: BaseUnit::Hearts}}));
    }

    #[test]
    fn lux() {
        let inp: Vec<u8> = to_u8("00136>800\r\n");
        let results: Vec<Result<Output, ParseError>> = Parser::new().parse(&inp);
        assert_eq!(results.len(), 1);
        let out: &Output = results[0].as_ref().unwrap();
        assert_eq!(out.status, Status { temperature_unit: TemperatureUnit::Celsius, sign: SIGN_PLUS, is_battery_depleted: false, is_overflow: false });
        assert_eq!(out.function, Function::Adp0);
        assert_eq!(out.range, Range::Range0);
        assert_eq!(out.option2, Option2 { is_dc: false, is_ac: false, is_auto: false });
        assert_eq!(&out.digits.to_value(DigitRadix::Zero), "136");
        assert_eq!(out.get_value(), None);
    }

    #[test]
    fn sound_level() {
        let inp: Vec<u8> = to_u8("00676<800\r\n");
        let results: Vec<Result<Output, ParseError>> = Parser::new().parse(&inp);
        assert_eq!(results.len(), 1);
        let out: &Output = results[0].as_ref().unwrap();
        assert_eq!(out.status, Status { temperature_unit: TemperatureUnit::Celsius, sign: SIGN_PLUS, is_battery_depleted: false, is_overflow: false });
        assert_eq!(out.function, Function::Adp1);
        assert_eq!(out.range, Range::Range0);
        assert_eq!(out.option2, Option2 { is_dc: false, is_ac: false, is_auto: false });
        assert_eq!(&out.digits.to_value(DigitRadix::Zero), "676");
        assert_eq!(out.get_value(), None);
    }

    #[test]
    fn temperature() {
        let inp: Vec<u8> = to_u8("060004900\r\n");
        let results: Vec<Result<Output, ParseError>> = Parser::new().parse(&inp);
        assert_eq!(results.len(), 1);
        let out: &Output = results[0].as_ref().unwrap();
        assert_eq!(out.status, Status { temperature_unit: TemperatureUnit::Celsius, sign: SIGN_PLUS, is_battery_depleted: false, is_overflow: true });
        assert_eq!(out.function, Function::Temperature);
        assert_eq!(out.range, Range::Range0);
        assert_eq!(out.option2, Option2 { is_dc: false, is_ac: false, is_auto: false });
        assert_eq!(&out.digits.to_value(DigitRadix::Zero), "6000");
        assert_eq!(out.get_value(), None);

        let inp: Vec<u8> = to_u8("000304800\r\n");
        let results: Vec<Result<Output, ParseError>> = Parser::new().parse(&inp);
        assert_eq!(results.len(), 1);
        let out: &Output = results[0].as_ref().unwrap();
        assert_eq!(out.status, Status { temperature_unit: TemperatureUnit::Celsius, sign: SIGN_PLUS, is_battery_depleted: false, is_overflow: false });
        assert_eq!(out.function, Function::Temperature);
        assert_eq!(out.range, Range::Range0);
        assert_eq!(out.option2, Option2 { is_dc: false, is_ac: false, is_auto: false });
        assert_eq!(&out.digits.to_value(DigitRadix::Zero), "30");
        assert_eq!(out.get_value(), None);
    }

    #[test]
    fn m_ampere() {
        let inp: Vec<u8> = to_u8("00002?<0:\r\n");
        let results: Vec<Result<Output, ParseError>> = Parser::new().parse(&inp);
        assert_eq!(results.len(), 1);
        let out: &Output = results[0].as_ref().unwrap();
        assert_eq!(out.status, Status { temperature_unit: TemperatureUnit::Celsius, sign: SIGN_MINUS, is_battery_depleted: false, is_overflow: false });
        assert_eq!(out.function, Function::MilliAmpere);
        assert_eq!(out.range, Range::Range0);
        assert_eq!(out.option2, Option2 { is_dc: true, is_ac: false, is_auto: true });
        assert_eq!(&out.digits.to_value(DigitRadix::Zero), "2");
        assert_eq!(out.get_value(), Some(OutputValue { digits: "0.02".to_owned(), value_unit: ValueUnit { prefix_unit: PrefixUnit::Millis, base_unit: BaseUnit::Ampere}}));
    }

    #[test]
    fn ampere() {
        let inp: Vec<u8> = to_u8("000019808\r\n");
        let results: Vec<Result<Output, ParseError>> = Parser::new().parse(&inp);
        assert_eq!(results.len(), 1);
        let out: &Output = results[0].as_ref().unwrap();
        assert_eq!(out.status, Status { temperature_unit: TemperatureUnit::Celsius, sign: SIGN_PLUS, is_battery_depleted: false, is_overflow: false });
        assert_eq!(out.function, Function::ManualAmpere);
        assert_eq!(out.range, Range::Range0);
        assert_eq!(out.option2, Option2 { is_dc: true, is_ac: false, is_auto: false });
        assert_eq!(&out.digits.to_value(DigitRadix::Zero), "1");
        assert_eq!(out.get_value(), Some(OutputValue { digits: "0.001".to_owned(), value_unit: ValueUnit { prefix_unit: PrefixUnit::None, base_unit: BaseUnit::Ampere}}));
    }

    #[test]
    fn only_cr() {
        let inp: Vec<u8> = to_u8("000019808\r000019808\r");
        let results: Vec<Result<Output, ParseError>> = Parser::new().parse(&inp);
        assert_eq!(results.len(), 2);
        let out: &Output = results[0].as_ref().unwrap();
        assert_eq!(out.status, Status { temperature_unit: TemperatureUnit::Celsius, sign: SIGN_PLUS, is_battery_depleted: false, is_overflow: false });
        assert_eq!(out.function, Function::ManualAmpere);
        assert_eq!(out.range, Range::Range0);
        assert_eq!(out.option2, Option2 { is_dc: true, is_ac: false, is_auto: false });
        assert_eq!(&out.digits.to_value(DigitRadix::Zero), "1");
        assert_eq!(out.get_value(), Some(OutputValue { digits: "0.001".to_owned(), value_unit: ValueUnit { prefix_unit: PrefixUnit::None, base_unit: BaseUnit::Ampere}}));
        assert_eq!(results[0].as_ref(), results[1].as_ref());
    }

    #[test]
    fn crlf() {
        let inp: Vec<u8> = to_u8("000019808\r\n000019808\r\n");
        let results: Vec<Result<Output, ParseError>> = Parser::new().parse(&inp);
        assert_eq!(results.len(), 2);
        let out: &Output = results[0].as_ref().unwrap();
        assert_eq!(out.status, Status { temperature_unit: TemperatureUnit::Celsius, sign: SIGN_PLUS, is_battery_depleted: false, is_overflow: false });
        assert_eq!(out.function, Function::ManualAmpere);
        assert_eq!(out.range, Range::Range0);
        assert_eq!(out.option2, Option2 { is_dc: true, is_ac: false, is_auto: false });
        assert_eq!(&out.digits.to_value(DigitRadix::Zero), "1");
        assert_eq!(out.get_value(), Some(OutputValue { digits: "0.001".to_owned(), value_unit: ValueUnit { prefix_unit: PrefixUnit::None, base_unit: BaseUnit::Ampere}}));
        assert_eq!(results[0].as_ref(), results[1].as_ref());
    }

    #[test]
    fn only_lf() {
        let inp: Vec<u8> = to_u8("000019808\n000019808\n");
        let results: Vec<Result<Output, ParseError>> = Parser::new().parse(&inp);
        assert_eq!(results.len(), 2);
        let out: &Output = results[0].as_ref().unwrap();
        assert_eq!(out.status, Status { temperature_unit: TemperatureUnit::Celsius, sign: SIGN_PLUS, is_battery_depleted: false, is_overflow: false });
        assert_eq!(out.function, Function::ManualAmpere);
        assert_eq!(out.range, Range::Range0);
        assert_eq!(out.option2, Option2 { is_dc: true, is_ac: false, is_auto: false });
        assert_eq!(&out.digits.to_value(DigitRadix::Zero), "1");
        assert_eq!(out.get_value(), Some(OutputValue { digits: "0.001".to_owned(), value_unit: ValueUnit { prefix_unit: PrefixUnit::None, base_unit: BaseUnit::Ampere}}));
        assert_eq!(results[0].as_ref(), results[1].as_ref());
    }
}
