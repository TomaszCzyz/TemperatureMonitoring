use std::fmt::{Display, Formatter};

pub enum Event {
    CloseWindow { temp_outside: f32, temp_inside: f32 },
    OpenWindow { temp_outside: f32, temp_inside: f32 },
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Event::CloseWindow { temp_inside, temp_outside }
            => format!("Temperature outside({temp_outside}) is higher than inside ({temp_inside})... close windows! "),
            Event::OpenWindow { temp_inside, temp_outside }
            => format!("Temperature outside({temp_outside}) is lower than inside ({temp_inside})... open windows! "),
        };
        write!(f, "{}", message)
    }
}

pub struct NotificationManager {
    last_notification: Event,
}

impl NotificationManager {
    fn notify(event: Event) {
        println!("{}", event)
    }
}

impl NotificationManager {}
