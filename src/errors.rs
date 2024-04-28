use thiserror::Error;

#[derive(Error, Debug)]
pub enum CronError {
    #[error("Invalid cron expression: {0}")]
    InvalidCron(String),

    #[error("No upcoming triggers found")]
    NoUpcomingTrigger,
}
