use crate::AtmosphereData;
use crate::notification_manager::Event;

pub struct ConditionsComparer;

impl ConditionsComparer {
    pub fn compare(outside: AtmosphereData, inside: AtmosphereData) -> Option<Event> {
        let (temp_inside, temp_outside) = (outside.temperature, inside.temperature);

        if temp_outside > temp_inside {
            return Some(Event::CloseWindow { temp_outside, temp_inside });
        }

        None
    }
}