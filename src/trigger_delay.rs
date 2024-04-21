use std::time::Duration;

#[derive(PartialEq, Debug)]
pub enum TriggerTime {
    Minute(u64),
    Second(u64),
    Hour(u64),
    Day(u64),
}

impl TriggerTime {
    fn to_duration(&self) -> Duration {
        match self {
            TriggerTime::Minute(mins) => Duration::from_secs(mins * 60),
            TriggerTime::Second(secs) => Duration::from_secs(*secs),
            TriggerTime::Hour(hours) => Duration::from_secs(hours * 3600),
            TriggerTime::Day(days) => Duration::from_secs(days * 86400),
        }
    }

    pub fn from_string(s: Option<String>) -> Option<Duration> {
        if let Some(delay) = s {
            let len = delay.len();
            let (num, unit) = delay.split_at(len - 1);
            let number = num.parse::<u64>().ok()?;

            return match unit {
                "m" => Some(TriggerTime::Minute(number).to_duration()),
                "s" => Some(TriggerTime::Second(number).to_duration()),
                "h" => Some(TriggerTime::Hour(number).to_duration()),
                "d" => Some(TriggerTime::Day(number).to_duration()),
                _ => None,
            };
        }
        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_return_trigger_delay() {
        let five_min = Some("5m".to_string());

        assert_eq!(
            TriggerTime::from_string(five_min).unwrap(),
            TriggerTime::Second(5 * 60).to_duration()
        )
    }
}
