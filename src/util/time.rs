/// Converts seconds to hours with 2 decimal places precision
///
/// # Examples
/// ```
/// use ddapi_rs::util::time::seconds_to_hours;
/// 
/// assert_eq!(seconds_to_hours(3600), 1.0);  // 1 hour
/// assert_eq!(seconds_to_hours(5400), 1.5);  // 1.5 hours
/// assert_eq!(seconds_to_hours(3672), 1.02); // 1.02 hours
/// ```
pub fn seconds_to_hours<T: Into<f64>>(seconds: T) -> f64 {
    seconds_to_hours_precision(seconds, 2)
}

/// Converts seconds to hours with specified decimal places precision
///
/// # Examples
/// ```
/// use ddapi_rs::util::time::seconds_to_hours_precision;
/// 
/// assert_eq!(seconds_to_hours_precision(3661, 2), 1.02);  // 2 decimal places
/// assert_eq!(seconds_to_hours_precision(3661, 3), 1.017); // 3 decimal places
/// ```
pub fn seconds_to_hours_precision<T: Into<f64>>(seconds: T, decimal_places: u32) -> f64 {
    let factor = 10_f64.powi(decimal_places as i32);
    let hours = seconds.into() / 3600.0;
    (hours * factor).round() / factor
}