use estim2b_lib::*;
use rocket::{get, post, launch, routes, serde::json::Json, State};
use std::str::FromStr;
use std::sync::{Arc, Mutex};

#[get("/refresh_state")]
fn refresh_state(two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>) -> Json<Result<(), TwoBError>> {
    two_b.lock().unwrap().refresh_state().into()
}

#[get("/reset")]
fn reset(two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>) -> Json<Result<(), TwoBError>> {
    two_b.lock().unwrap().reset().into()
}

#[get("/kill")]
fn kill(two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>) -> Json<Result<(), TwoBError>> {
    two_b.lock().unwrap().kill().into()
}

#[get("/set_joined_channels?<enable>")]
fn set_joined_channels(
    two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>,
    enable: &str,
) -> Json<Result<(), TwoBError>> {
    if let Ok(enable) = bool::from_str(enable) {
        two_b.lock().unwrap().set_joined_channels(enable).into()
    } else {
        Json(Err(TwoBError::ParserError(
            "'enable' has to be true or false!".into(),
        )))
    }
}

#[get("/set_mode?<mode>")]
fn set_mode(two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>, mode: &str) -> Json<Result<(), TwoBError>> {
    if let Ok(mode) = TwoBMode::from_str(mode) {
        two_b.lock().unwrap().set_mode(mode).into()
    } else {
        Json(Err(TwoBError::ParserError("Invalid mode!".into())))
    }
}

#[get("/set_power?<power>")]
fn set_power(two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>, power: &str) -> Json<Result<(), TwoBError>> {
    if let Ok(power) = TwoBPower::from_str(power) {
        two_b.lock().unwrap().set_power(power).into()
    } else {
        Json(Err(TwoBError::ParserError("Invalid power!".into())))
    }
}

#[get("/set_map?<map>")]
fn set_map(two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>, map: &str) -> Json<Result<(), TwoBError>> {
    if let Ok(map) = TwoBMap::from_str(map) {
        two_b.lock().unwrap().set_map(map).into()
    } else {
        Json(Err(TwoBError::ParserError("Invalid map!".into())))
    }
}

#[get("/set_bias?<bias>")]
fn set_bias(two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>, bias: &str) -> Json<Result<(), TwoBError>> {
    if let Ok(bias) = TwoBBias::from_str(bias) {
        two_b.lock().unwrap().set_bias(bias).into()
    } else {
        Json(Err(TwoBError::ParserError("Invalid bias!".into())))
    }
}

#[get("/set_ramp?<ramp>")]
fn set_ramp(two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>, ramp: &str) -> Json<Result<(), TwoBError>> {
    if let Ok(ramp) = TwoBRamp::from_str(ramp) {
        two_b.lock().unwrap().set_ramp(ramp).into()
    } else {
        Json(Err(TwoBError::ParserError("Invalid ramp!".into())))
    }
}

#[get("/set_warp?<warp>")]
fn set_warp(two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>, warp: &str) -> Json<Result<(), TwoBError>> {
    if let Ok(warp) = TwoBWarp::from_str(warp) {
        two_b.lock().unwrap().set_warp(warp).into()
    } else {
        Json(Err(TwoBError::ParserError("Invalid warp!".into())))
    }
}

#[get("/increment_channel?<id>")]
fn increment_channel(
    two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>,
    id: &str,
) -> Json<Result<(), TwoBError>> {
    if let Ok(channel) = TwoBChannel::from_str(id) {
        two_b.lock().unwrap().increment_channel(channel).into()
    } else {
        Json(Err(TwoBError::ParserError("Invalid Channel ID!".into())))
    }
}

#[get("/decrement_channel?<id>")]
fn decrement_channel(
    two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>,
    id: &str,
) -> Json<Result<(), TwoBError>> {
    if let Ok(channel) = TwoBChannel::from_str(id) {
        two_b.lock().unwrap().decrement_channel(channel).into()
    } else {
        Json(Err(TwoBError::ParserError("Invalid Channel ID!".into())))
    }
}

