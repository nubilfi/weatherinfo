use derive_more::Into;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use crate::{format_string, Error};

const FREEZING_POINT_KELVIN: f64 = 273.15;
const FAHRENHEIT_OFFSET: f64 = 459.67;
const FAHRENHEIT_FACTOR: f64 = 1.8;

/// Temperature struct, data is stored as Kelvin
#[derive(Into, Debug, PartialEq, Copy, Clone, PartialOrd, Serialize, Deserialize, Default)]
#[serde(into = "f64", try_from = "f64")]
pub struct Temperature(f64);

impl TryFrom<f64> for Temperature {
    type Error = Error;

    fn try_from(item: f64) -> Result<Self, Self::Error> {
        if item >= 0.0 {
            Ok(Self(item))
        } else {
            Err(Error::InvalidValue(format_string!(
                "{item} is not a valid Temperature"
            )))
        }
    }
}

impl Temperature {
    /// # Errors
    /// Will return error if input is less than zero
    pub fn from_celcius(t: f64) -> Result<Self, Error> {
        if t >= -FREEZING_POINT_KELVIN {
            Ok(Self(t + FREEZING_POINT_KELVIN))
        } else {
            Err(Error::InvalidValue(format_string!(
                "{t} is not a valid temperature in Celcius"
            )))
        }
    }

    /// # Errors
    /// Will return error if input is less than zero
    pub fn from_fahrenheit(t: f64) -> Result<Self, Error> {
        if t >= -FAHRENHEIT_OFFSET {
            Ok(Self((t + FAHRENHEIT_OFFSET) / FAHRENHEIT_FACTOR))
        } else {
            Err(Error::InvalidValue(format_string!(
                "{t} is not a valid temperature in Fahrenheit",
            )))
        }
    }

    #[inline]
    #[must_use]
    pub fn celcius(self) -> f64 {
        self.0 - FREEZING_POINT_KELVIN
    }

    #[inline]
    #[must_use]
    pub fn fahrenheit(self) -> f64 {
        self.0 * FAHRENHEIT_FACTOR - FAHRENHEIT_OFFSET
    }
}
