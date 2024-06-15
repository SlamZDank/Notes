use std::time::{SystemTime, UNIX_EPOCH};



pub fn now_date() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn convert_unix_to_custom_date() {

}