use chrono::{DateTime, Local, NaiveTime, Utc};

pub fn parse_time(s: Option<String>) -> Option<DateTime<Utc>> {
    let lower = s.unwrap_or("".to_string()).to_lowercase();
    let (time, period) = lower.split_at(lower.len() - 2);

    let time = if !time.contains(":") {
        format!("{}:00 {}", time, period)
    } else {
        lower.to_string()
    };

    let format = match period {
        "am" => "%I:%M %p",
        "pm" => "%I:%M %p",
        _ => return None,
    };

    NaiveTime::parse_from_str(&time, format)
        .map(|time| Local::now().date_naive().and_time(time).and_utc())
        .ok()
}
