use chrono::Duration;

use axum::http::HeaderMap;
use serde_json::Value;

pub fn format_duration(duration: Duration) -> String {
    let minutes = duration.num_seconds() / 60;
    let seconds = duration.num_seconds() % 60;
    let hours = minutes / 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

pub fn convert(headers: &HeaderMap) -> serde_json::Value {
    format!("{:?}", headers).into()
}
