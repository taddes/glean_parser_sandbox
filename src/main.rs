pub mod glean {
    pub mod server_events;
}

pub mod user_agents {
    pub mod user_agent;
}

use crate::glean::server_events::{
    BackendObjectUpdateEvent, EventsPing, GleanEventsLogger, RequestInfo,
};
use std::thread::sleep;
use std::time::Duration;
fn main() {
    let headers: Vec<&str> = vec![
    "Firefox/130.0.1 (Windows NT 10.0; Win64; x64) FxSync/1.132.0.20240913135723.desktop",
    "Firefox/130.0.1 (Linux x86_64) FxSync/1.132.0.20240916093609.desktop",
    "Firefox-iOS-FxA/24",
    "Mozilla/5.0 (Linux; Android 9; SM-A920F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/86.0.4216.0 Mobile Safari/537.36",
    "Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.1; Trident/4.0)",
    "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/115.0",
    "Mozilla/5.0 (Linux; Android 9; SM-A920F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/86.0.4216.0 Mobile Safari/537.36",
    "Floorp/128.3.0 (Intel Mac OS X 10.15) FxSync/1.130.0.20240908033729.desktop",
    "Mozilla/5.0 (Android 13; Mobile; rv:130.0) Gecko/130.0 Firefox/130.0",
    "Firefox-iOS-Sync/108.1b24234 (iPad; iPhone OS 16.4.1) (Firefox)",
    ];

    let logger = GleanEventsLogger {
        app_id: "test-rust-logger".to_string(),
        app_display_version: "1.0.0".to_string(),
        app_channel: "development".to_string(),
    };
    // Simulate events in a loop
    loop {
        logger.record_events_ping(
            &RequestInfo {
                user_agent: "Mozilla/5.0".to_string(),
                ip_address: "192.168.1.1".to_string(),
            },
            &EventsPing {
                identifiers_fxa_account_id: "1234".to_string(),
                syncstorage_device_family: "desktop".to_string(),
                syncstorage_hashed_device_id: "f307ee".to_string(),
                syncstorage_hashed_fxa_uid: "e330a3".to_string(),
                syncstorage_platform: "MacOS".to_string(),
                event: Some(Box::new(BackendObjectUpdateEvent {
                    object_type: "your_object_type".to_string(),
                    object_state: "your_object_state".to_string(),
                    linking: true,
                })),
            },
        );

        sleep(Duration::from_secs(2)); // Adjust the duration as needed
    }
}
