use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};
use std::io::BufRead;
use std::io::BufReader;
use std::time::Duration;

use crate::*;

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyUSB0";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM3";

const TIMEOUT: u64 = 100;

impl From<serialport::Error> for TwoBError {
    fn from(sp_error: serialport::Error) -> TwoBError {
        TwoBError::ConnectionError(sp_error.to_string())
    }
}

impl From<std::io::Error> for TwoBError {
    fn from(io_error: std::io::Error) -> TwoBError {
        TwoBError::ConnectionError(io_error.to_string())
    }
}

pub struct USBTwoB {
    state: TwoBState,
    version: String,
    io: Box<dyn SerialPort>,
}

impl TryFrom<&str> for USBTwoB {
    type Error = TwoBError;
    fn try_from(tty: &str) -> Result<Self, TwoBError> {
        let mut io = serialport::new(tty, 9600)
            .data_bits(DataBits::Eight)
            .stop_bits(StopBits::One)
            .parity(Parity::None)
            .timeout(Duration::from_millis(TIMEOUT))
            .flow_control(FlowControl::None)
            .open()?;
        //sleep(Duration::from_millis(TIMEOUT));
        let answer = Self::_send(&mut io, String::from("V"))?;
        let (state, version) = Self::parse(answer)?;
        Ok(USBTwoB {
            io,
            state,
            version,
        })
    }
}

impl USBTwoB {
    pub fn new() -> Result<Self, TwoBError> {
        USBTwoB::try_from(DEFAULT_TTY)
    }

    fn _send(io: &mut Box<dyn SerialPort>, msg: String) -> Result<String, TwoBError> {
        let full_msg = msg + "\r";
        assert!(io.write(full_msg.as_bytes())? != 0);
        io.flush()?;
        let mut answer = String::new();
        let mut reader = BufReader::new(io);
        reader.read_line(&mut answer)?;
        Ok(answer)
    }

    fn parse(answer: String) -> Result<(TwoBState, String), TwoBError> {
        let state = TwoBState::try_from(&answer)?;
        let version = match answer.trim().split(':').last() {
            Some(x) => x,
            None => return Err("Couldn't parse version string".into()),
        };
        Ok((state, version.into()))
    }

    fn send(&mut self, msg: String) -> Result<(), TwoBError> {
        let answer = Self::_send(&mut self.io, msg)?;
        let (state, version) = Self::parse(answer)?;
        self.state = state;
        self.version = version;
        Ok(())
    }
}

impl TwoB for USBTwoB {
    fn refresh_state(&mut self) -> Result<(), TwoBError> {
        self.send("V".to_string())
    }

    fn reset(&mut self) -> Result<(), TwoBError> {
        self.send("E".to_string())
    }

    fn kill(&mut self) -> Result<(), TwoBError> {
        self.send("K".to_string())
    }

    fn set_joined_channels(&mut self, enable: bool) -> Result<(), TwoBError> {
        self.send(format!("J{}", if enable { 1 } else { 0 }))
    }

    fn set_mode(&mut self, mode: TwoBMode) -> Result<(), TwoBError> {
        let number: u8 = mode.into();
        return self.send(format!("M{}", number));
    }

    fn set_power(&mut self, power: TwoBPower) -> Result<(), TwoBError> {
        self.send(power.value().to_string())
    }

    fn set_map(&mut self, map: TwoBMap) -> Result<(), TwoBError> {
        let number: u8 = map.into();
        return self.send(format!("O{}", number));
    }

    fn set_bias(&mut self, bias: TwoBBias) -> Result<(), TwoBError> {
        let number: u8 = bias.into();
        return self.send(format!("Q{}", number));
    }

    fn set_ramp(&mut self, ramp: TwoBRamp) -> Result<(), TwoBError> {
        let number: u8 = ramp.into();
        return self.send(format!("R{}", number));
    }

    fn set_warp(&mut self, warp: TwoBWarp) -> Result<(), TwoBError> {
        let number: u8 = warp.into();
        return self.send(format!("W{}", number));
    }

    fn increment_channel(&mut self, channel: TwoBChannel) -> Result<(), TwoBError> {
        self.send(format!("{}+", channel.value()))
    }

    fn decrement_channel(&mut self, channel: TwoBChannel) -> Result<(), TwoBError> {
        self.send(format!("{}-", channel.value()))
    }

    fn set_channel(&mut self, channel: TwoBChannel, value: u8) -> Result<(), TwoBError> {
        self.send(format!("{}{}", channel.value(), value))
    }

    fn set_state(&mut self, state: TwoBState) -> Result<(), TwoBError> {
        macro_rules! compare {
            ($field:ident) => {
                self.state.$field != state.$field
            };
            ($field:ident, $func:ident) => {
                if compare!($field) {
                    self.$func(state.$field)?;
                }
            };
            ($field:ident, $channel: path) => {
                if compare!($field) {
                    self.set_channel($channel, state.$field)?;
                }
            };
        }

        compare!(mode, set_mode);
        compare!(power, set_power);
        compare!(bias, set_bias);
        compare!(joined_channels, set_joined_channels);
        compare!(map, set_map);
        compare!(ramp, set_ramp);
        compare!(warp, set_warp);

        compare!(channel_a, TwoBChannel::A);
        compare!(channel_b, TwoBChannel::B);
        compare!(channel_c, TwoBChannel::C);
        compare!(channel_d, TwoBChannel::D);

        Ok(())
    }

    fn get_state(&self) -> TwoBState {
        self.state.clone()
    }

    fn get_channel(&self, channel: TwoBChannel) -> u8 {
        match channel {
            TwoBChannel::A => self.state.channel_a,
            TwoBChannel::B => self.state.channel_b,
            TwoBChannel::C => self.state.channel_c,
            TwoBChannel::D => self.state.channel_d,
        }
    }

    fn get_version(&self) -> String {
        self.version.clone()
    }
}
