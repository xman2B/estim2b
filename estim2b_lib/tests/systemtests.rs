use estim2b_lib::*;

#[cfg(feature = "usb")]
#[test]
fn parse_response() -> Result<(), TwoBError> {
    let s = "344:10:12:120:116:15:L:0:0:0:0:0:2.120B".to_string();
    assert_eq!(
        TwoBState::try_from(&s)?,
        TwoBState {
            battery: 344,
            channel_a: 5,
            channel_b: 6,
            channel_c: 60,
            channel_d: 58,
            mode: TwoBMode::try_from(15u8)?,
            power: TwoBPower::LOW,
            bias: TwoBBias::A,
            joined_channels: false,
            map: TwoBMap::A,
            ramp: TwoBRamp::X1,
            warp: TwoBWarp::X1,
        }
    );
    Ok(())
}

#[cfg(feature = "usb")]
#[test]
#[serial_test::serial]
fn usb_cycle_modes() -> Result<(), TwoBError> {
    cycle_modes(USBTwoB::new()?)
}

#[cfg(feature = "virtual")]
#[test]
fn virtual_cycle_modes() -> Result<(), TwoBError> {
    cycle_modes(VirtualTwoB::new()?)
}

fn cycle_modes(mut twob: impl TwoB) -> Result<(), TwoBError> {
    let start: u8 = TwoBMode::Pulse.into();
    let end: u8 = TwoBMode::Training.into();
    for mode in start..=end {
        println!("{}", mode);
        twob.set_mode(TwoBMode::try_from(mode)?)?;
    }

    Ok(())
}

#[cfg(feature = "usb")]
#[test]
#[serial_test::serial]
fn usb_state_changing() -> Result<(), TwoBError> {
    state_changing(USBTwoB::new()?)
}

#[cfg(feature = "virtual")]
#[test]
fn virtual_state_changing() -> Result<(), TwoBError> {
    state_changing(VirtualTwoB::new()?)
}

fn state_changing(mut twob: impl TwoB) -> Result<(), TwoBError> {
    twob.reset()?;
    let mut state = twob.get_state();

    assert_eq!(
        state,
        TwoBState {
            channel_a: 0,
            channel_b: 0,
            channel_c: 50,
            channel_d: 50,
            mode: TwoBMode::Pulse,
            joined_channels: false,
            ..state
        }
    );

    twob.set_state(TwoBState {
        channel_a: 1,
        channel_b: 2,
        channel_c: 4,
        channel_d: 8,
        mode: TwoBMode::Throb,
        joined_channels: true,
        power: TwoBPower::HIGH,
        bias: TwoBBias::MAX,
        map: TwoBMap::C,
        ramp: TwoBRamp::X4,
        warp: TwoBWarp::X32,
        ..state
    })?;

    state = twob.get_state();
    assert_eq!(
        state,
        TwoBState {
            channel_a: 1,
            channel_b: 2,
            channel_c: 4,
            channel_d: 8,
            mode: TwoBMode::Throb,
            joined_channels: true,
            power: TwoBPower::HIGH,
            bias: TwoBBias::MAX,
            map: TwoBMap::C,
            ramp: TwoBRamp::X4,
            warp: TwoBWarp::X32,
            ..state
        }
    );

    twob.set_state(TwoBState {
        channel_a: 16,
        channel_b: 32,
        channel_c: 64,
        channel_d: 96,
        mode: TwoBMode::Waterfall,
        joined_channels: true,
        power: TwoBPower::HIGH,
        bias: TwoBBias::AVERAGE,
        map: TwoBMap::B,
        ramp: TwoBRamp::X2,
        warp: TwoBWarp::X16,
        ..state
    })?;

    state = twob.get_state();
    assert_eq!(
        state,
        TwoBState {
            channel_a: 16,
            channel_b: 32,
            channel_c: 64,
            channel_d: 96,
            mode: TwoBMode::Waterfall,
            joined_channels: true,
            power: TwoBPower::HIGH,
            bias: TwoBBias::AVERAGE,
            map: TwoBMap::B,
            ramp: TwoBRamp::X2,
            warp: TwoBWarp::X16,
            ..state
        }
    );

    twob.reset()?;
    state = twob.get_state();
    assert_eq!(
        state,
        TwoBState {
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
            ..state
        }
    );
    Ok(())
}
