use std::env;
use std::error::Error;
use std::path::PathBuf;

use async_trait::async_trait;
use tokio::io::AsyncReadExt;
use tokio::time;

use crate::conditions_comparer::ConditionsComparer;
use crate::fake_weather_provider::FakeWeatherProvider;
use crate::home_info_provider::HomeInfoProvider;
use crate::notification_manager::NotificationManager;
use crate::settings::Settings;

mod open_weather_provider;
mod home_info_provider;
mod conditions_comparer;
mod notification_manager;
mod fake_weather_provider;
mod settings;

#[async_trait]
pub trait WeatherProvider {
    async fn get_current(&self) -> Result<AtmosphereData, Box<dyn Error>>;
}

#[derive(Debug)]
pub struct AtmosphereData {
    pub temperature: f32,
    pub humidity: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    setup_logging();
    let mut settings = Settings::new("Settings".into());

    // let apikey = get_api_key().await;
    // let weather_provider = OpenWeatherProvider {
    //     url: "https://api.openweathermap.org/data/2.5/weather".to_string(),
    //     api_key: apikey,
    //     location: open_weather_provider::Location { lat: 50.078613883148385, lon: 21.99193609164846 },
    // };

    let notification_manager = NotificationManager::new();

    let weather_provider = FakeWeatherProvider;
    let home_info_provider = HomeInfoProvider {
        default_temp: 21f32,
    };

    loop {
        time::sleep(settings.parse_duration()?).await;

        let outside = weather_provider.get_current().await.unwrap();
        let inside = home_info_provider.get_current_temp().await.unwrap();
        println!("outside: {:?}\t\tinside: {:?}", &outside, &inside);

        match ConditionsComparer::compare(outside, inside) {
            None => {}
            Some(event) => {
                notification_manager.notify(event);
            }
        }

        settings.refresh();
    }
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

