use std::{error::Error, str::FromStr};

use chrono::{DateTime, Duration, Local, Utc};
use cron::Schedule;

pub fn calculate_next_trigger_time_cron(cron: &str) -> Result<Duration, Box<dyn Error>> {
    let schedule = Schedule::from_str(cron)?;
    let now = Utc::now();

    if let Some(next) = schedule.upcoming(Utc).take(1).next() {
        let time_until_next = next - now;

        Ok(Duration::milliseconds(time_until_next.num_milliseconds()))
    } else {
        Err("No upcoming trigger found".into())
    }
}

pub fn check_validity_of_cron(cron: &str) -> Result<(), Box<dyn Error>> {
	Schedule::from_str(cron)?;

	Ok(())
}