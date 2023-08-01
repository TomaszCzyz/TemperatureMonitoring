use crate::open_weather_models::OpenWeatherModel;

pub(crate) struct Location {
    pub(crate) lat: f32,
    pub(crate) lon: f32,
}

pub struct OpenWeatherProvider {
    pub(crate) url: String,
    pub(crate) api_key: String,
    pub(crate) location: Location,
}

impl OpenWeatherProvider {
    pub async fn get_current(&self) -> Result<OpenWeatherModel, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let url = format!("{}?lat={}&lon={}&appid={}", self.url, self.location.lat, self.location.lon, self.api_key);
        let response = client.get(url)
            .send().await?
            .json::<OpenWeatherModel>().await?;

        Ok(response)
    }
}

