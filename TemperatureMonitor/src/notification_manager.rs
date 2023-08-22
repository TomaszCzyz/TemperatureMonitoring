use std::fmt::{Display, Formatter};
use notify_rust::{Notification, Timeout};

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
    last_notification: Option<Event>,
}

impl NotificationManager {
    pub fn new() -> Self {
        Self {
            last_notification: None
        }
    }

    pub fn notify(&self, event: Event) -> Option<()> {
        Notification::new()
            .summary("Close your windows!")
            .body(&format!("{}", event))
            .icon("firefox")
            .timeout(Timeout::Milliseconds(1000))
            .show()
            .ok()
    }
}
