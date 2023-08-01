use std::fmt::{Display, Formatter};

pub enum Notification {
    CloseWindow { temp_outside: f32, temp_inside: f32 },
    OpenWindow { temp_outside: f32, temp_inside: f32 },
}

impl Display for Notification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Notification::CloseWindow { temp_inside, temp_outside }
            => format!("Temperature outside({temp_outside}) is higher than inside ({temp_inside})... close windows! "),
            Notification::OpenWindow { temp_inside, temp_outside }
            => format!("Temperature outside({temp_outside}) is lower than inside ({temp_inside})... open windows! "),
        };
        write!(f, "{}", message)
    }
}

pub struct NotificationManager {
    last_notification: Notification,
}

impl NotificationManager {
    fn notify(event: Notification) {
        println!("{}", event)
    }
}

impl NotificationManager {}
