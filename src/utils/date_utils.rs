use chrono::{DateTime, FixedOffset, ParseError, Utc};

pub fn now_utc_with_timezone() -> DateTime<FixedOffset>{
    let utc_minus_3: FixedOffset = FixedOffset::west_opt(3 * 3600).unwrap();

    let now_utc: DateTime<Utc> = Utc::now();

    now_utc.with_timezone(&utc_minus_3)
}

pub fn parse_str_to_date(date: &str) -> Result<DateTime<FixedOffset>, ParseError> {
    DateTime::parse_from_str(date, "%d-%m-%Y %H:%M:%S %z")
}