use std::fmt;

#[cfg(feature = "cli")]
use reqwest::{Client, Url};

use crate::{
    weather_data::WeatherData, Error, ApiStringType, StringType,
};

#[derive(Clone)]
pub enum WeatherLocation {
    CityName(StringType),
}

impl WeatherLocation {
    #[must_use]
    pub fn from_city_name(city_name: &str) -> Self {
        Self::CityName(city_name.into())
    }

    #[must_use]
    pub fn get_cityname(&self) -> Vec<(&'static str, ApiStringType)> {
        match self {
            Self::CityName(city_name) => {
                let city_name = city_name.into();
                vec![("q", city_name)]
            }
        }
    }
}

/// `WeatherApi` contains a `reqwest` Client and all the metadata required to
/// query the openweathermap.org api.
#[cfg(feature = "cli")]
#[derive(Clone)]
pub struct WeatherApi {
    client: Client,
    api_key: ApiStringType,
    api_endpoint: StringType,
    api_path: StringType,
}

#[derive(Clone, Copy)]
enum WeatherCommands {
    WeatherInfo,
}

impl WeatherCommands {
    fn to_str(self) -> &'static str {
        match self {
            Self::WeatherInfo => "weather",
        }
    }
}

impl fmt::Display for WeatherCommands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

#[cfg(feature = "cli")]
impl WeatherApi {
    /// Create `WeatherApi` instance specifying `api_key`, `api_endpoint` and
    /// `api_path`
    #[must_use]
    pub fn new(api_key: &str, api_endpoint: &str, api_path: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            api_endpoint: api_endpoint.into(),
            api_path: api_path.into(),
        }
    }

    /// Get `WeatherData` from api
    /// # Errors
    ///
    /// Will return error if `WeatherApi::run_api` fails
    pub async fn get_weather_data(&self, location: &WeatherLocation) -> Result<WeatherData, Error> {
        let options = self.get_options(location);
        self.run_api(WeatherCommands::WeatherInfo, &options).await
    }

    fn get_options(&self, location: &WeatherLocation) -> Vec<(&'static str, ApiStringType)> {
        let mut options = location.get_cityname();
        options.push(("APPID", self.api_key.clone()));
        options
    }

    /// # Errors
    /// Will return error if :
    ///     * `base_url` is invalid
    ///     * request fails
    ///     * deserializing json response fails
    async fn run_api<T: serde::de::DeserializeOwned>(
        &self,
        command: WeatherCommands,
        options: &[(&'static str, ApiStringType)],
    ) -> Result<T, Error> {
        let api_endpoint = &self.api_endpoint;
        let api_path = &self.api_path;
        let base_url = format!("https://{api_endpoint}/{api_path}{command}");
        let url = Url::parse_with_params(&base_url, options)?;

        self.client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .map_err(Into::into)
    }
}
