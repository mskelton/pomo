use chrono::{DateTime, Local, NaiveTime};

pub fn parse_time(s: Option<String>) -> Option<DateTime<Local>> {
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

    NaiveTime::parse_from_str(&time, format).ok().and_then(|time| {
        let l = Local::now();
        l.date_naive().and_time(time).and_local_timezone(l.timezone()).single()
    })
}
