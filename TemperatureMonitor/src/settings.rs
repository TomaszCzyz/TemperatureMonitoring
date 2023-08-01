use std::error::Error;
use std::time::Duration;

use config::{Config, ConfigBuilder, File, FileFormat};
use config::builder::DefaultState;

pub struct Settings {
    filename: String,
    config: Config,
}

impl Settings {
    pub fn new(filename: String) -> Self {
        let config = ConfigBuilder::<DefaultState>::default()
            .add_source(File::new(&filename, FileFormat::Toml))
            .build()
            .unwrap();

        Self {
            filename,
            config,
        }
    }

    pub fn refresh(&mut self) {
        self.config = ConfigBuilder::<DefaultState>::default()
            .add_source(File::new(&self.filename, FileFormat::Toml))
            .build()
            .unwrap();
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn parse_duration(&self) -> Result<Duration, Box<dyn Error>> {
        let config_value = self.config.get::<String>("temperature_checks_frequency")?;

        parse_duration::parse(&config_value).map_err(|e| Box::try_from(e).unwrap())
    }
}