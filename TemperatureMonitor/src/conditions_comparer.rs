use crate::notification_manager::{Notification};

struct AtmosphereData {
    temperature: f32,
    humidity: u32,
}

struct ConditionsComparer {
    weather: AtmosphereData,
    home: AtmosphereData,
    last_notification: String,
}

impl ConditionsComparer {
    fn check_temp_diff(&mut self) -> Option<Notification> {
        let (temp_inside, temp_outside) = (self.weather.temperature, self.home.temperature);

        if temp_outside > temp_inside {
            return Some(Notification::CloseWindow { temp_outside, temp_inside });
        }

        None
    }
}