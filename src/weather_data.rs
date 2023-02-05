use serde::{Deserialize, Serialize};
use std::fmt::Write;

use crate::{
    temperature::Temperature, StringType
};

#[derive(Serialize, Deserialize)]
pub struct WeatherCond {
    pub main: StringType,
    pub description: StringType,
}

#[derive(Serialize, Deserialize, Default)]
pub struct WeatherMain {
    pub temp: Temperature,
}

#[derive(Deserialize, Serialize)]
pub struct Sys {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<StringType>,
}

impl Default for Sys {
    fn default() -> Self {
        Self {
            country: None,
        }
    }
}

#[derive(Deserialize, Serialize)]
/// Set fields name based on Openweathermap output fields
pub struct WeatherData {
    pub weather: Vec<WeatherCond>,
    pub main: WeatherMain,
    pub sys: Sys,
    pub name: StringType,
}

impl Default for WeatherData {
    fn default() -> Self {
        Self {
            weather: Vec::new(),
            main: WeatherMain::default(),
            sys: Sys::default(),
            name: "".into(),
        }
    }
}

impl WeatherData {
    /// TODO: PUT DOCTEST HERE
    #[must_use]
    pub fn get_current_conditions(&self) -> StringType {
        let mut output: StringType = "Your location: ".into();

        if let Some(country) = &self.sys.country {
            let name = &self.name;
            write!(output, "{name}, {country} ").unwrap_or(());
        };

        writeln!(
            output,
            "\nTemperature: {c:0.2}°C ({f:0.2}°F)",
            f = self.main.temp.fahrenheit(),
            c = self.main.temp.celcius(),
        )
        .unwrap_or(());

        writeln!(
            output,
            "Conditions: {}",
            self.weather.get(0).map_or_else(|| "", |w| &w.description)
        )
        .unwrap_or(());

        output.push('\n');
        output
    }
}
