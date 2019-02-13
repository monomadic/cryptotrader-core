#![allow(dead_code)]
#![allow(unused_variables)]

use chrono::offset::Local;
use chrono::prelude::DateTime;
use std::time::{Duration, UNIX_EPOCH};

/// Converts a unix timestamp to a rust DateTime.
pub fn local_datetime_from_unix(time: u64) -> DateTime<Local> {
    DateTime::<Local>::from(UNIX_EPOCH + Duration::from_millis(time))
}

/// Expresses the difference as a percentage between two floats.
///
/// ```rust
/// use cryptotrader::presenters::price_percent;
/// assert_eq!(price_percent(5.0, 10.0), 100.0);
/// assert_eq!(price_percent(100.0, 50.0), -50.0);
/// ```
pub fn price_percent(entry_price: f64, exit_price: f64) -> f64 {
    if entry_price < exit_price {
        (100. / entry_price * exit_price) - 100.
    } else {
        -(100. + -100. / entry_price * exit_price)
    }
}

/// Finds the average (mean) of a series of f64 floats.
///
/// ```rust
/// assert_eq!(average(5.0, 10.0), 7.5);
/// assert_eq!(average(50.0, 50.0, -100.0), 0.0);
/// ```
pub fn average(numbers: &Vec<f64>) -> f64 {
    let sum: f64 = numbers.iter().sum();
    sum / numbers.len() as f64
}
