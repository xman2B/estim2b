use crate::*;
use pyo3::{prelude::*, PyObjectProtocol};

use std::boxed::Box;
use std::str::FromStr;

#[pymodule]
fn estim2b_lib(py: Python, m: &PyModule) -> PyResult<()> {
    register(py, m)?;
    Ok(())
}

impl From<TwoBError> for PyErr {
    fn from(err: TwoBError) -> PyErr {
        match err {
            TwoBError::ConnectionError(e) => pyo3::exceptions::PyIOError::new_err(e),
            TwoBError::ParserError(e) => pyo3::exceptions::PyUnicodeDecodeError::new_err(e),
        }
    }
}

#[pyclass(name = "TwoB")]
struct PythonWrapper {
    device: Box<dyn TwoB + Send>,
}

#[pyproto]
impl PyObjectProtocol for TwoBState {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }
}

#[pyproto]
impl PyObjectProtocol for TwoBBias {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }
}

#[pyproto]
impl PyObjectProtocol for TwoBMode {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }
}

#[pyproto]
impl PyObjectProtocol for TwoBMap {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }
}

#[pyproto]
impl PyObjectProtocol for TwoBChannel {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }
}

#[pyproto]
impl PyObjectProtocol for TwoBPower {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }
}

#[pyproto]
impl PyObjectProtocol for TwoBRamp {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }
}

#[pyproto]
impl PyObjectProtocol for TwoBWarp {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl PythonWrapper {
    #[new]
    #[args(path = "None")]
    fn new(path: Option<&str>) -> PyResult<Self> {
        if let Some(path) = path {
            if path == "virtual" && cfg!(feature = "virtual") {
                Ok(PythonWrapper {
                    device: Box::new(VirtualTwoB::new()?),
                })
            } else {
                Ok(PythonWrapper {
                    device: Box::new(USBTwoB::try_from(path)?),
                })
            }
        } else {
            Ok(PythonWrapper {
                device: Box::new(USBTwoB::new()?),
            })
        }
    }

    #[pyo3(text_signature = "()")]
    fn refresh_state(&mut self) -> Result<(), TwoBError> {
        self.device.refresh_state()
    }

    #[pyo3(text_signature = "()")]
    fn reset(&mut self) -> Result<(), TwoBError> {
        self.device.reset()
    }

    #[pyo3(text_signature = "()")]
    fn kill(&mut self) -> Result<(), TwoBError> {
        self.device.kill()
    }

    #[pyo3(text_signature = "(enable)")]
    fn set_joined_channels(&mut self, enable: bool) -> Result<(), TwoBError> {
        self.device.set_joined_channels(enable)
    }

    #[pyo3(text_signature = "(mode)")]
    fn set_mode(&mut self, mode: TwoBMode) -> Result<(), TwoBError> {
        self.device.set_mode(mode)
    }

    #[pyo3(text_signature = "(power)")]
    fn set_power(&mut self, power: TwoBPower) -> Result<(), TwoBError> {
        self.device.set_power(power)
    }

    #[pyo3(text_signature = "(map)")]
    fn set_map(&mut self, map: TwoBMap) -> Result<(), TwoBError> {
        self.device.set_map(map)
    }

    #[pyo3(text_signature = "(bias)")]
    fn set_bias(&mut self, bias: TwoBBias) -> Result<(), TwoBError> {
        self.device.set_bias(bias)
    }

    #[pyo3(text_signature = "(ramp)")]
    fn set_ramp(&mut self, ramp: TwoBRamp) -> Result<(), TwoBError> {
        self.device.set_ramp(ramp)
    }

    #[pyo3(text_signature = "(warp)")]
    fn set_warp(&mut self, warp: TwoBWarp) -> Result<(), TwoBError> {
        self.device.set_warp(warp)
    }

    #[pyo3(text_signature = "(channel)")]
    fn increment_channel(&mut self, channel: TwoBChannel) -> Result<(), TwoBError> {
        self.device.increment_channel(channel)
    }

    #[pyo3(text_signature = "(channel)")]
    fn decrement_channel(&mut self, channel: TwoBChannel) -> Result<(), TwoBError> {
        self.device.decrement_channel(channel)
    }

    #[pyo3(text_signature = "(channel, value)")]
    fn set_channel(&mut self, channel: TwoBChannel, value: u8) -> Result<(), TwoBError> {
        self.device.set_channel(channel, value)
    }

    #[pyo3(text_signature = "(state)")]
    fn set_state(&mut self, state: TwoBState) -> Result<(), TwoBError> {
        self.device.set_state(state)
    }

    #[pyo3(text_signature = "()")]
    fn get_state(&self) -> TwoBState {
        self.device.get_state()
    }

    #[pyo3(text_signature = "()")]
    fn get_mode(&self) -> TwoBMode {
        self.device.get_mode()
    }

    #[pyo3(text_signature = "()")]
    fn get_power(&self) -> TwoBPower {
        self.device.get_power()
    }

    #[pyo3(text_signature = "()")]
    fn get_bias(&self) -> TwoBBias {
        self.device.get_bias()
    }

    #[pyo3(text_signature = "()")]
    fn get_joined_channels(&self) -> bool {
        self.device.get_joined_channels()
    }

    #[pyo3(text_signature = "()")]
    fn get_map(&self) -> TwoBMap {
        self.device.get_map()
    }

    #[pyo3(text_signature = "()")]
    fn get_ramp(&self) -> TwoBRamp {
        self.device.get_ramp()
    }

    #[pyo3(text_signature = "()")]
    fn get_warp(&self) -> TwoBWarp {
        self.device.get_warp()
    }

    #[pyo3(text_signature = "()")]
    fn get_battery(&self) -> u16 {
        self.device.get_battery()
    }

    #[pyo3(text_signature = "(channel)")]
    fn get_channel(&self, channel: &str) -> Result<u8, TwoBError> {
        Ok(self.device.get_channel(TwoBChannel::from_str(channel)?))
    }

    #[pyo3(text_signature = "()")]
    fn get_version(&self) -> String {
        self.device.get_version()
    }
}

// register methods for exporting with pyo3
fn register(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PythonWrapper>()?;
    m.add_class::<TwoBBias>()?;
    m.add_class::<TwoBChannel>()?;
    m.add_class::<TwoBMap>()?;
    m.add_class::<TwoBMode>()?;
    m.add_class::<TwoBPower>()?;
    m.add_class::<TwoBRamp>()?;
    m.add_class::<TwoBState>()?;
    Ok(())
}
