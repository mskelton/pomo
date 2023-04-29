use chrono::Duration;

pub fn parse_duration(str: String) -> Option<Duration> {
    return match humantime::parse_duration(&str) {
        Ok(d) => Some(Duration::from_std(d).unwrap()),
        Err(e) => match humantime::parse_duration(&fallback) {},
    };
}
