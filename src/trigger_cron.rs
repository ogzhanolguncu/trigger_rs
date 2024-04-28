use std::str::FromStr;

use anyhow::{bail, Context, Result};
use chrono::{Duration, Utc};
use cron::Schedule;

use crate::errors::CronError;

pub fn calculate_next_trigger_time_cron(cron: String) -> Result<Duration> {
    let schedule = Schedule::from_str(&cron)?;
    let now = Utc::now();

    if let Some(next) = schedule.upcoming(Utc).take(1).next() {
        let time_until_next = next - now;

        Ok(Duration::milliseconds(time_until_next.num_milliseconds()))
    } else {
        bail!(CronError::NoUpcomingTrigger)
    }
}

pub fn check_validity_of_cron(cron: &str) -> Result<()> {
    Schedule::from_str(cron).with_context(|| CronError::InvalidCron(cron.to_string()))?;

    Ok(())
}
