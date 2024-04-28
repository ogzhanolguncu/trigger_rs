use chrono::Duration;

pub fn format_duration(duration: Duration) -> String {
    let minutes = duration.num_seconds() / 60;
    let seconds = duration.num_seconds() % 60;
    let hours = minutes / 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)

}