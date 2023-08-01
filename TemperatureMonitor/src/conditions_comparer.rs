use crate::notification_manager::{Event};

#[derive(Debug)]
pub struct AtmosphereData {
    pub temperature: f32,
    pub humidity: u32,
}

struct ConditionsComparer {
    weather: AtmosphereData,
    home: AtmosphereData,
    last_notification: String,
}

impl ConditionsComparer {
    fn check_temp_diff(&mut self) -> Option<Event> {
        let (temp_inside, temp_outside) = (self.weather.temperature, self.home.temperature);

        if temp_outside > temp_inside {
            return Some(Event::CloseWindow { temp_outside, temp_inside });
        }

        None
    }
}