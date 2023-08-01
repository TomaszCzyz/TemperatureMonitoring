use std::error::Error;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{AtmosphereData, WeatherProvider};

#[derive(Serialize, Deserialize)]
pub struct OpenWeatherModel {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: i32,
    pub wind: Wind,
    pub rain: Option<Rain>,
    pub clouds: Clouds,
    pub dt: i32,
    pub sys: Sys,
    pub timezone: i32,
    pub id: i32,
    pub name: String,
    pub cod: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Sys {
    #[serde(rename = "type")]
    pub r#type: i32,
    pub id: i32,
    pub country: String,
    pub sunrise: i32,
    pub sunset: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Weather {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize)]
pub struct Clouds {
    pub all: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Rain {
    #[serde(rename = "1h")]
    one_hour: Option<f32>,
    #[serde(rename = "3h")]
    three_hours: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct Wind {
    pub speed: f32,
    pub deg: i32,
    pub gust: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct Main {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: i32,
    pub humidity: u32,
    pub sea_level: Option<i32>,
    pub grnd_level: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct Coord {
    pub lon: f32,
    pub lat: f32,
}

pub struct Location {
    pub lat: f32,
    pub lon: f32,
}

pub struct OpenWeatherProvider {
    pub url: String,
    pub api_key: String,
    pub location: Location,
}

#[async_trait]
impl WeatherProvider for OpenWeatherProvider {
    async fn get_current(&self) -> Result<AtmosphereData, Box<dyn Error>> {
        let client = reqwest::Client::new();

        let url = format!("{}?lat={}&lon={}&appid={}", self.url, self.location.lat, self.location.lon, self.api_key);
        let response = client.get(url)
            .send().await?
            .json::<OpenWeatherModel>().await?;

        Ok(AtmosphereData {
            temperature: response.main.temp,
            humidity: response.main.humidity,
        })
    }
}

