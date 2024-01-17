use pros_sys::PROS_ERR;

use super::{AdiError, AdiPort};
use crate::error::bail_on;

#[derive(Debug, Eq, PartialEq)]
pub struct AdiGyro {
    raw: pros_sys::ext_adi_gyro_t,
}

impl AdiGyro {
    /// Create an AdiGyro, returning err `AdiError::InvalidPort` if the port is invalid.
    pub fn new(port: AdiPort, multiplier: f64) -> Result<Self, AdiError> {
        Ok(Self {
            raw: unsafe {
                bail_on!(
                    PROS_ERR.into(),
                    pros_sys::ext_adi_gyro_init(
                        port.internal_expander_index(),
                        port.index(),
                        multiplier
                    )
                )
            },
        })
    }

    /// Gets the current gyro angle in tenths of a degree. Unless a multiplier is applied to the gyro, the return value will be a whole number representing the number of degrees of rotation times 10.
    ///
    /// There are 360 degrees in a circle, thus the gyro will return 3600 for one whole rotation.
    pub fn value(&self) -> Result<f64, AdiError> {
        Ok(unsafe { bail_on!(PROS_ERR.into(), pros_sys::ext_adi_gyro_get(self.raw)) })
    }

    /// Gets the current gyro angle in tenths of a degree. Unless a multiplier is applied to the gyro, the return value will be a whole number representing the number of degrees of rotation times 10.
    pub fn zero(&mut self) -> Result<i32, AdiError> {
        Ok(unsafe { bail_on!(PROS_ERR.into(), pros_sys::ext_adi_gyro_reset(self.raw)) })
    }
}
