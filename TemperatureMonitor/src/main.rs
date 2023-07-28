use std::env;
use std::path::PathBuf;
use std::time::Duration;
use tokio::fs::File;

use tokio::io::AsyncReadExt;
use tokio::time;
use tracing::subscriber::SetGlobalDefaultError;

use crate::models::OpenWeatherModel;

pub mod models;

#[tokio::main]
async fn main() -> Result<(), SetGlobalDefaultError> {
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

    let fact = get_weather().await;
    let model = fact.unwrap();
    let json = serde_json::to_string_pretty(&model).unwrap();

    tracing::warn! {
            %json,
            "failed to parse command from frame"
        }

    let delay_duration = Duration::from_secs(3);

    println!("Task started...");
    time::sleep(delay_duration).await;
    println!("Task completed after the delay!");
    println!("fact = {:#?}", json);

    Ok(())
}

fn get_log_dir() -> PathBuf {
    let mut path_buf = env::current_dir().unwrap();
    path_buf.push("logs");
    path_buf
}

async fn get_weather() -> Result<OpenWeatherModel, Box<dyn std::error::Error>> {
    let mut file = File::open("open-weather-api-key").await?;
    let mut apikey = String::new();
    file.read_to_string(&mut apikey).await?;


    let client = reqwest::Client::new();
    let url = format!("https://api.openweathermap.org/data/2.5/weather?lat=50.078613883148385&lon=21.99193609164846&appid={apikey}&units");
    let body = client.get(url).send()
        .await?
        .json::<OpenWeatherModel>()
        .await?;

    Ok(body)
}