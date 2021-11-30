use crate::*;

pub struct VirtualTwoB {
    state: TwoBState,
    version: String,
}

impl VirtualTwoB {
    pub fn new() -> Result<Self, TwoBError> {
        let version = String::from("2.122B");
        let state = TwoBState {
            channel_a: 0,
            channel_b: 0,
            channel_c: 50,
            channel_d: 50,
            mode: TwoBMode::Pulse,
            joined_channels: false,
            power: TwoBPower::LOW,
            bias: TwoBBias::A,
            map: TwoBMap::A,
            ramp: TwoBRamp::X1,
            warp: TwoBWarp::X1,
            battery: 1000,
        };
        Ok(VirtualTwoB {
            state,
            version,
        })
    }
}

impl TwoB for VirtualTwoB {
    fn refresh_state(&mut self) -> Result<(), TwoBError> {
        Ok(())
    }

    fn reset(&mut self) -> Result<(), TwoBError> {
        self.state = TwoBState {
            channel_a: 0,
            channel_b: 0,
            channel_c: 50,
            channel_d: 50,
            mode: TwoBMode::Pulse,
            joined_channels: false,
            power: TwoBPower::LOW,
            bias: TwoBBias::A,
            map: TwoBMap::A,
            ramp: TwoBRamp::X1,
            warp: TwoBWarp::X1,
            ..self.state
        };
        Ok(())
    }

    fn kill(&mut self) -> Result<(), TwoBError> {
        self.state.channel_a = 0;
        self.state.channel_b = 0;
        Ok(())
    }

    fn set_joined_channels(&mut self, enable: bool) -> Result<(), TwoBError> {
        self.state.joined_channels = enable;
        Ok(())
    }

    fn set_mode(&mut self, mode: TwoBMode) -> Result<(), TwoBError> {
        self.state.mode = mode;
        Ok(())
    }

    fn set_power(&mut self, power: TwoBPower) -> Result<(), TwoBError> {
        self.state.power = power;
        Ok(())
    }

    fn set_map(&mut self, map: TwoBMap) -> Result<(), TwoBError> {
        self.state.map = map;
        Ok(())
    }

    fn set_bias(&mut self, bias: TwoBBias) -> Result<(), TwoBError> {
        self.state.bias = bias;
        Ok(())
    }

    fn set_ramp(&mut self, ramp: TwoBRamp) -> Result<(), TwoBError> {
        self.state.ramp = ramp;
        Ok(())
    }

    fn set_warp(&mut self, warp: TwoBWarp) -> Result<(), TwoBError> {
        self.state.warp = warp;
        Ok(())
    }

    fn increment_channel(&mut self, channel: TwoBChannel) -> Result<(), TwoBError> {
        match channel {
            TwoBChannel::A => self.state.channel_a += 1,
            TwoBChannel::B => self.state.channel_b += 1,
            TwoBChannel::C => self.state.channel_c += 1,
            TwoBChannel::D => self.state.channel_d += 1,
        }
        Ok(())
    }

    fn decrement_channel(&mut self, channel: TwoBChannel) -> Result<(), TwoBError> {
        match channel {
            TwoBChannel::A => self.state.channel_a -= 1,
            TwoBChannel::B => self.state.channel_b -= 1,
            TwoBChannel::C => self.state.channel_c -= 1,
            TwoBChannel::D => self.state.channel_d -= 1,
        }
        Ok(())
    }

    fn set_channel(&mut self, channel: TwoBChannel, value: u8) -> Result<(), TwoBError> {
        match channel {
            TwoBChannel::A => self.state.channel_a = value,
            TwoBChannel::B => self.state.channel_b = value,
            TwoBChannel::C => self.state.channel_c = value,
            TwoBChannel::D => self.state.channel_d = value,
        }
        Ok(())
    }

    fn set_state(&mut self, state: TwoBState) -> Result<(), TwoBError> {
        self.state = state;
        Ok(())
    }

    fn get_state(&self) -> TwoBState {
        self.state.clone()
    }

    fn get_version(&self) -> String {
        self.version.clone()
    }
}