#[get("/set_channel?<id>&<value>")]
fn set_channel(
    two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>,
    id: &str,
    value: u8,
) -> Json<Result<(), TwoBError>> {
    if let Ok(channel) = TwoBChannel::from_str(id) {
        two_b.lock().unwrap().set_channel(channel, value).into()
    } else {
        Json(Err(TwoBError::ParserError("Invalid Channel ID!".into())))
    }
}

#[post("/set_state", data = "<state>")]
fn set_state(
    two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>,
    state: Json<TwoBState>,
) -> Json<Result<(), TwoBError>> {
    two_b.lock().unwrap().set_state(state.into_inner()).into()
}

#[get("/")]
fn get_state(two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>) -> Json<TwoBState> {
    two_b.lock().unwrap().get_state().into()
}

#[get("/get_mode")]
fn get_mode(two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>) -> Json<TwoBMode> {
    two_b.lock().unwrap().get_mode().into()
}

#[get("/get_power")]
fn get_power(two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>) -> Json<TwoBPower> {
    two_b.lock().unwrap().get_power().into()
}

#[get("/get_bias")]
fn get_bias(two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>) -> Json<TwoBBias> {
    two_b.lock().unwrap().get_bias().into()
}

#[get("/get_joined_channels")]
fn get_joined_channels(two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>) -> Json<bool> {
    two_b.lock().unwrap().get_joined_channels().into()
}

#[get("/get_map")]
fn get_map(two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>) -> Json<TwoBMap> {
    two_b.lock().unwrap().get_map().into()
}

#[get("/get_ramp")]
fn get_ramp(two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>) -> Json<TwoBRamp> {
    two_b.lock().unwrap().get_ramp().into()
}

#[get("/get_warp")]
fn get_warp(two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>) -> Json<TwoBWarp> {
    two_b.lock().unwrap().get_warp().into()
}

#[get("/get_battery")]
fn get_battery(two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>) -> Json<u16> {
    two_b.lock().unwrap().get_battery().into()
}

#[get("/get_channel?<id>")]
fn get_channel(two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>, id: &str) -> Json<Result<u8, TwoBError>> {
    if let Ok(channel) = TwoBChannel::from_str(id) {
        Ok(two_b.lock().unwrap().get_channel(channel)).into()
    } else {
        Json(Err(TwoBError::ParserError("Invalid Channel ID!".into())))
    }
}

#[get("/get_version")]
fn get_version(two_b: &State<Arc<Mutex<Box<dyn TwoB>>>>) -> Json<String> {
    two_b.lock().unwrap().get_version().into()
}

#[launch]
fn rocket() -> _ {
    let args: Vec<String> = std::env::args().collect();
    let two_b: Box<dyn TwoB>;
    if args.len() >= 2 {
        let path = args[1].trim();
        if path == "virtual" {
            two_b = Box::new(VirtualTwoB::new().unwrap());
        } else {
            two_b = Box::<USBTwoB>::new(
                USBTwoB::try_from(path).expect("No 2B found on path the given path"),
            );
        }
    } else {
        two_b = Box::new(USBTwoB::new().expect("No 2B found"));
    }
    rocket::build()
        .manage(Arc::new(Mutex::new(two_b)))
        .mount(
            "/api",
            routes![
                refresh_state,
                reset,
                kill,
                set_joined_channels,
                set_mode,
                set_power,
                set_map,
                set_bias,
                set_ramp,
                set_warp,
                increment_channel,
                decrement_channel,
                set_channel,
                set_state,
                get_state,
                get_mode,
                get_power,
                get_bias,
                get_joined_channels,
                get_map,
                get_ramp,
                get_warp,
                get_battery,
                get_channel,
                get_version
            ],
        )
        .mount("/api/get_state", routes![get_state])
}
