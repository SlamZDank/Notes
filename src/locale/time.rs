use chrono::DateTime;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn now_date() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 3600
    // add an hour to conform to our time since its a personal project
}

pub fn convert_unix_to_custom_date(time_in_unix: u64) -> (String, String) {
    let date_time = DateTime::from_timestamp(time_in_unix as i64, 0)
        .expect("Unix timestamp should be a 64 bit integer");
    (
        format!("{}", date_time.format("%d/%m/%Y")),
        format!("{}", date_time.format("%H:%M:%S")),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        println!(
            "{} , {}",
            convert_unix_to_custom_date(1718468171).0,
            convert_unix_to_custom_date(1718468171).1
        );
    }
}
