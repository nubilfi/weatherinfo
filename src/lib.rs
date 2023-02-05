/// Configuration data
pub mod config;
/// Temperature module: conversions between Kelvin, Ceclius and Fahrenheit
pub mod temperature;
/// Reqwest Client
pub mod api;
/// Representation of Weather Data from openweathermap.org
pub mod weather_data;
/// CLI Options
pub mod cli_opts;
/// Application Error
pub mod error;

/// re-export error module
pub use error::Error;

pub type StringType = String;
pub type ApiStringType = String;

#[macro_export]
macro_rules! format_string {
    ($($arg:tt)*) => {
        {
            use std::fmt::Write;
            let mut buf = String::new();
            std::write!(buf, "{}", std::format_args!($($arg)*)).unwrap();
            buf
        }
    };
}
