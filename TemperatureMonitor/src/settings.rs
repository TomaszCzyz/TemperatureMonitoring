use std::error::Error;
use std::time::Duration;

use config::{Config, ConfigBuilder, File, FileFormat};
use config::builder::DefaultState;

pub struct Settings<'a> {
    filename: &'a str,
    config: Config,
}

impl<'a> Settings<'a> {
    pub fn new(filename: &'a str) -> Self {
        let config = Self::create_config(&filename);

        Self {
            filename,
            config,
        }
    }

    pub fn refresh(&mut self) {
        self.config = Self::create_config(self.filename);
    }

    fn create_config(filename: &str) -> Config {
        ConfigBuilder::<DefaultState>::default()
            .add_source(File::new(&filename, FileFormat::Toml))
            .build()
            .unwrap()
    }

    pub fn duration(&self) -> Result<Duration, Box<dyn Error>> {
        let config_value = self.config.get::<String>("temperature_checks_frequency")?;
        
        parse_duration::parse(&config_value).map_err(|e| Box::from(e))
    }
}