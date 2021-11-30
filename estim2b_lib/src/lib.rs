mod device;

use serde::{Deserialize, Serialize};
use std::fmt;

#[cfg(feature = "usb")]
pub use device::usb_two_b::USBTwoB;
#[cfg(feature = "virtual")]
pub use device::virtual_two_b::VirtualTwoB;

use num_enum::{IntoPrimitive, TryFromPrimitive, TryFromPrimitiveError};
use pyo3::prelude::*;
use strum::ParseError as StrumParserError;
use strum_macros::{Display, EnumString, EnumVariantNames};

#[derive(Debug, Display, Serialize, Deserialize)]
pub enum TwoBError {
    ConnectionError(String),
    ParserError(String),
}

impl From<&str> for TwoBError {
    fn from(s: &str) -> TwoBError {
        TwoBError::ParserError(s.into())
    }
}

impl<T: TryFromPrimitive> From<TryFromPrimitiveError<T>> for TwoBError {
    fn from(e: TryFromPrimitiveError<T>) -> TwoBError {
        TwoBError::ParserError(format!("{}", e))
    }
}

impl From<StrumParserError> for TwoBError {
    fn from(s: StrumParserError) -> TwoBError {
        TwoBError::ParserError(s.to_string())
    }
}

#[pyclass]
#[repr(u8)]
#[derive(
    Clone,
    Copy,
    Display,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    TryFromPrimitive,
    IntoPrimitive,
    EnumVariantNames,
    EnumString,
    Serialize,
    Deserialize,
)]
pub enum TwoBMode {
    Pulse,
    Bounce,
    Continuous,
    Flo,
    ASplit,
    BSplit,
    Wave,
    Waterfall,
    Squeeze,
    Milk,
    Throb,
    Thrust,
    Cycle,
    Twist,
    Random,
    Step,
    Training,
}

#[pyclass]
#[repr(u8)]
#[derive(
    Clone,
    Copy,
    Display,
    Debug,
    Eq,
    PartialEq,
    TryFromPrimitive,
    IntoPrimitive,
    EnumVariantNames,
    EnumString,
    Serialize,
    Deserialize,
)]
pub enum TwoBBias {
    A,
    B,
    AVERAGE,
    MAX,
}

#[pyclass]
#[repr(u8)]
#[derive(
    Clone,
    Copy,
    Display,
    Debug,
    Eq,
    PartialEq,
    TryFromPrimitive,
    IntoPrimitive,
    EnumVariantNames,
    EnumString,
    Serialize,
    Deserialize,
)]
pub enum TwoBMap {
    A,
    B,
    C,
}

#[pyclass]
#[repr(u8)]
#[derive(
    Clone,
    Copy,
    Display,
    Debug,
    Eq,
    PartialEq,
    TryFromPrimitive,
    IntoPrimitive,
    EnumVariantNames,
    EnumString,
    Serialize,
    Deserialize,
)]
pub enum TwoBRamp {
    X1,
    X2,
    X3,
    X4,
}

#[pyclass]
#[repr(u8)]
#[derive(
    Clone,
    Copy,
    Display,
    Debug,
    Eq,
    PartialEq,
    TryFromPrimitive,
    IntoPrimitive,
    EnumVariantNames,
    EnumString,
    Serialize,
    Deserialize,
)]
pub enum TwoBWarp {
    X1,
    X2,
    X4,
    X8,
    X16,
    X32,
}

#[pyclass]
#[derive(
    Clone, Copy, Display, Debug, Eq, PartialEq, EnumVariantNames, EnumString, Serialize, Deserialize,
)]
pub enum TwoBPower {
    HIGH,
    LOW,
    DYNAMIC,
}

impl TwoBPower {
    const fn value(self) -> char {
        use TwoBPower::*;
        match self {
            HIGH => 'H',
            LOW => 'L',
            DYNAMIC => 'Y',
        }
    }
}

impl TryFrom<String> for TwoBPower {
    type Error = TwoBError;
    fn try_from(s: String) -> Result<Self, TwoBError> {
        match s {
            x if Self::HIGH.value().to_string() == x => Ok(Self::HIGH),
            y if Self::LOW.value().to_string() == y => Ok(Self::LOW),
            z if *"D" == z => Ok(Self::LOW),
            _ => Err("Cannot parse Power".into()),
        }
    }
}

#[pyclass]
#[derive(
    Clone, Copy, Display, Debug, Eq, PartialEq, EnumVariantNames, EnumString, Serialize, Deserialize,
)]
pub enum TwoBChannel {
    A,
    B,
    C,
    D,
}

