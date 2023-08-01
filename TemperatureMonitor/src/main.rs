use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;

use async_trait::async_trait;
use config::{Config, ConfigBuilder, File, FileFormat};
use config::builder::DefaultState;
use tokio::io::AsyncReadExt;
use tokio::time;

use crate::conditions_comparer::AtmosphereData;
use crate::fake_weather_provider::FakeWeatherProvider;
use crate::home_info_provider::HomeInfoProvider;

mod open_weather_provider;
mod home_info_provider;
mod conditions_comparer;
mod notification_manager;
mod fake_weather_provider;

fn get_config() -> Config {
    ConfigBuilder::<DefaultState>::default()
        .add_source(File::new("Settings", FileFormat::Toml))
        .build()
        .unwrap()
}

#[async_trait]
pub trait WeatherProvider {
    async fn get_current(&self) -> Result<AtmosphereData, Box<dyn Error>>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    setup_logging();

    // let apikey = get_api_key().await;
    // let weather_provider = OpenWeatherProvider {
    //     url: "https://api.openweathermap.org/data/2.5/weather".to_string(),
    //     api_key: apikey,
    //     location: open_weather_provider::Location { lat: 50.078613883148385, lon: 21.99193609164846 },
    // };

    let weather_provider = FakeWeatherProvider;
    let home_info_provider = HomeInfoProvider {
        default_temp: 21f32,
    };

    loop {
        let config = get_config();
        println!("{:?}", config);

        let duration = parse_duration(config)?;
        println!("parsed duration: {duration:?}");
        time::sleep(duration).await;

        let outside = weather_provider.get_current().await.unwrap();
        let inside = home_info_provider.get_current_temp().await.unwrap();

        println!("{:?}\n{:?}", outside, inside);
    }
}

fn parse_duration(config: Config) -> Result<Duration, Box<dyn Error>> {
    let config_value = &config.get::<String>("temperature_checks_frequency")?;

    parse_duration::parse(config_value).map_err(|e| Box::try_from(e).unwrap())
}

async fn get_api_key() -> String {
    let mut apikey = String::new();
    let mut file = tokio::fs::File::open("open-weather-api-key").await.unwrap();
    file.read_to_string(&mut apikey).await.unwrap();
    apikey
}

fn setup_logging() {
    let path = get_log_dir();
    let file_appender = tracing_appender::rolling::daily(path, "prefix.log");

    let (non_blocking_file, _guard) = tracing_appender::non_blocking(file_appender);
    let (non_blocking_stdout, _guard2) = tracing_appender::non_blocking(std::io::stdout());

    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_writer(non_blocking_file)
        .with_writer(non_blocking_stdout)
        .init();
}

fn get_log_dir() -> PathBuf {
    let mut path_buf = env::current_dir().unwrap();
    path_buf.push("logs");
    path_buf
}

