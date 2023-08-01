use std::env;
use std::path::PathBuf;
use std::sync::RwLock;
use std::time::Duration;

use config::Config;
use parse_duration::parse::Error;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::time;
use crate::home_info_provider::HomeInfoProvider;

use crate::weather_provider::OpenWeatherProvider;

pub mod open_weather_models;
mod weather_provider;
mod home_info_provider;
mod conditions_comparer;
mod notification_manager;

lazy_static::lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new({
        let mut settings = Config::builder()
            .add_source(config::File::with_name("Settings"))
            .build()
            .unwrap();

        settings
    });
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_logging();
    println!("{:?}", SETTINGS.read().unwrap());

    let mut apikey = String::new();
    let mut file = File::open("open-weather-api-key").await.unwrap();
    file.read_to_string(&mut apikey).await.unwrap();

    let open_weather_provider = OpenWeatherProvider {
        url: "https://api.openweathermap.org/data/2.5/weather".to_string(),
        api_key: apikey,
        location: weather_provider::Location { lat: 50.078613883148385, lon: 21.99193609164846 },
    };

    let home_info_provider = HomeInfoProvider {
        default_temp: 21f32,
    };

    loop {
        let duration = parse_duration_from_settings()?;
        println!("parsed duration: {duration:?}");
        time::sleep(duration).await;

        let weather_model = open_weather_provider.get_current().await.unwrap();
        let home_temp = home_info_provider.get_current_temp();



        // tokio::spawn(get_weather());
    }
}

fn parse_duration_from_settings() -> Result<Duration, Error> {
    parse_duration::parse(
        &SETTINGS.read()
            .unwrap()
            .get::<String>("temperature_checks_frequency").expect("Error: Unable to get duration string"))
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

