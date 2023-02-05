use serde::Deserialize;
use std::{
    ops::Deref, // Used for immutable dereferencing operations
    path::Path, // A slice of a path
    sync::Arc // A thread-safe reference-counting pointer
};

use crate::{
    ApiStringType,
    Error,
    StringType
};

/// Configuration data
#[derive(Default, Deserialize)]
pub struct ConfigBase {
    /// api key
    pub api_key: Option<ApiStringType>,

    /// api endpoint
    #[serde(default = "default_api_endpoint")]
    pub api_endpoint: StringType,

    /// api path (default is `data/2.5`) based on current openweathermap endpoint
    #[serde(default = "default_api_path")]
    pub api_path: StringType,

    /// optional default country code
    pub country_code: Option<StringType>,

    /// optional default city name
    pub city_name: Option<StringType>,
}

fn default_api_endpoint() -> StringType {
    "api.openweathermap.org".into()
}

fn default_api_path() -> StringType {
    "data/2.5/".into()
}

/// Configuration struct
#[derive(Default)]
pub struct ConfigApp(Arc<ConfigBase>);

impl ConfigApp {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // TODO: PUT DOCTEST HERE
    pub fn initialize_app(config_path: Option<&Path>) -> Result<Self, Error> {
        let filename = config_path.unwrap_or_else(|| Path::new("config.env"));
        let config_dir = dirs::config_dir().unwrap_or_else(|| "./".into());
        let default_filename = config_dir.join("weatherinfo_util").join("config.env");

        let env_file = if filename.exists() {
            filename
        } else {
            &default_filename
        };

        dotenvy::dotenv().ok();

        if env_file.exists() {
            dotenvy::from_path(env_file).ok();
        }

        let conf: ConfigBase = envy::from_env()?;

        Ok(Self(Arc::new(conf)))
    }
}

impl Deref for ConfigApp {
    type Target = ConfigBase;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}