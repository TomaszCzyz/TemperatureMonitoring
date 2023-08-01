use std::error::Error;

use async_trait::async_trait;
use rand::Rng;

use crate::conditions_comparer::AtmosphereData;
use crate::WeatherProvider;

pub struct FakeWeatherProvider;

#[async_trait]
impl WeatherProvider for FakeWeatherProvider {
    async fn get_current(&self) -> Result<AtmosphereData, Box<dyn Error>> {
        let mut rng = rand::thread_rng();
        let temperature = rng.gen_range(-20.0..40.0);
        let humidity = rng.gen_range(0..100);

        Ok(AtmosphereData {
            temperature,
            humidity,
        })
    }
}

