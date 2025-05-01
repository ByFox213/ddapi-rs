pub fn seconds_to_hours<T: Into<u64>>(seconds: T) -> f64 {
    let seconds_f64: f64 = seconds.into() as f64;
    (seconds_f64 / 3600.0 * 100.0).round() / 100.0
}
