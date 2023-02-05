use serde::{Serialize, Deserialize};

#[cfg(feature = "cli")]
use clap::{CommandFactory, Parser};

#[cfg(feature = "cli")]
use tokio::io::{stdout, AsyncWriteExt};

use crate::{
    format_string,
    config::ConfigApp,
    api::WeatherLocation,
    Error,
    ApiStringType,
    StringType
};

#[cfg(feature = "cli")]
use crate::api::WeatherApi;

#[cfg(feature = "cli")]
#[derive(Parser, Serialize, Deserialize)]
pub struct CliOpts {
    /// Country Code (optional), if not specified `id (indonesia)` will be assumed
    #[clap(short, long)]
    country_code: Option<StringType>,
    /// City Name (optional)
    #[clap(long)]
    city_name: Option<StringType>,
    /// Api key (optional but either this or API_KEY environment variable must exist)
    #[clap(short = 'k', long)]
    api_key: Option<ApiStringType>,
}

#[cfg(feature = "cli")]
impl CliOpts {
    /// Parse options from stdin
    /// # Errors
    /// Returns error if call to retreive weather data fails or if write to
    /// stdout fails
    pub async fn parse_opts(config: &ConfigApp) -> Result<(), Error> {
        let mut opts = Self::parse();
        opts.apply_defaults(config);

        let mut stdout = stdout();
        for output in opts.run_opts(config).await? {
            stdout.write_all(output.as_bytes()).await?;
        }
        Ok(())
    }

    /// # Errors
    /// Return Error if api key invalid or cannot be found
    #[cfg(feature = "cli")]
    fn get_api(&self, config: &ConfigApp) -> Result<WeatherApi, Error> {
        let api_key = self
            .api_key
            .as_deref()
            .ok_or_else(|| Error::InvalidInputError(format_string!("invalid api key")))?;

        Ok(WeatherApi::new(
            api_key,
            &config.api_endpoint,
            &config.api_path,
        ))
    }

    /// Extract options from `CliOpts` and apply to `WeatherApi`
    /// # Errors
    /// Returns Error if clap help output fails
    pub fn get_location(&self) -> Result<WeatherLocation, Error> {
        let loc =  if let Some(city_name) = &self.city_name {
            WeatherLocation::from_city_name(city_name)
        } else {
            return Err(Error::InvalidInputError(format_string!(
                "\nERROR: You must specify at least one option between --country-code or --city-name\n"
            )));
        };
        Ok(loc)
    }

    /// # Errors
    /// Returns error if call to retreive weather data fails
    async fn run_opts(&self, config: &ConfigApp) -> Result<Vec<StringType>, Error> {
        let api = self.get_api(config)?;
        let loc = self.get_location()?;

        let data = api.get_weather_data(&loc).await?;

        let output = vec![data.get_current_conditions()];
        Ok(output)
    }

    fn apply_defaults(&mut self, config: &ConfigApp) {
        if self.api_key.is_none() {
            self.api_key = config.api_key.clone();
        }
    }

    #[must_use]
    pub fn api_help_msg() -> StringType {
        format_string!("{}", Self::command().render_help())
    }
}
