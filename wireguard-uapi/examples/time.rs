use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let timestamp_secs = timestamp.as_secs();
    println!("Timestamp (seconds): {}", timestamp_secs);
}