impl TwoBChannel {
    const fn value(self) -> char {
        use TwoBChannel::*;
        match self {
            A => 'A',
            B => 'B',
            C => 'C',
            D => 'D',
        }
    }
}

#[pyclass]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TwoBState {
    #[pyo3(get, set)]
    pub mode: TwoBMode,
    #[pyo3(get, set)]
    pub channel_a: u8,
    #[pyo3(get, set)]
    pub channel_b: u8,
    #[pyo3(get, set)]
    pub channel_c: u8,
    #[pyo3(get, set)]
    pub channel_d: u8,
    #[pyo3(get, set)]
    pub power: TwoBPower,
    #[pyo3(get, set)]
    pub bias: TwoBBias,
    #[pyo3(get, set)]
    pub joined_channels: bool,
    #[pyo3(get, set)]
    pub map: TwoBMap,
    #[pyo3(get, set)]
    pub ramp: TwoBRamp,
    #[pyo3(get, set)]
    pub warp: TwoBWarp,
    #[pyo3(get)]
    pub battery: u16,
}

impl TryFrom<&String> for TwoBState {
    type Error = TwoBError;
    fn try_from(s: &String) -> Result<Self, TwoBError> {
        macro_rules! get_value {
            ($x:ident) => {
                $x.next().unwrap().parse().unwrap()
            };
            ($x:ident, $t:ty) => {
                $x.next().unwrap().parse::<$t>().unwrap()
            };
        }

        let mut split = s.split(':');

        Ok(TwoBState {
            battery: get_value!(split),
            channel_a: get_value!(split, u8) / 2,
            channel_b: get_value!(split, u8) / 2,
            channel_c: get_value!(split, u8) / 2,
            channel_d: get_value!(split, u8) / 2,
            mode: TwoBMode::try_from(get_value!(split, u8))?,
            power: TwoBPower::try_from(get_value!(split, String))?,
            bias: TwoBBias::try_from(get_value!(split, u8))?,
            joined_channels: get_value!(split, u8) == 1,
            map: TwoBMap::try_from(get_value!(split, u8))?,
            warp: TwoBWarp::try_from(get_value!(split, u8))?,
            ramp: TwoBRamp::try_from(get_value!(split, u8))?,
        })
    }
}

impl fmt::Display for TwoBState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

pub trait TwoB: Send {
    fn refresh_state(&mut self) -> Result<(), TwoBError>;

    fn reset(&mut self) -> Result<(), TwoBError>;

    fn kill(&mut self) -> Result<(), TwoBError>;

    fn set_joined_channels(&mut self, enable: bool) -> Result<(), TwoBError>;

    fn set_mode(&mut self, mode: TwoBMode) -> Result<(), TwoBError>;

    fn set_power(&mut self, power: TwoBPower) -> Result<(), TwoBError>;

    fn set_map(&mut self, map: TwoBMap) -> Result<(), TwoBError>;

    fn set_bias(&mut self, bias: TwoBBias) -> Result<(), TwoBError>;

    fn set_ramp(&mut self, ramp: TwoBRamp) -> Result<(), TwoBError>;

    fn set_warp(&mut self, warp: TwoBWarp) -> Result<(), TwoBError>;

    fn increment_channel(&mut self, channel: TwoBChannel) -> Result<(), TwoBError>;

    fn decrement_channel(&mut self, channel: TwoBChannel) -> Result<(), TwoBError>;

    fn set_channel(&mut self, channel: TwoBChannel, value: u8) -> Result<(), TwoBError>;

    fn set_state(&mut self, state: TwoBState) -> Result<(), TwoBError>;

    fn get_state(&self) -> TwoBState;

    fn get_mode(&self) -> TwoBMode {
        self.get_state().mode
    }

    fn get_power(&self) -> TwoBPower {
        self.get_state().power
    }

    fn get_bias(&self) -> TwoBBias {
        self.get_state().bias
    }

    fn get_joined_channels(&self) -> bool {
        self.get_state().joined_channels
    }

    fn get_map(&self) -> TwoBMap {
        self.get_state().map
    }

    fn get_ramp(&self) -> TwoBRamp {
        self.get_state().ramp
    }

    fn get_warp(&self) -> TwoBWarp {
        self.get_state().warp
    }

    fn get_battery(&self) -> u16 {
        self.get_state().battery
    }

    fn get_channel(&self, channel: TwoBChannel) -> u8 {
        match channel {
            TwoBChannel::A => self.get_state().channel_a,
            TwoBChannel::B => self.get_state().channel_b,
            TwoBChannel::C => self.get_state().channel_c,
            TwoBChannel::D => self.get_state().channel_d,
        }
    }

    fn get_version(&self) -> String;
}
